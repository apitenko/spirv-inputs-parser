use std::{
    collections::HashMap,
    error::Error,
    fmt::format,
    fs::File,
    io::{BufReader, Read},
};

use byteorder::ByteOrder;
use byteorder::LittleEndian;

mod parsers;
mod type_parsers;
mod types;
mod util;

use parsers::*;
use types::OpCodes;

use crate::types::OpEntryPoint;

fn main() -> anyhow::Result<()> {
    println!("---------------------!");
    let file_path = "shader.bin";

    let f = File::open(file_path).unwrap();
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    let mut buf = Vec::<u8>::with_capacity(2420);
    let len = reader.read_to_end(&mut buf).unwrap();

    let vec32 = vec_u8_to_u32(&buf);
    parse_spirv_inputs(&vec32)?;

    Ok(())
}

pub fn vec_u8_to_u32(vec8: &Vec<u8>) -> Vec<u32> {
    let mut vec32: Vec<u32> = vec![0; vec8.len() / 4];
    LittleEndian::read_u32_into(&vec8, &mut vec32);
    return vec32;
}

pub fn parse_opcode(opcode_word: u32) -> (u16, usize) {
    const LOW_BITS_MASK: u32 = 0x0000ffff;
    const HIGH_BITS_MASK: u32 = 0xffff0000;

    let op_length_words = opcode_word >> 16;
    let op_code = opcode_word & LOW_BITS_MASK;

    return (op_code as u16, op_length_words as usize);
}

pub fn debug_hex_output(buf: &Vec<u32>) {
    for op in buf {
        let hex = format!("{:x}", *op);
        let letters: [u8; 4] = unsafe { std::mem::transmute(op.to_le()) };
        let ascii = letters.map(|v| v as char);
        print!(
            "0x{} - {}{}{}{}\n",
            hex, ascii[0], ascii[1], ascii[2], ascii[3],
        );
    }
}

pub fn check_magic(buf: &Vec<u32>) -> anyhow::Result<()> {
    const SPIRV_MAGIC: u32 = 0x07230203;

    if buf[0] != SPIRV_MAGIC {
        return Err(anyhow::Error::msg("not spirv"));
    }

    Ok(())
}

pub fn extract_result_id(buf: &OpCodeUnparsed) -> u32 {
    let opcodes_with_result: HashMap<OpCodes, usize> = HashMap::from([
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

pub fn split_by_opcodes(buf: &Vec<u32>) -> Vec<OpCodeUnparsed> {
    let mut by_opcodes: Vec<OpCodeUnparsed> = Vec::new();

    // skip the rest of the file header
    let mut current_index: usize = 5;
    let buffer_length = buf.len().try_into().unwrap();
    // let mut op_shader_index = 0;

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

            // current_index += opcode_length;
            skip = opcode_length - 1;
            // op_shader_index += 1;
        } else {
            // just copy data
            current_opcode_unparsed.data.push(buf[current_index]);
            skip -= 1;

            if skip == 0 {
                current_opcode_unparsed.result_id = extract_result_id(&current_opcode_unparsed);
                by_opcodes.push(current_opcode_unparsed.clone());
                current_opcode_unparsed = OpCodeUnparsed::default();
            }
        }

        current_index += 1;
    }
    return by_opcodes;
}

fn extract_entry_point(by_opcodes: &Vec<OpCodeUnparsed>) -> anyhow::Result<OpEntryPoint> {
    // find entry point
    let found = by_opcodes
        .into_iter()
        .find(|v| v.opcode == (OpCodes::OpEntryPoint as u16));

    let found = match found {
        Some(d) => d,
        None => return Err(anyhow::Error::msg("No entry point found")),
    };

    let entry_parsed = <OpCodeUnparsed as ParseOpCode<OpEntryPoint>>::parse_into(found, by_opcodes);

    println!("{:?}", entry_parsed);

    Ok(entry_parsed)
}

fn parse_spirv_inputs(buf: &Vec<u32>) -> anyhow::Result<()> {
    check_magic(buf)?;
    let by_opcodes = split_by_opcodes(buf);
    let entry = extract_entry_point(&by_opcodes);
    // println!("buf {:?}", &by_opcodes[0..6]);

    return Ok(());
}
