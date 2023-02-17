use std::collections::HashSet;
use std::rc::Rc;

use num_derive::FromPrimitive;

use crate::types::*;
use crate::util::{lookup_and_parse, parse_string};
use crate::{
    parsers::{OpCodeUnparsed, ParseOpCode},
    types::IsOpcode,
};

impl ParseOpCode<OpTypeWrapper> for OpCodeUnparsed {
    fn parse_into(opcode: &OpCodeUnparsed, by_opcodes: &Vec<OpCodeUnparsed>) -> OpTypeWrapper {
        match OpCodes::from(opcode.opcode) {
            OpCodes::OpTypeStruct => OpTypeWrapper::OpTypeStruct(Box::new(OpTypeStruct {
                result_id: opcode.result_id,
                members: opcode.data[2..]
                    .iter()
                    .map(|v| lookup_and_parse::<OpTypeWrapper>(*v, by_opcodes))
                    .collect(),
            })),
            OpCodes::OpTypeArray => OpTypeWrapper::OpTypeArray(Box::new(OpTypeArray {
                result_id: opcode.result_id,
                element_type: lookup_and_parse::<OpTypeWrapper>(opcode.data[2], by_opcodes),
                length: opcode.data[3],
            })),
            // OpCodes::OpTypeSampledImage => {},
            // OpCodes::OpTypeSampler => {},
            // OpCodes::OpTypeImage => {},
            OpCodes::OpTypeMatrix => OpTypeWrapper::OpTypeMatrix(Box::new(OpTypeMatrix {
                result_id: opcode.result_id,
                column_type: lookup_and_parse::<OpTypeWrapper>(opcode.data[2], by_opcodes), // ? must be vector type; we don't do validation here
                column_count: opcode.data[3],
            })),
            OpCodes::OpTypeVector => OpTypeWrapper::OpTypeVector(Box::new(OpTypeVector {
                result_id: opcode.result_id,
                element_type: lookup_and_parse::<OpTypeWrapper>(opcode.data[2], by_opcodes),
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
            OpCodes::OpTypeBool => OpTypeWrapper::OpTypeBool(Box::new(OpTypeBool {
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
    fn parse_into(opcode: &OpCodeUnparsed, by_opcodes: &Vec<OpCodeUnparsed>) -> OpTypeStruct {
        if opcode.opcode != OpTypeStruct::opcode() as u16 {
            panic!("Yeet");
        } else {
            OpTypeStruct {
                result_id: opcode.result_id,
                members: opcode.data[2..]
                    .iter()
                    .map(|v| lookup_and_parse::<OpTypeWrapper>(*v, by_opcodes))
                    .collect(),
            }
        }
    }
}
