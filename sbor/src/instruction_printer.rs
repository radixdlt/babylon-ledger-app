use crate::bech32::encoder::*;
use crate::bech32::hrp::*;
use crate::bech32::network::*;
use crate::display_io::DisplayIO;
use crate::instruction::{InstructionInfo, ParameterType};
use crate::instruction_extractor::{ExtractorEvent, InstructionHandler};
use crate::math::Decimal;
use crate::sbor_decoder::SborEvent;
use crate::type_info::{ADDRESS_LEN, TYPE_ENUM, TYPE_STRING};
use arrform::{arrform, ArrForm};
use core::str::from_utf8;

pub struct InstructionPrinter {
    active_instruction: Option<InstructionInfo>,
    instruction_printer: Option<&'static dyn ParameterPrinter>,
    state: ParameterPrinterState,
    display: &'static dyn DisplayIO,
}

struct ParameterPrinterState {
    data: [u8; Self::PARAMETER_AREA_SIZE],
    data_counter: u8,
    miscellaneous: u8,
    flip_flop: bool,
    network_id: NetworkId,
    resource_id: HrpType,
    phase: u8,
    expected_len: u32,
}

impl InstructionHandler for InstructionPrinter {
    fn handle(&mut self, event: ExtractorEvent) {
        match event {
            ExtractorEvent::InstructionStart(info) => self.start_instruction(info),
            ExtractorEvent::ParameterStart(_type_kind, ordinal) => self.parameter_start(ordinal),
            ExtractorEvent::ParameterData(data) => self.parameter_data(data),
            ExtractorEvent::ParameterEnd(_) => self.parameter_end(),
            ExtractorEvent::InstructionEnd => self.instruction_end(),
            // TODO: decide what to do with these cases
            ExtractorEvent::WrongParameterCount(_, _) => {}
            ExtractorEvent::UnknownInstruction(_) => {}
            ExtractorEvent::InvalidEventSequence => {}
            ExtractorEvent::UnknownParameterType(_) => {}
        }
    }
}

impl InstructionPrinter {
    pub fn new(display: &'static dyn DisplayIO, network_id: NetworkId) -> Self {
        Self {
            active_instruction: None,
            instruction_printer: None,
            state: ParameterPrinterState::new(network_id),
            display: display,
        }
    }

    pub fn set_network(&mut self, network_id: NetworkId) {
        self.state.set_network(network_id);
    }

    pub fn start_instruction(&mut self, info: InstructionInfo) {
        self.active_instruction = Some(info);
        self.display.scroll(info.name);
    }

    pub fn instruction_end(&mut self) {
        if let Some(..) = self.active_instruction {
            //TODO: replace with something usable in device
            self.display.scroll("End\n".as_bytes());
        }

        self.active_instruction = None;
        self.instruction_printer = None;
    }

    pub fn parameter_start(&mut self, ordinal: u32) {
        self.state.reset();
        self.instruction_printer = self
            .active_instruction
            .filter(|info| (info.params.len() as u32) > ordinal)
            .map(|info| info.params[ordinal as usize])
            .map(|param_type| get_printer_for_type(param_type));
    }

    pub fn parameter_data(&mut self, source_event: SborEvent) {
        self.get_printer()
            .handle_data_event(&mut self.state, source_event, self.display);
    }

    pub fn parameter_end(&mut self) {
        self.get_printer().display(&self.state, self.display);
        self.state.reset();
    }

    fn get_printer(&self) -> &'static dyn ParameterPrinter {
        self.instruction_printer
            .unwrap_or(&IGNORED_PARAMETER_PRINTER)
    }
}

impl ParameterPrinterState {
    const PARAMETER_AREA_SIZE: usize = 128;

    pub fn new(network_id: NetworkId) -> Self {
        Self {
            data: [0; Self::PARAMETER_AREA_SIZE],
            data_counter: 0,
            miscellaneous: 0,
            flip_flop: false,
            network_id,
            resource_id: HrpType::Autodetect,
            phase: 0,
            expected_len: 0,
        }
    }

    pub fn set_network(&mut self, network_id: NetworkId) {
        self.network_id = network_id;
    }

    pub fn reset(&mut self) {
        self.data = [0; Self::PARAMETER_AREA_SIZE];
        self.data_counter = 0;
        self.miscellaneous = 0;
        self.flip_flop = false;
        self.resource_id = HrpType::Autodetect;
        self.phase = 0;
        self.expected_len = 0;
    }

