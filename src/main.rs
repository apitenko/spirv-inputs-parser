use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
};

mod output;
mod parsers;
mod types;
mod util;

use output::ShaderMetaInfo;
use parsers::{unparsed::*, variables::*};
use types::{unparsed::*, variables::*};

use crate::util::vec_u8_to_u32;

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

// struct Annotation {

// }

// fn parse_annotations(opcodes: &Vec<OpCodeUnparsed>) {
//     // let output: HashMap<u32> = HashMap::new();

//     for opcode in by_opcodes {
//         match OpCodes::from(opcode.opcode) {
//             OpCodes::OpDecorate => {}
//             OpCodes::OpMemberDecorate => {}
//             _ => (),
//         }
//     }
// }

pub fn parse_spirv_inputs(buf: &Vec<u32>) -> anyhow::Result<ShaderMetaInfo> {
    check_magic(buf)?;
    let (opcodes, opcodes_by_id) = split_by_opcodes(buf);

    
    
    
    // let entry = extract_entry_point(&by_opcodes)?;
    // println!("buf {:?}", &by_opcodes[0..6]);

    // let output = prepare_output(&entry);

    // println!("{:?}", output);
    // Ok(output)

    Ok(())
}
