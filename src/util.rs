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

    let temp_data_ptr = temp_data.as_ptr() as *const u8;
    let temp_data_len = temp_data.len() * 4;

    let mut data = vec![0; temp_data_len];

    unsafe {
        std::ptr::copy_nonoverlapping(temp_data_ptr, data.as_mut_ptr(), temp_data_len);
    }

    let output_string = std::str::from_utf8(&data).unwrap();

    return (output_string.to_string(), temp_data.len());
}