    pub fn data(&self) -> &[u8] {
        &self.data[0..self.data_counter as usize]
    }

    pub fn push_byte(&mut self, byte: u8) {
        if (self.data_counter as usize) < Self::PARAMETER_AREA_SIZE {
            self.data[self.data_counter as usize] = byte;
            self.data_counter += 1;
        }
    }

    pub fn push_byte_for_string(&mut self, byte: u8) {
        self.push_byte(byte);

        // Add '...' at the end of truncated string.
        if self.data_counter as usize == ParameterPrinterState::PARAMETER_AREA_SIZE - 2 {
            self.data_counter -= 1; // Override last characted
            self.push_byte(b'.');
            self.push_byte(b'.');
            self.push_byte(b'.');
        }
    }
}

trait ParameterPrinter {
    fn handle_data_event(
        &self,
        state: &mut ParameterPrinterState,
        event: SborEvent,
        display: &'static dyn DisplayIO,
    );
    fn display(&self, state: &ParameterPrinterState, display: &'static dyn DisplayIO); // TODO: can we break flow in the mid of instruction?
}

fn get_printer_for_type(param_type: ParameterType) -> &'static dyn ParameterPrinter {
    match param_type {
        ParameterType::Ignored => &IGNORED_PARAMETER_PRINTER,
        ParameterType::AccessRule => &ACCESS_RULE_PARAMETER_PRINTER,
        ParameterType::AccessRulesConfig => &IGNORED_PARAMETER_PRINTER,
        ParameterType::MethodKey => &METHOD_KEY_PARAMETER_PRINTER,
        ParameterType::BTreeMapByStringToRoyaltyConfig => &IGNORED_PARAMETER_PRINTER,
        ParameterType::BTreeMapByStringToString => &IGNORED_PARAMETER_PRINTER,
        ParameterType::BTreeSetOfNonFungibleLocalId => &IGNORED_PARAMETER_PRINTER,
        ParameterType::ComponentAddress => &COMPONENT_ADDRESS_PARAMETER_PRINTER,
        ParameterType::Decimal => &DECIMAL_PARAMETER_PRINTER,
        ParameterType::ManifestAddress => &MANIFEST_ADDRESS_PARAMETER_PRINTER,
        ParameterType::ManifestBlobRef => &MANIFEST_BLOB_REF_PARAMETER_PRINTER,
        ParameterType::ManifestBucket => &U32_PARAMETER_PRINTER,
        ParameterType::ManifestProof => &U32_PARAMETER_PRINTER,
        ParameterType::ManifestValue => &IGNORED_PARAMETER_PRINTER, // use discriminator to select correct printer
        ParameterType::PackageAddress => &PACKAGE_ADDRESS_PARAMETER_PRINTER,
        ParameterType::ResourceAddress => &RESOURCE_ADDRESS_PARAMETER_PRINTER,
        ParameterType::RoyaltyConfig => &IGNORED_PARAMETER_PRINTER,
        ParameterType::String => &STRING_PARAMETER_PRINTER,
        ParameterType::ObjectId => &OBJECT_ID_PARAMETER_PRINTER,
        ParameterType::U8 => &U8_PARAMETER_PRINTER,
    }
}

// Parameter which we just skip, without printing anything
struct IgnoredParameter {}

const IGNORED_PARAMETER_PRINTER: IgnoredParameter = IgnoredParameter {};

impl ParameterPrinter for IgnoredParameter {
    fn handle_data_event(
        &self,
        _state: &mut ParameterPrinterState,
        _event: SborEvent,
        _display: &'static dyn DisplayIO,
    ) {
    }
    fn display(&self, _state: &ParameterPrinterState, display: &'static dyn DisplayIO) {
        display.scroll("<not decoded>".as_bytes())
    }
}

// U8 parameter printer
struct U8ParameterPrinter {}

const U8_PARAMETER_PRINTER: U8ParameterPrinter = U8ParameterPrinter {};

impl ParameterPrinter for U8ParameterPrinter {
    fn handle_data_event(
        &self,
        state: &mut ParameterPrinterState,
        event: SborEvent,
        _display: &'static dyn DisplayIO,
    ) {
        if let SborEvent::Data(byte) = event {
            state.data[0] = byte;
            state.data_counter = 1;
        }
    }

