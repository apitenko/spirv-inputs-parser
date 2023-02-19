use std::collections::HashMap;

use crate::types::{unparsed::OpCodeUnparsed, variables::OpCodes};

pub type OpcodesById = HashMap<u32, OpCodeUnparsed>;

pub fn check_magic(buf: &Vec<u32>) -> anyhow::Result<()> {
    const SPIRV_MAGIC: u32 = 0x07230203;

    if buf[0] != SPIRV_MAGIC {
        return Err(anyhow::Error::msg("not spirv"));
    }

    Ok(())
}

fn parse_opcode(opcode_word: u32) -> (u16, usize) {
    const LOW_BITS_MASK: u32 = 0x0000ffff;
    const HIGH_BITS_MASK: u32 = 0xffff0000;

    let op_length_words = opcode_word >> 16;
    let op_code = opcode_word & LOW_BITS_MASK;

    return (op_code as u16, op_length_words as usize);
}

pub fn extract_result_id(buf: &OpCodeUnparsed) -> u32 {
    let opcodes_with_result: HashMap<OpCodes, usize> = HashMap::from([
        // TODO: incomplete, more opcodes can have ids
        (OpCodes::OpVariable, 2),
        (OpCodes::OpTypePointer, 1),
        (OpCodes::OpTypeStruct, 1),
        // (OpCodes::OpTypeRuntimeArray, 1),
        (OpCodes::OpTypeArray, 1),
        (OpCodes::OpTypeSampledImage, 1),
        (OpCodes::OpTypeSampler, 1),
        (OpCodes::OpTypeImage, 1),
        (OpCodes::OpTypeMatrix, 1),
        (OpCodes::OpTypeVector, 1),
        (OpCodes::OpTypeFloat, 1),
        (OpCodes::OpTypeInt, 1),
        (OpCodes::OpTypeBool, 1),
        (OpCodes::OpTypeVoid, 1),
    ]);

    let opcode_enum = num::FromPrimitive::from_u16(buf.opcode).unwrap_or(OpCodes::Unknown);
    let result = opcodes_with_result.get(&opcode_enum);

    if let Some(found) = result {
        return buf.data[*found];
    } else {
        return 0;
    }
}

pub fn split_by_opcodes(buf: &Vec<u32>) -> (Vec<OpCodeUnparsed>, OpcodesById) {
    let mut opcodes_tokenized: Vec<OpCodeUnparsed> = Vec::new();
    let mut opcodes_by_id: OpcodesById = HashMap::new();

    // skip the rest of the file header
    let mut current_index: usize = 5;
    let buffer_length = buf.len().try_into().unwrap();

    let mut skip = 0;

    let mut current_opcode_unparsed = OpCodeUnparsed::default();

    while current_index < buffer_length {
        if skip == 0 {
            // get new opcode
            let (opcode, opcode_length) = parse_opcode(buf[current_index]);
            // println!("[{op_shader_index}] {opcode}, {opcode_length}");

            current_opcode_unparsed.data.push(buf[current_index]);
            current_opcode_unparsed.opcode = opcode;
            current_opcode_unparsed.length = opcode_length;
            skip = opcode_length - 1;
        } else {
            // just copy data
            current_opcode_unparsed.data.push(buf[current_index]);
            skip -= 1;

            if skip == 0 {
                current_opcode_unparsed.result_id = extract_result_id(&current_opcode_unparsed);
                opcodes_tokenized.push(current_opcode_unparsed.clone());
                if current_opcode_unparsed.result_id != 0 {
                    opcodes_by_id.insert(current_opcode_unparsed.result_id, current_opcode_unparsed.clone());
                }
                current_opcode_unparsed = OpCodeUnparsed::default();
            }
        }

        current_index += 1;
    }
    return (opcodes_tokenized, opcodes_by_id);
}
