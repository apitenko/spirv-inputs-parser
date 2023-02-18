use std::rc::Rc;

use num_derive::FromPrimitive;

use crate::output::{StorageClass, ExecutionModel};


pub trait IsOpcode {
    fn opcode() -> OpCodes;
}

#[derive(Debug)]
pub struct OpTypePointer {
    pub result_id: u32,
    pub storage_class: StorageClass,
    pub type_id: OpTypeWrapper,
}

impl IsOpcode for OpTypePointer {
    fn opcode() -> OpCodes {
        OpCodes::OpTypePointer
    }
}

#[derive(Debug)]
pub struct OpVariable {
    pub result_type: OpTypePointer,
    pub result_id: u32,
    pub storage_class: StorageClass,
    // pub initializer_id: Option<u32>, // idk idc
}

impl IsOpcode for OpVariable {
    fn opcode() -> OpCodes {
        OpCodes::OpVariable
    }
}

#[derive(Debug)]
pub struct OpEntryPoint {
    pub execution_model: ExecutionModel,
    pub entry_point_result_id_op_function: u32, // doesn't matter
    pub name: String,
    pub interfaces: Vec<OpVariable>,
}

impl IsOpcode for OpEntryPoint {
    fn opcode() -> OpCodes {
        OpCodes::OpEntryPoint
    }
}

#[derive(FromPrimitive, Debug, PartialEq, Eq, Hash)]
pub enum OpCodes {
    Unknown = 0,
    OpEntryPoint = 15,
    OpTypeVoid = 19,
    OpTypeBool = 20,
    OpTypeInt = 21,
    OpTypeFloat = 22,
    OpTypeVector = 23,
    OpTypeMatrix = 24,
    OpTypeImage = 25,
    OpTypeSampler = 26,
    OpTypeSampledImage = 27,
    OpTypeArray = 28,
    // OpTypeRuntimeArray = 29,
    OpTypeStruct = 30,
    OpTypePointer = 32,
    OpVariable = 59,
}

impl From<u16> for OpCodes {
    fn from(value: u16) -> Self {
        num::FromPrimitive::from_u16(value).unwrap_or(OpCodes::Unknown)
    }
}

#[derive(Debug)]
pub struct OpTypeVoid {
    pub result_id: u32,
}
impl IsOpcode for OpTypeVoid {
    fn opcode() -> OpCodes {
        return OpCodes::OpTypeVoid;
    }
}

#[derive(Debug)]
pub struct OpTypeBool {
    pub result_id: u32,
}
impl IsOpcode for OpTypeBool {
    fn opcode() -> OpCodes {
        return OpCodes::OpTypeBool;
    }
}

#[derive(FromPrimitive, Debug)]
pub enum Signedness {
    Unsigned = 0,
    Signed = 1,
}

impl From<u32> for Signedness {
    fn from(value: u32) -> Self {
        return num::FromPrimitive::from_u32(value).unwrap();
    }
}

#[derive(Debug)]
pub struct OpTypeInt {
    pub result_id: u32,
    pub width: u32,
    pub signedness: Signedness,
}
impl IsOpcode for OpTypeInt {
    fn opcode() -> OpCodes {
        return OpCodes::OpTypeInt;
    }
}

#[derive(Debug)]
pub struct OpTypeFloat {
    pub result_id: u32,
    pub width: u32,
}
impl IsOpcode for OpTypeFloat {
    fn opcode() -> OpCodes {
        return OpCodes::OpTypeFloat;
    }
}

#[derive(Debug)]
pub struct OpTypeMatrix {
    pub result_id: u32,
    pub column_type: OpTypeWrapper,
    pub column_count: u32,
}

impl IsOpcode for OpTypeMatrix {
    fn opcode() -> OpCodes {
        return OpCodes::OpTypeMatrix;
    }
}

#[derive(Debug)]
pub struct OpTypeVector {
    pub result_id: u32,
    pub element_type: OpTypeWrapper,
    pub length: u32,
}

impl IsOpcode for OpTypeVector {
    fn opcode() -> OpCodes {
        return OpCodes::OpTypeVector;
    }
}

#[derive(Debug)]
pub struct OpTypeArray {
    pub result_id: u32,
    pub element_type: OpTypeWrapper,
    pub length: u32,
}

impl IsOpcode for OpTypeArray {
    fn opcode() -> OpCodes {
        return OpCodes::OpTypeArray;
    }
}

#[derive(Debug)]
pub enum OpTypeWrapper {
    OpTypeStruct(Box<OpTypeStruct>),
    OpTypeArray(Box<OpTypeArray>),
    OpTypeMatrix(Box<OpTypeMatrix>),
    OpTypeVector(Box<OpTypeVector>),
    OpTypeFloat(Box<OpTypeFloat>),
    OpTypeInt(Box<OpTypeInt>),
    OpTypeBool(Box<OpTypeBool>),
    OpTypeVoid(Box<OpTypeVoid>),
}

#[derive(Debug)]
pub struct OpTypeStruct {
    pub result_id: u32,
    pub members: Vec<OpTypeWrapper>,
}

impl IsOpcode for OpTypeStruct {
    fn opcode() -> OpCodes {
        return OpCodes::OpTypeStruct;
    }
}