    fn display(&self, state: &ParameterPrinterState, display: &'static dyn DisplayIO) {
        if state.data_counter != 1 {
            //TODO: an error condition, should we handle it somehow?
            return;
        }

        display.scroll(arrform!(8, "{}u8", state.data[0]).as_bytes());
    }
}

// U32 parameter printer
struct U32ParameterPrinter {}

const U32_PARAMETER_PRINTER: U32ParameterPrinter = U32ParameterPrinter {};

impl ParameterPrinter for U32ParameterPrinter {
    fn handle_data_event(
        &self,
        state: &mut ParameterPrinterState,
        event: SborEvent,
        display: &'static dyn DisplayIO,
    ) {
        if let SborEvent::Data(byte) = event {
            state.data[state.data_counter as usize] = byte;
            state.data_counter += 1;

            if state.data_counter > 4 {
                display.scroll(b"<Invalid parameter size>");
            }
        }
    }

    fn display(&self, state: &ParameterPrinterState, display: &'static dyn DisplayIO) {
        if state.data_counter != 4 {
            return;
        }

        fn to_array(input: &[u8]) -> [u8; 4] {
            input.try_into().expect("<should not happen>")
        }

        let value = u32::from_le_bytes(to_array(&state.data[..(state.data_counter as usize)]));

        display.scroll(arrform!(20, "{}u32", value).as_bytes());
    }
}

// String parameter printer
struct StringParameterPrinter {}

const STRING_PARAMETER_PRINTER: StringParameterPrinter = StringParameterPrinter {};

impl ParameterPrinter for StringParameterPrinter {
    fn handle_data_event(
        &self,
        state: &mut ParameterPrinterState,
        event: SborEvent,
        _display: &'static dyn DisplayIO,
    ) {
        if let SborEvent::Data(byte) = event {
            //TODO: split longer strings into chunks; keep in mind utf8 boundaries
            state.push_byte_for_string(byte);
        }
    }

    fn display(&self, state: &ParameterPrinterState, display: &'static dyn DisplayIO) {
        match from_utf8(&state.data[..(state.data_counter as usize)]) {
            Ok(message) => display.scroll(message.as_bytes()),
            Err(_) => display.scroll(b"<String decoding error>"),
        }
    }
}

// Printer for various parameters formatted as hex string
struct HexParameterPrinter {
    fixed_len: u32,
}

const OBJECT_ID_LEN: u32 = 1 + 26 + 4; // ENTITY_BYTES_LENGTH + OBJECT_HASH_LENGTH + OBJECT_INDEX_LENGTH
const OBJECT_ID_PARAMETER_PRINTER: HexParameterPrinter = HexParameterPrinter {
    fixed_len: OBJECT_ID_LEN,
};
const MANIFEST_BLOB_REF_PARAMETER_PRINTER: HexParameterPrinter =
    HexParameterPrinter { fixed_len: 32 };

impl HexParameterPrinter {
    const USER_INFO_SPACE_LEN: usize = 20; // "###/###" - show part of part
    const PRINTABLE_SIZE: usize =
        ParameterPrinterState::PARAMETER_AREA_SIZE - HexParameterPrinter::USER_INFO_SPACE_LEN;
}

impl ParameterPrinter for HexParameterPrinter {
    fn handle_data_event(
        &self,
        state: &mut ParameterPrinterState,
        event: SborEvent,
        display: &'static dyn DisplayIO,
    ) {
        // TODO: show to user that this is 'piece # of ##'
        if let SborEvent::Len(len) = event {
            if self.fixed_len > 0 && self.fixed_len != len {
                display.scroll(b"<payload size mismatch>");
                state.flip_flop = true;
            }
            state.expected_len = len;
        }

        // If error is triggered, ignore remaining data
        if state.flip_flop {
            return;
        }

        if let SborEvent::Data(byte) = event {
            if state.data_counter as usize == Self::PRINTABLE_SIZE {
                self.display(state, display);
                state.data_counter = 0;
                return;
            }

            state.push_byte(byte);
        }
    }

