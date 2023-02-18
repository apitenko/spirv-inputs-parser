use byteorder::{LittleEndian, ByteOrder};

use crate::parsers::{OpCodeUnparsed, ParseOpCode};

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

pub fn parse_opcode<T>(opcode: &OpCodeUnparsed, by_opcodes: &Vec<OpCodeUnparsed>) -> T
where
    OpCodeUnparsed: ParseOpCode<T>,
{
    let parsed: T = <OpCodeUnparsed as ParseOpCode<T>>::parse_into(opcode, by_opcodes);
    parsed
}

pub fn lookup_and_parse<T>(result_id: u32, by_opcodes: &Vec<OpCodeUnparsed>) -> T
where
    OpCodeUnparsed: ParseOpCode<T>,
{
    let opcode = by_opcodes.result_id_lookup(result_id).unwrap();
    parse_opcode(opcode, by_opcodes)    
}

pub fn parse_string(slice: &[u32]) -> (String, usize) {
    // find the end (\0)
    let mut temp_data = Vec::<u32>::new();

    for word in slice {
        temp_data.push(*word);
        if *word == 0 {
            break;
        }
    }

    let data = vec_u32_to_u8(&temp_data);
    let output_string = std::str::from_utf8(&data).unwrap();
    return (output_string.to_string(), temp_data.len());
}

pub fn vec_u8_to_u32(vec8: &Vec<u8>) -> Vec<u32> {
    let mut vec32: Vec<u32> = vec![0; vec8.len() / 4];
    LittleEndian::read_u32_into(&vec8, &mut vec32);
    return vec32;
}

pub fn vec_u32_to_u8(vec32: &Vec<u32>) -> Vec<u8> {
    let mut vec8: Vec<u8> = vec![0; vec32.len() * 4];
    byteorder::LittleEndian::write_u32_into(&vec32, &mut vec8);
    return vec8;
}