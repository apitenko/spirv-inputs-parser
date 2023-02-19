use crate::types::unparsed::{ParseOpCode, OpCodeUnparsed};
use crate::util::{lookup_and_parse, parse_string};
use crate::types::variables::*;

use super::unparsed::OpcodesById;

impl ParseOpCode<OpTypePointer> for OpCodeUnparsed {
    fn parse_into(opcode: &OpCodeUnparsed, opcodes_by_id: &OpcodesById) -> OpTypePointer {
        if opcode.opcode != OpTypePointer::opcode() as u16 {
            panic!("Yeet");
        } else {
            OpTypePointer {
                result_id: opcode.data[1],
                storage_class: num::FromPrimitive::from_u32(opcode.data[2]).unwrap(),
                type_id: lookup_and_parse::<OpTypeWrapper>(opcode.data[3], opcodes_by_id),
            }
        }
    }
}

impl ParseOpCode<OpVariable> for OpCodeUnparsed {
    fn parse_into(opcode: &OpCodeUnparsed, opcodes_by_id: &OpcodesById) -> OpVariable {
        if opcode.opcode != OpVariable::opcode() as u16 {
            panic!("Yeet");
        } else {
            let pointer = lookup_and_parse::<OpTypePointer>(opcode.data[1], opcodes_by_id);

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
    fn parse_into(opcode: &OpCodeUnparsed, opcodes_by_id: &OpcodesById) -> OpEntryPoint {
        if opcode.opcode != OpEntryPoint::opcode() as u16 {
            panic!("Yeet");
        } else {
            let (name, name_length_words) = parse_string(&opcode.data[3..]);

            let slic = &opcode.data[3 + name_length_words..];
            let interfaces: Vec<OpVariable> = slic
                .iter()
                .map(|v| -> OpVariable { lookup_and_parse::<OpVariable>(*v, opcodes_by_id) })
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


impl ParseOpCode<OpTypeWrapper> for OpCodeUnparsed {
    fn parse_into(opcode: &OpCodeUnparsed, opcodes_by_id: &OpcodesById) -> OpTypeWrapper {
        match OpCodes::from(opcode.opcode) {
            OpCodes::OpTypeStruct => OpTypeWrapper::OpTypeStruct(Box::new(OpTypeStruct {
                result_id: opcode.result_id,
                members: opcode.data[2..]
                    .iter()
                    .map(|v| lookup_and_parse::<OpTypeWrapper>(*v, opcodes_by_id))
                    .collect(),
            })),
            OpCodes::OpTypeArray => OpTypeWrapper::OpTypeArray(Box::new(OpTypeArray {
                result_id: opcode.result_id,
                element_type: lookup_and_parse::<OpTypeWrapper>(opcode.data[2], opcodes_by_id),
                length: opcode.data[3],
            })),
            // OpCodes::OpTypeSampledImage => {},
            // OpCodes::OpTypeSampler => {},
            // OpCodes::OpTypeImage => {},
            OpCodes::OpTypeMatrix => OpTypeWrapper::OpTypeMatrix(Box::new(OpTypeMatrix {
                result_id: opcode.result_id,
                column_type: lookup_and_parse::<OpTypeWrapper>(opcode.data[2], opcodes_by_id), // ? must be vector type; we don't do validation here
                column_count: opcode.data[3],
            })),
            OpCodes::OpTypeVector => OpTypeWrapper::OpTypeVector(Box::new(OpTypeVector {
                result_id: opcode.result_id,
                element_type: lookup_and_parse::<OpTypeWrapper>(opcode.data[2], opcodes_by_id),
                length: opcode.data[3],
            })),
            OpCodes::OpTypeFloat => OpTypeWrapper::OpTypeFloat(Box::new(OpTypeFloat {
                result_id: opcode.result_id,
                width: opcode.data[2],
            })),
            OpCodes::OpTypeInt => OpTypeWrapper::OpTypeInt(Box::new(OpTypeInt {
                result_id: opcode.result_id,
                width: opcode.data[2],
                signedness: Signedness::from(opcode.data[3]),
            })),
            OpCodes::OpTypeBool => OpTypeWrapper::OpTypeBool(Box::new(OpTypeBool{
                result_id: opcode.result_id,
            })),
            OpCodes::OpTypeVoid => OpTypeWrapper::OpTypeVoid(Box::new(OpTypeVoid {
                result_id: opcode.result_id,
            })),
            _ => panic!("Yeeeeeeeeeeet"),
        }
    }
}

impl ParseOpCode<OpTypeStruct> for OpCodeUnparsed {
    fn parse_into(opcode: &OpCodeUnparsed, opcodes_by_id: &OpcodesById) -> OpTypeStruct {
        if opcode.opcode != OpTypeStruct::opcode() as u16 {
            panic!("Yeet");
        } else {
            OpTypeStruct {
                result_id: opcode.result_id,
                members: opcode.data[2..]
                    .iter()
                    .map(|v| lookup_and_parse::<OpTypeWrapper>(*v, opcodes_by_id))
                    .collect(),
            }
        }
    }
}