    fn display(&self, state: &ParameterPrinterState, display: &'static dyn DisplayIO) {
        if state.flip_flop {
            return;
        }

        let mut hex = [0u8; Self::PRINTABLE_SIZE * 2];

        let mut i = 0;
        for &c in state.data[..(state.data_counter as usize)].iter() {
            let c0 = char::from_digit((c >> 4).into(), 16).unwrap();
            let c1 = char::from_digit((c & 0xf).into(), 16).unwrap();
            hex[i] = c0 as u8;
            hex[i + 1] = c1 as u8;
            i += 2;
        }

        let len = (state.data_counter as usize) * 2;
        let message = from_utf8(&hex[..len]).unwrap();
        display.scroll(message.as_bytes());
    }
}

// Address printers for ResourceAddress/ComponentAddress/PackageAddress/ManifestAddress
struct AddressParameterPrinter {
    resource_id: HrpType,
}

const RESOURCE_ADDRESS_PARAMETER_PRINTER: AddressParameterPrinter = AddressParameterPrinter {
    resource_id: HrpType::Resource,
};
const COMPONENT_ADDRESS_PARAMETER_PRINTER: AddressParameterPrinter = AddressParameterPrinter {
    resource_id: HrpType::Component,
};
const PACKAGE_ADDRESS_PARAMETER_PRINTER: AddressParameterPrinter = AddressParameterPrinter {
    resource_id: HrpType::Package,
};
const MANIFEST_ADDRESS_PARAMETER_PRINTER: AddressParameterPrinter = AddressParameterPrinter {
    resource_id: HrpType::Autodetect,
};

impl ParameterPrinter for AddressParameterPrinter {
    fn handle_data_event(
        &self,
        state: &mut ParameterPrinterState,
        event: SborEvent,
        _display: &'static dyn DisplayIO,
    ) {
        if let SborEvent::Data(byte) = event {
            if state.flip_flop == false {
                state.flip_flop = true;
                if self.resource_id == HrpType::Autodetect {
                    // See ManifestAddress enum in radixdlt-scrypto
                    state.resource_id = match byte {
                        0x00 => HrpType::Resource,
                        0x01 => HrpType::Package,
                        0x02..=0x0C => HrpType::Component,
                        _ => HrpType::Autodetect,
                    };
                } else {
                    state.resource_id = self.resource_id;
                }
            }

            state.push_byte(byte);
        }
    }

    fn display(&self, state: &ParameterPrinterState, display: &'static dyn DisplayIO) {
        if state.data_counter != ADDRESS_LEN {
            display.scroll(b"Invalid address format");
            return;
        }

        match hrp_prefix(state.resource_id, state.data[0]) {
            None => {
                display.scroll(b"Unknown address type");
                return;
            }
            Some(hrp_prefix) => self.format_address(&state, display, hrp_prefix),
        }
    }
}

impl AddressParameterPrinter {
    fn format_address(
        &self,
        state: &ParameterPrinterState,
        display: &dyn DisplayIO,
        hrp_prefix: &str,
    ) {
        let encodind_result = Bech32::encode(
            arrform!(
                { Bech32::HRP_MAX_LEN },
                "{}{}",
                hrp_prefix,
                hrp_suffix(state.network_id)
            )
            .as_bytes(),
            &state.data[1..(state.data_counter as usize)],
        );
        match encodind_result {
            Ok(encoder) => display.scroll(encoder.encoded()),
            Err(err) => {
                display.scroll(
                    arrform!(
                        { Bech32::HRP_MAX_LEN + 250 },
                        "Error decoding {:?}({}) address {:?}: >>{:?}<<",
                        state.resource_id,
                        state.data[0],
                        err,
                        &state.data[..(state.data_counter as usize)]
                    )
                    .as_bytes(),
                );
            }
        }
    }
}

// AccessRule parameter printer
struct AccessRuleParameterPrinter {}

const ACCESS_RULE_PARAMETER_PRINTER: AccessRuleParameterPrinter = AccessRuleParameterPrinter {};

impl ParameterPrinter for AccessRuleParameterPrinter {
    fn handle_data_event(
        &self,
        state: &mut ParameterPrinterState,
        event: SborEvent,
        _display: &'static dyn DisplayIO,
    ) {
        if let SborEvent::Discriminator(byte) = event {
            if state.data_counter > 0 {
                return;
            }

            state.push_byte(byte);
        }
    }

