// Process events received from decoder and extract data related to instructions

use crate::instruction::{to_instruction, Instruction};
use crate::sbor_notifications::SborEvent;
use crate::type_info::{TYPE_ARRAY, TYPE_DATA_BUFFER_SIZE, TYPE_ENUM, TYPE_STRUCT};

#[repr(u8)]
enum ExtractorPhases {
    WaitingForInstructionsStruct,
    WaitingForInstructionsArray,
    CollectingInstructions,
    Done,
}

#[derive(Debug)]
#[repr(u8)]
pub enum ExtractionError {
    UnknownInstruction,
}

#[repr(u8)]
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
        data: &'a [u8],
        is_enum_name: bool,
    },
    ParameterEnd,
    InstructionEnd,
    Error(ExtractionError),
}

pub struct InstructionExtractor<T> {
    handler: fn(&mut T, ExtractorEvent) -> (),
    data_len: usize,
    data_ptr: usize,
    data: [u8; TYPE_DATA_BUFFER_SIZE],
    counter: u8,
    phase: ExtractorPhases,
    current_nesting: u8,
    instruction_ready: bool,
}

impl<T> InstructionExtractor<T> {
    pub fn new(fun: fn(&mut T, ExtractorEvent) -> ()) -> Self {
        Self {
            handler: fun,
            data_len: 0,
            data_ptr: 0,
            data: [0; TYPE_DATA_BUFFER_SIZE],
            counter: 0,
            phase: ExtractorPhases::WaitingForInstructionsStruct,
            current_nesting: 0,
            instruction_ready: false,
        }
    }

    pub fn handle_event(&mut self, opaque: &mut T, event: SborEvent) {
        match self.phase {
            ExtractorPhases::WaitingForInstructionsStruct => {
                self.wait_for_instruction_struct(event)
            }
            ExtractorPhases::WaitingForInstructionsArray => self.wait_for_instruction_array(event),
            ExtractorPhases::CollectingInstructions => self.process_instruction(event, opaque),
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
                if nesting_level == 1 && type_id == TYPE_STRUCT {
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

    fn process_instruction(&mut self, event: SborEvent, opaque: &mut T) {
        match event {
            SborEvent::Start {
                type_id,
                nesting_level,
                fixed_size,
            } => {
                self.current_nesting = nesting_level;
                self.data_len = fixed_size as usize;

                if nesting_level >= 4 {
                    (self.handler)(opaque, ExtractorEvent::ParameterStart {
                        type_id,
                        nesting_level: nesting_level - 4,
                    });
                }
            }

            SborEvent::NameLen(len) | SborEvent::Len(len) => {
                if self.instruction_ready {
                    self.instruction_ready = false;

                    match to_instruction(&self.data[0..self.data_ptr]) {
                        None => (self.handler)(opaque, ExtractorEvent::Error(
                            ExtractionError::UnknownInstruction,
                        )),
                        Some(instruction) => (self.handler)(opaque, ExtractorEvent::InstructionStart {
                            instruction,
                            parameter_count: len as u8,
                        }),
                    };
                }

                self.data_len = len as usize;
                self.data_ptr = 0;
            }

            SborEvent::Data(byte) | SborEvent::Name(byte) => {
                self.data[self.data_ptr] = byte;
                self.data_ptr += 1;

                if self.data_ptr == self.data_len {
                    if self.current_nesting == 3 {
                        self.instruction_ready = true;
                    }

                    if self.current_nesting >= 4 {
                        (self.handler)(opaque, ExtractorEvent::ParameterData {
                            data: &self.data[0..self.data_ptr],
                            is_enum_name: event == SborEvent::Name(byte),
                        });
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
                            (self.handler)(opaque, ExtractorEvent::InstructionEnd);
                        }
                    }
                    4..=255 => {
                        (self.handler)(opaque, ExtractorEvent::ParameterEnd);
                    }

                    _ => {}
                }
            }
        }
    }
}
