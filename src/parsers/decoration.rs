use crate::types::decoration::OpDecorate;

use crate::types::decoration::*;
use crate::types::unparsed::{OpCodeUnparsed, ParseOpCode};
use crate::types::variables::*;

use super::unparsed::OpcodesById;

fn parse_decoration_data(decoration: Decoration, decoration_data: &[u32]) -> DecorationData {
    // TODO: incomplete, parse all decoration data
    match decoration {
        _ => DecorationData::None,
        Location => {
            return DecorationData::Location(DecorationDataLocation {
                location: decoration_data[0],
            })
        }
        Index => {
            return DecorationData::Index(DecorationDataIndex {
                index: decoration_data[0],
            })
        }
        Binding => {
            return DecorationData::Binding(DecorationDataBinding {
                binding_point: decoration_data[0],
            })
        }
        DescriptorSet => {
            return DecorationData::DescriptorSet(DecorationDataDescriptorSet {
                descriptor_set: decoration_data[0],
            })
        }
        Offset => {
            return DecorationData::Offset(DecorationDataOffset {
                offset: decoration_data[0],
            })
        }
    }
}

impl ParseOpCode<OpDecorate> for OpCodeUnparsed {
    fn parse_into(opcode: &OpCodeUnparsed, opcodes_by_id: &OpcodesById) -> OpDecorate {
        if opcode.opcode != OpDecorate::opcode() as u16 {
            panic!("Yeet");
        } else {
            let decoration = num::FromPrimitive::from_u32(opcode.data[2]).unwrap();

            OpDecorate {
                target_id: opcode.data[1],
                decoration,
                decoration_data: parse_decoration_data(decoration, &opcode.data[3..]),
            }
        }
    }
}

impl ParseOpCode<OpMemberDecorate> for OpCodeUnparsed {
    fn parse_into(opcode: &OpCodeUnparsed, opcodes_by_id: &OpcodesById) -> OpMemberDecorate {
        if opcode.opcode != OpMemberDecorate::opcode() as u16 {
            panic!("Yeet");
        } else {
            let decoration = num::FromPrimitive::from_u32(opcode.data[3]).unwrap();

            OpMemberDecorate {
                target_id: opcode.data[1],
                member: opcode.data[2],
                decoration,
                decoration_data: parse_decoration_data(decoration, &opcode.data[3..]),
            }
        }
    }
}
