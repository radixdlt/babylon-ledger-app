// Process events received from decoder and extract data related to instructions

use crate::instruction::{to_instruction, Instruction};
use crate::sbor_notifications::SborEvent;
use crate::type_info::{TYPE_ARRAY, TYPE_DATA_BUFFER_SIZE, TYPE_ENUM, TYPE_MAP};

#[repr(u8)]
enum ExtractorPhases {
    WaitingForInstructionsStruct,
    WaitingForInstructionsArray,
    CollectingInstructions,
    Done,
}

#[derive(Debug)]
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum ExtractionError {
    UnknownInstruction(u8),
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum ExtractorEvent<'a> {
    InstructionStart {
        instruction: Instruction,
        parameter_count: u8,
    },
    ParameterStart {
        type_id: u8,
        nesting_level: u8,
    },
    ParameterData {
        start: usize,
        data: &'a [u8],
        is_enum_name: bool,
    },
    ParameterEnd,
    InstructionEnd,
    Error(ExtractionError),
}

pub trait InstructionHandler {
    fn handle(&mut self, event: ExtractorEvent);
}

pub struct InstructionExtractor {
    data_len: usize,
    data_ptr: usize,
    data_start: usize,
    data: [u8; TYPE_DATA_BUFFER_SIZE],
    counter: u8,
    phase: ExtractorPhases,
    current_nesting: u8,
    instruction_ready: bool,
    instruction: Option<Instruction>,
    chunked_data: bool,
}

impl InstructionExtractor {
    pub fn new() -> Self {
        Self {
            data_len: 0,
            data_ptr: 0,
            data_start: 0,
            data: [0; TYPE_DATA_BUFFER_SIZE],
            counter: 0,
            phase: ExtractorPhases::WaitingForInstructionsStruct,
            current_nesting: 0,
            instruction_ready: false,
            instruction: None,
            chunked_data: false,
        }
    }

    pub fn handle_event(&mut self, handler: &mut impl InstructionHandler, event: SborEvent) {
        match self.phase {
            ExtractorPhases::WaitingForInstructionsStruct => {
                self.wait_for_instruction_struct(event)
            }
            ExtractorPhases::WaitingForInstructionsArray => self.wait_for_instruction_array(event),
            ExtractorPhases::CollectingInstructions => self.process_instruction(event, handler),
            ExtractorPhases::Done => {}
        }
    }

    // Skip everything until second structure appears in the stream (field of top-level struct)
    fn wait_for_instruction_struct(&mut self, event: SborEvent) {
        match event {
            SborEvent::Start {
                type_id,
                nesting_level,
                fixed_size: _,
            } => {
                if nesting_level == 1 && type_id == TYPE_MAP {
                    self.counter += 1;
                }

                if self.counter == 2 {
                    self.phase = ExtractorPhases::WaitingForInstructionsArray;
                }
            }
            _ => {}
        }
    }

    // Skip wrapping types until actual array of instructions appear
    fn wait_for_instruction_array(&mut self, event: SborEvent) {
        match event {
            SborEvent::Start {
                type_id,
                nesting_level,
                fixed_size: _,
            } => {
                if type_id == TYPE_ARRAY && nesting_level == 2 {
                    self.phase = ExtractorPhases::CollectingInstructions;
                    self.current_nesting = nesting_level;
                }
            }
            _ => {}
        }
    }

    fn process_instruction(&mut self, event: SborEvent, handler: &mut impl InstructionHandler) {
        match event {
            SborEvent::Start {
                type_id,
                nesting_level,
                fixed_size,
            } => {
                self.current_nesting = nesting_level;
                self.data_len = fixed_size as usize;

                if nesting_level >= 4 {
                    handler.handle(ExtractorEvent::ParameterStart {
                        type_id,
                        nesting_level: nesting_level - 4,
                    });
                }
            }

            SborEvent::Discriminator(id) => {
                self.instruction = to_instruction(id);
                self.instruction_ready = true;

                match self.instruction {
                    None => handler.handle(ExtractorEvent::Error(
                        ExtractionError::UnknownInstruction(id),
                    )),
                    _ => self.instruction_ready = true,
                }
            }

            SborEvent::Len(len) => {
                if self.instruction_ready {
                    // len contains number of instruction parameters
                    self.instruction_ready = false;

                    match self.instruction {
                        Some(instruction) => handler.handle(ExtractorEvent::InstructionStart {
                            instruction,
                            parameter_count: len as u8,
                        }),
                        _ => {}
                    };
                }

                self.data_len = len as usize;
                self.data_ptr = 0;
                self.data_start = 0;
                self.chunked_data = self.data_len > TYPE_DATA_BUFFER_SIZE;
            }

            SborEvent::Data(byte) => {
                self.data[self.data_ptr - self.data_start] = byte;
                self.data_ptr += 1;

                let end_of_chunk = if self.chunked_data {
                    self.data_ptr - self.data_start == TYPE_DATA_BUFFER_SIZE
                } else {
                    self.data_ptr == self.data_len
                };

                if end_of_chunk {
                    if self.current_nesting == 3 {
                        self.instruction_ready = true;
                    }

                    if self.current_nesting >= 4 {
                        handler.handle(ExtractorEvent::ParameterData {
                            start: self.data_start,
                            data: &self.data[0..(self.data_ptr - self.data_start)],
                            is_enum_name: event == SborEvent::Discriminator(byte),
                        });

                        if self.chunked_data {
                            self.data_start += TYPE_DATA_BUFFER_SIZE;
                        }
                    }
                }
            }

            SborEvent::End {
                type_id,
                nesting_level,
            } => {
                self.current_nesting = nesting_level;

                match nesting_level {
                    2 => self.phase = ExtractorPhases::Done,
                    3 => {
                        if type_id == TYPE_ENUM {
                            handler.handle(ExtractorEvent::InstructionEnd);
                        }
                    }
                    4..=255 => {
                        handler.handle(ExtractorEvent::ParameterEnd);
                    }

                    _ => {}
                }
            }
        }
    }
}
//todo!("Add tests; Add test for chunked data reporting!");