    fn display(&self, state: &ParameterPrinterState, display: &'static dyn DisplayIO) {
        let message: &[u8] = match (state.data_counter, state.data[0]) {
            (1, 0) => b"AllowAll",
            (1, 1) => b"DenyAll",
            (1, 2) => b"Protected(<rules not decoded>)",
            (1, _) => b"<unknown access rule>",
            (_, _) => b"<invalid encoding>",
        };

        display.scroll(message);
    }
}

// MethodKey parameter printer
struct MethodKeyParameterPrinter {}

const METHOD_KEY_PARAMETER_PRINTER: MethodKeyParameterPrinter = MethodKeyParameterPrinter {};

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
enum MethodKeyPhase {
    Init,
    ModuleIdDiscrimitor,
    Ident,
}

impl From<u8> for MethodKeyPhase {
    fn from(ins: u8) -> MethodKeyPhase {
        match ins {
            0 => MethodKeyPhase::Init,
            1 => MethodKeyPhase::ModuleIdDiscrimitor,
            2 => MethodKeyPhase::Ident,
            _ => MethodKeyPhase::Init,
        }
    }
}

impl From<MethodKeyPhase> for u8 {
    fn from(ins: MethodKeyPhase) -> u8 {
        match ins {
            MethodKeyPhase::Init => 0,
            MethodKeyPhase::ModuleIdDiscrimitor => 1,
            MethodKeyPhase::Ident => 2,
        }
    }
}

fn module_id_to_name(byte: u8) -> &'static str {
    match byte {
        0 => "SELF",
        1 => "TypeInfo",
        2 => "Metadata",
        3 => "AccessRules",
        4 => "AccessRules1",
        5 => "ComponentRoyalty",
        6 => "PackageRoyalty",
        7 => "FunctionAccessRules",
        _ => "<Unknown>",
    }
}

impl ParameterPrinter for MethodKeyParameterPrinter {
    fn handle_data_event(
        &self,
        state: &mut ParameterPrinterState,
        event: SborEvent,
        _display: &'static dyn DisplayIO,
    ) {
        let phase: MethodKeyPhase = state.phase.into();

        match phase {
            MethodKeyPhase::Init => {
                if let SborEvent::Start {
                    type_id: TYPE_ENUM, ..
                } = event
                {
                    state.phase = MethodKeyPhase::ModuleIdDiscrimitor.into();
                }
            }
            MethodKeyPhase::ModuleIdDiscrimitor => {
                if let SborEvent::Discriminator(byte) = event {
                    state.miscellaneous = byte;
                }
                if let SborEvent::Start {
                    type_id: TYPE_STRING,
                    ..
                } = event
                {
                    state.phase = MethodKeyPhase::Ident.into();
                }
            }
            MethodKeyPhase::Ident => {
                if let SborEvent::Data(byte) = event {
                    state.push_byte_for_string(byte);
                }
            }
        };
    }

    fn display(&self, state: &ParameterPrinterState, display: &'static dyn DisplayIO) {
        let text = match from_utf8(&state.data[0..(state.data_counter as usize)]) {
            Ok(text) => text,
            Err(_) => "<invalid string>",
        };

        let message = arrform!(
            { ParameterPrinterState::PARAMETER_AREA_SIZE + 32 },
            "({} {})",
            module_id_to_name(state.miscellaneous),
            text
        );

        display.scroll(message.as_bytes());
    }
}

// Decimal parameter printer
// TODO: at present only positive values are printed properly
struct DecimalParameterPrinter {}

const DECIMAL_PARAMETER_PRINTER: DecimalParameterPrinter = DecimalParameterPrinter {};

impl ParameterPrinter for DecimalParameterPrinter {
    fn handle_data_event(
        &self,
        state: &mut ParameterPrinterState,
        event: SborEvent,
        _display: &'static dyn DisplayIO,
    ) {
        if let SborEvent::Data(byte) = event {
            state.push_byte(byte);
        }
    }

