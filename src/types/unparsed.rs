use crate::parsers::unparsed::OpcodesById;

pub trait ParseOpCode<T> {
    fn parse_into(opcode: &OpCodeUnparsed, opcodes_by_id: &OpcodesById) -> T;
}

#[derive(Default, Debug, Clone)]
pub struct OpCodeUnparsed {
    pub opcode: u16,
    pub length: usize,
    pub result_id: u32, // 0 means None and is default
    pub data: Vec<u32>,
}
