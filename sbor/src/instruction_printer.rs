use crate::bech32::encoder::*;
use crate::bech32::hrp::*;
use crate::bech32::network::*;
use crate::display_io::DisplayIO;
use crate::instruction::{InstructionInfo, ParameterType};
use crate::instruction_extractor::{ExtractorEvent, InstructionHandler};
use crate::sbor_decoder::SborEvent;
use crate::type_info::ADDRESS_LEN;
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
    nesting_level: u8,
    flip_flop: bool,
    network_id: NetworkId,
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
            .filter(|info| (info.parameter_count as u32) > ordinal)
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
            nesting_level: 0,
            flip_flop: false,
            network_id,
        }
    }

    pub fn set_network(&mut self, network_id: NetworkId) {
        self.network_id = network_id;
    }

    pub fn reset(&mut self) {
        self.data = [0; Self::PARAMETER_AREA_SIZE];
        self.data_counter = 0;
        self.nesting_level = 0;
        self.flip_flop = false;
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
        ParameterType::MethodKey => &IGNORED_PARAMETER_PRINTER,
        ParameterType::AccessRules => &IGNORED_PARAMETER_PRINTER,
        ParameterType::BTreeMapByNonFungibleLocalId => &IGNORED_PARAMETER_PRINTER,
        ParameterType::BTreeMapByStringToRoyaltyConfig => &IGNORED_PARAMETER_PRINTER,
        ParameterType::BTreeMapByStringToString => &IGNORED_PARAMETER_PRINTER,
        ParameterType::BTreeSetOfNonFungibleLocalId => &IGNORED_PARAMETER_PRINTER,
        ParameterType::ComponentAddress => &COMPONENT_ADDRESS_PARAMETER_PRINTER,
        ParameterType::Decimal => &IGNORED_PARAMETER_PRINTER,
        ParameterType::ManifestAddress => &IGNORED_PARAMETER_PRINTER,
        ParameterType::ManifestBlobRef => &IGNORED_PARAMETER_PRINTER,
        ParameterType::ManifestBucket => &IGNORED_PARAMETER_PRINTER,
        ParameterType::ManifestProof => &IGNORED_PARAMETER_PRINTER,
        ParameterType::PackageAddress => &PACKAGE_ADDRESS_PARAMETER_PRINTER,
        ParameterType::ResourceAddress => &RESOURCE_ADDRESS_PARAMETER_PRINTER,
        ParameterType::RoyaltyConfig => &IGNORED_PARAMETER_PRINTER,
        ParameterType::String => &STRING_PARAMETER_PRINTER,
        ParameterType::ObjectId => &IGNORED_PARAMETER_PRINTER,
        ParameterType::VecOfVecTuple => &IGNORED_PARAMETER_PRINTER,
        ParameterType::VecOfU8 => &VEC_OF_U8_PARAMETER_PRINTER,
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
            if state.data_counter as usize == ParameterPrinterState::PARAMETER_AREA_SIZE {
                return;
            }

            state.data[state.data_counter as usize] = byte;
            state.data_counter += 1;

            if state.data_counter as usize == ParameterPrinterState::PARAMETER_AREA_SIZE - 4 {
                state.data[state.data_counter as usize + 1] = b'.';
                state.data[state.data_counter as usize + 2] = b'.';
                state.data[state.data_counter as usize + 3] = b'.';
                state.data_counter += 3;
            }
        }
    }

    fn display(&self, state: &ParameterPrinterState, display: &'static dyn DisplayIO) {
        match from_utf8(&state.data[..(state.data_counter as usize)]) {
            Ok(message) => display.scroll(message.as_bytes()),
            Err(_) => display.scroll(b"<String decoding error>"),
        }
    }
}

// Vec<u8> parameter printer
struct VecOfU8ParameterPrinter {}

const VEC_OF_U8_PARAMETER_PRINTER: VecOfU8ParameterPrinter = VecOfU8ParameterPrinter {};

impl ParameterPrinter for VecOfU8ParameterPrinter {
    fn handle_data_event(
        &self,
        state: &mut ParameterPrinterState,
        event: SborEvent,
        display: &'static dyn DisplayIO,
    ) {
        if let SborEvent::Data(byte) = event {
            if state.data_counter as usize == ParameterPrinterState::PARAMETER_AREA_SIZE {
                self.display(state, display);
                state.data_counter = 0;
                return;
            }

            state.data[state.data_counter as usize] = byte;
            state.data_counter += 1;
        }
    }

    fn display(&self, state: &ParameterPrinterState, display: &'static dyn DisplayIO) {
        let mut hex = [0u8; ParameterPrinterState::PARAMETER_AREA_SIZE * 2];

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

// Address printers

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

impl ParameterPrinter for AddressParameterPrinter {
    fn handle_data_event(
        &self,
        state: &mut ParameterPrinterState,
        event: SborEvent,
        _display: &'static dyn DisplayIO,
    ) {
        if let SborEvent::Data(byte) = event {
            if state.data_counter > ADDRESS_LEN {
                //TODO: an error condition, should we handle it somehow?
            }

            state.data[state.data_counter as usize] = byte;
            state.data_counter += 1;
        }
    }

    fn display(&self, state: &ParameterPrinterState, display: &'static dyn DisplayIO) {
        match hrp_prefix(self.resource_id, state.data[0]) {
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
                        self.resource_id,
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
// Vec<u8> parameter printer
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

            state.data[state.data_counter as usize] = byte;
            state.data_counter += 1;
        }
    }

    fn display(&self, state: &ParameterPrinterState, display: &'static dyn DisplayIO) {
        let message:&[u8] = match (state.data_counter, state.data[0]) {
            (1, 0) => b"AllowAll",
            (1, 1) => b"DenyAll",
            (1, 2) => b"Protected(<rules not decoded>)",
            (1, _) => b"<unknown access rule>",
            (_, _) => b"<invalid encoding>",
        };

        display.scroll(message);
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
    use core::cmp::min;
    use core::str::from_utf8;
    use crate::tx_intent_test_data::tests::*;

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
                printer: InstructionPrinter::new(&PRINTER, NetworkId::LocalNet),
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
    pub fn test_access_rule() {
        check_partial_decoding(&TX_ACCESS_RULE, &[Instruction::SetMethodAccessRule]);
    }
}