    fn display(&self, state: &ParameterPrinterState, display: &'static dyn DisplayIO) {
        match Decimal::try_from(state.data()) {
            Ok(value) => display.scroll(arrform!(80, "{}", value).as_bytes()),
            Err(_) => display.scroll(b"<invalid decimal value>"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bech32::network::NetworkId;
    use crate::display_io::DisplayIO;
    use crate::instruction::Instruction;
    use crate::instruction_extractor::*;
    use crate::sbor_decoder::*;
    use crate::tx_intent_test_data::tests::*;
    use core::cmp::min;
    use core::str::from_utf8;

    static PRINTER: TestPrinter = TestPrinter {};

    struct TestPrinter {}

    impl DisplayIO for TestPrinter {
        fn scroll(&self, message: &[u8]) {
            print!("{} ", from_utf8(message).unwrap());
        }

        fn ask(&self, _question: &str) -> bool {
            true
        }
    }

    struct InstructionProcessor {
        extractor: InstructionExtractor,
        handler: InstructionFormatter,
    }

    struct InstructionFormatter {
        instruction_count: usize,
        instructions: [Instruction; Self::SIZE],
        printer: InstructionPrinter,
    }

    impl InstructionProcessor {
        pub fn new() -> Self {
            Self {
                extractor: InstructionExtractor::new(),
                handler: InstructionFormatter::new(),
            }
        }
    }

    impl InstructionFormatter {
        pub const SIZE: usize = 20;
        pub fn new() -> Self {
            Self {
                instruction_count: 0,
                instructions: [Instruction::TakeFromWorktop; Self::SIZE],
                printer: InstructionPrinter::new(&PRINTER, NetworkId::Simulator),
            }
        }

        pub fn verify(&self, expected: &[Instruction]) {
            assert_eq!(self.instruction_count, expected.len());
            let mut cnt = 0;
            self.instructions[..self.instruction_count]
                .iter()
                .zip(expected)
                .all(|(a, b)| {
                    assert_eq!(*a, *b, "Elements are not equal at index {}", cnt);
                    cnt += 1;
                    true
                });
        }
    }

    impl SborEventHandler for InstructionProcessor {
        fn handle(&mut self, evt: SborEvent) {
            self.extractor.handle_event(&mut self.handler, evt);
        }
    }

    impl InstructionHandler for InstructionFormatter {
        fn handle(&mut self, event: ExtractorEvent) {
            if let ExtractorEvent::InstructionStart(info) = event {
                self.instructions[self.instruction_count] = info.instruction;
                self.instruction_count += 1;
                //println!("Instruction::{:?},", info.instruction);
            }

            self.printer.handle(event);

            // println!("Event: {:?}", event);
        }
    }

    const CHUNK_SIZE: usize = 255;

    fn check_partial_decoding(input: &[u8], expected_instructions: &[Instruction]) {
        let mut decoder = SborDecoder::new(true);
        let mut handler = InstructionProcessor::new();

        let mut start = 0;
        let mut end = min(input.len(), CHUNK_SIZE);

        while start < input.len() {
            match decoder.decode(&mut handler, &input[start..end]) {
                Ok(outcome) => {
                    if end - start == CHUNK_SIZE {
                        assert_eq!(outcome, DecodingOutcome::NeedMoreData(end));
                    } else {
                        assert_eq!(outcome, DecodingOutcome::Done(input.len()))
                    }
                }
                Err(err) => {
                    assert!(false, "Should not return an error {:?}", err)
                }
            }

            start += CHUNK_SIZE;
            end += CHUNK_SIZE;

            if end >= input.len() {
                end = input.len();
            }
            // println!("start, end, len = {}, {}, {}", start, end, input.len());
        }

        //println!("Total {} instructions", handler.handler.instruction_count);
        handler.handler.verify(expected_instructions);
        println!();
    }

    #[test]
    pub fn test_push_byte_for_string() {
        let mut state = ParameterPrinterState::new(NetworkId::LocalNet);
        for i in 0..ParameterPrinterState::PARAMETER_AREA_SIZE {
            if state.data_counter != (i as u8) {
                assert_eq!(
                    state.data_counter as usize,
                    ParameterPrinterState::PARAMETER_AREA_SIZE
                );
                return;
            }
            state.push_byte_for_string(b'a');
        }
        assert!(false, "Should not reach here!")
    }

    #[test]
    pub fn test_access_rule() {
        check_partial_decoding(&TX_ACCESS_RULE, &[Instruction::SetMethodAccessRule]);
    }

    #[test]
    pub fn test_assert_access_rule() {
        check_partial_decoding(
            &TX_ASSERT_ACCESS_RULE,
            &[Instruction::CallMethod, Instruction::AssertAccessRule],
        );
    }

    #[test]
    pub fn test_call_function() {
        check_partial_decoding(&TX_CALL_FUNCTION, &[Instruction::CallFunction]);
    }

    #[test]
    pub fn test_call_method() {
        check_partial_decoding(&TX_CALL_METHOD, &[Instruction::CallMethod]);
    }

    #[test]
    pub fn test_create_access_controller() {
        check_partial_decoding(
            &TX_CREATE_ACCESS_CONTROLLER,
            &[Instruction::TakeFromWorktop, Instruction::CallFunction],
        );
    }

    #[test]
    pub fn test_create_account() {
        check_partial_decoding(&TX_CREATE_ACCOUNT, &[Instruction::CallFunction]);
    }

    #[test]
    pub fn test_create_fungible_resource_with_initial_supply() {
        check_partial_decoding(
            &TX_CREATE_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY,
            &[
                Instruction::CallMethod,
                Instruction::CallFunction,
                Instruction::CallMethod,
            ],
        );
    }

    #[test]
    pub fn test_create_fungible_resource_with_no_initial_supply() {
        check_partial_decoding(
            &TX_CREATE_FUNGIBLE_RESOURCE_WITH_NO_INITIAL_SUPPLY,
            &[Instruction::CallMethod, Instruction::CallFunction],
        );
    }

    #[test]
    pub fn test_create_identity() {
        check_partial_decoding(&TX_CREATE_IDENTITY, &[Instruction::CallFunction]);
    }

    #[test]
    pub fn test_create_non_fungible_resource_with_no_initial_supply() {
        check_partial_decoding(
            &TX_CREATE_NON_FUNGIBLE_RESOURCE_WITH_NO_INITIAL_SUPPLY,
            &[Instruction::CallMethod, Instruction::CallFunction],
        );
    }

    #[test]
    pub fn test_metadata() {
        check_partial_decoding(
            &TX_METADATA,
            &[
                Instruction::SetMetadata,
                Instruction::SetMetadata,
                Instruction::SetMetadata,
                Instruction::RemoveMetadata,
                Instruction::RemoveMetadata,
                Instruction::RemoveMetadata,
            ],
        );
    }

    #[test]
    pub fn test_mint_fungible() {
        check_partial_decoding(
            &TX_MINT_FUNGIBLE,
            &[
                Instruction::CallMethod,
                Instruction::CallMethod,
                Instruction::MintFungible,
                Instruction::CallMethod,
            ],
        );
    }

    #[test]
    pub fn test_mint_non_fungible() {
        check_partial_decoding(
            &TX_MINT_NON_FUNGIBLE,
            &[
                Instruction::CallMethod,
                Instruction::CallMethod,
                Instruction::MintNonFungible,
                Instruction::CallMethod,
            ],
        );
    }

    #[test]
    pub fn test_publish_package() {
        check_partial_decoding(
            &TX_PUBLISH_PACKAGE,
            &[Instruction::CallMethod, Instruction::PublishPackage],
        );
    }

    #[test]
    pub fn test_resource_recall() {
        check_partial_decoding(&TX_RESOURCE_RECALL, &[Instruction::RecallResource]);
    }

    #[test]
    pub fn test_resource_worktop() {
        check_partial_decoding(
            &TX_RESOURCE_WORKTOP,
            &[
                Instruction::CallMethod,
                Instruction::TakeFromWorktopByAmount,
                Instruction::CallMethod,
                Instruction::AssertWorktopContainsByAmount,
                Instruction::AssertWorktopContains,
                Instruction::TakeFromWorktop,
                Instruction::ReturnToWorktop,
                Instruction::TakeFromWorktopByIds,
                Instruction::CallMethod,
            ],
        );
    }

    #[test]
    pub fn test_royalty() {
        check_partial_decoding(
            &TX_ROYALTY,
            &[
                Instruction::SetPackageRoyaltyConfig,
                Instruction::SetComponentRoyaltyConfig,
                Instruction::ClaimPackageRoyalty,
                Instruction::ClaimComponentRoyalty,
            ],
        );
    }

    #[test]
    pub fn test_values() {
        check_partial_decoding(
            &TX_VALUES,
            &[
                Instruction::TakeFromWorktop,
                Instruction::CreateProofFromAuthZone,
                Instruction::CallMethod,
                Instruction::CallMethod,
            ],
        );
    }
}
