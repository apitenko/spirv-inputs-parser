use num_derive::FromPrimitive;

use crate::{
    types::{IsOpcode, OpEntryPoint, OpTypePointer, OpVariable, OpTypeWrapper},
    util::{lookup_and_parse, parse_string},
};

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

impl ParseOpCode<OpTypePointer> for OpCodeUnparsed {
    fn parse_into(opcode: &OpCodeUnparsed, by_opcodes: &Vec<OpCodeUnparsed>) -> OpTypePointer {
        if opcode.opcode != OpTypePointer::opcode() as u16 {
            panic!("Yeet");
        } else {
            OpTypePointer {
                result_id: opcode.data[1],
                storage_class: num::FromPrimitive::from_u32(opcode.data[2]).unwrap(),
                type_id: lookup_and_parse::<OpTypeWrapper>(opcode.data[3], by_opcodes),
            }
        }
    }
}

impl ParseOpCode<OpVariable> for OpCodeUnparsed {
    fn parse_into(opcode: &OpCodeUnparsed, by_opcodes: &Vec<OpCodeUnparsed>) -> OpVariable {
        if opcode.opcode != OpVariable::opcode() as u16 {
            panic!("Yeet");
        } else {
            let pointer = lookup_and_parse::<OpTypePointer>(opcode.data[1], by_opcodes);

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
                .map(|v| -> OpVariable { lookup_and_parse::<OpVariable>(*v, by_opcodes) })
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
