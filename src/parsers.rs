use num_derive::FromPrimitive;

use crate::types::{OpTypePointer, IsOpcode, OpVariable, OpEntryPoint};

fn parse_string(slice: &[u32]) -> (String, usize) {
    // find the end (\0)
    let mut temp_data = Vec::<u32>::new();

    for word in slice {
        temp_data.push(*word);
        if *word == 0 {
            break;
        }
    }

    let temp_data_ptr = temp_data.as_ptr() as *const u8;
    let temp_data_len = temp_data.len() * 4;

    let mut data = vec![0; temp_data_len];

    unsafe {
        std::ptr::copy_nonoverlapping(temp_data_ptr, data.as_mut_ptr(), temp_data_len);
    }

    let output_string = std::str::from_utf8(&data).unwrap();

    return (output_string.to_string(), temp_data.len());
}

pub trait ParseOpCode<T> {
    fn parse_into(opcode: &OpCodeUnparsed, by_opcodes: &Vec<OpCodeUnparsed>) -> T;
}

#[derive(Default, Debug, Clone)]
pub struct OpCodeUnparsed {
    pub opcode: u16,
    pub length: usize,
    pub result_id: u32, // 0 means None and is default
    pub data: Vec<u32>,
}

pub trait OpCodeUnparsedResultIdLookupHelper {
    fn result_id_lookup(&self, id: u32) -> Option<&OpCodeUnparsed>;
}

impl OpCodeUnparsedResultIdLookupHelper for Vec<OpCodeUnparsed> {
    fn result_id_lookup(&self, id: u32) -> Option<&OpCodeUnparsed> {
        if id == 0 {
            return None;
        } else {
            let res = self.iter().find(|v| v.result_id == id);
            return res;
        }
    }
}

impl ParseOpCode<OpTypePointer> for OpCodeUnparsed {
    fn parse_into(opcode: &OpCodeUnparsed, by_opcodes: &Vec<OpCodeUnparsed>) -> OpTypePointer {
        if opcode.opcode != OpTypePointer::opcode() as u16 {
            panic!("Yeet");
        } else {
            OpTypePointer {
                result_id: opcode.data[1],
                storage_class: num::FromPrimitive::from_u32(opcode.data[2]).unwrap(),
                type_id: opcode.data[3],
            }
        }
    }
}

impl ParseOpCode<OpVariable> for OpCodeUnparsed {
    fn parse_into(opcode: &OpCodeUnparsed, by_opcodes: &Vec<OpCodeUnparsed>) -> OpVariable {
        if opcode.opcode != OpVariable::opcode() as u16 {
            panic!("Yeet");
        } else {
            let pointer_unparsed = by_opcodes.result_id_lookup(opcode.data[1]).unwrap();
            let pointer: OpTypePointer = <OpCodeUnparsed as ParseOpCode<OpTypePointer>>::parse_into(
                &pointer_unparsed,
                by_opcodes,
            );

            // TODO: Initializer id
            OpVariable {
                result_type: pointer,
                result_id: opcode.data[2],
                storage_class: num::FromPrimitive::from_u32(opcode.data[3]).unwrap(),
                // initializer_id: opcode.data[4],
            }
        }
    }
}
impl ParseOpCode<OpEntryPoint> for OpCodeUnparsed {
    fn parse_into(opcode: &OpCodeUnparsed, by_opcodes: &Vec<OpCodeUnparsed>) -> OpEntryPoint {
        if opcode.opcode != OpEntryPoint::opcode() as u16 {
            panic!("Yeet");
        } else {
            let (name, name_length_words) = parse_string(&opcode.data[3..]);

            let slic = &opcode.data[3 + name_length_words..];
            let interfaces: Vec<OpVariable> = slic
                .iter()
                .map(|v| -> OpVariable {
                    let interface_opcode = by_opcodes.result_id_lookup(*v).unwrap();
                    let interface_parsed: OpVariable = <OpCodeUnparsed as ParseOpCode<
                        OpVariable,
                    >>::parse_into(
                        interface_opcode, by_opcodes
                    );
                    interface_parsed
                })
                .collect();
            OpEntryPoint {
                execution_model: num::FromPrimitive::from_u32(opcode.data[1]).unwrap(),
                entry_point_result_id_op_function: opcode.data[2],
                name,
                interfaces,
            }
        }
    }
}
