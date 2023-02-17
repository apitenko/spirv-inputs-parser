use num_derive::FromPrimitive;


#[derive(FromPrimitive, Debug)]
pub enum ExecutionModel {
    Vertex = 0,
    TessellationControl = 1,
    TessellationEvaluation = 2,
    Geometry = 3,
    Fragment = 4,
    GLCompute = 5,
    Kernel = 6,
}

#[derive(FromPrimitive, Debug)]
pub enum StorageClass {
    UniformConstant = 0,
    Input = 1,
    Uniform = 2,
    Output = 3,
    Workgroup = 4,
    CrossWorkgroup = 5,
    Private = 6,
    Function = 7,
    Generic = 8,
    PushConstant = 9,
    AtomicCounter = 10,
    Image = 11,
    StorageBuffer = 12,
}

pub trait IsOpcode {
    fn opcode() -> OpCodes;
}

#[derive(Debug)]
pub struct OpTypePointer {
    pub result_id: u32,
    pub storage_class: StorageClass,
    pub type_id: u32,
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
    OpTypeRuntimeArray = 29,
    OpTypeStruct = 30,
    OpTypePointer = 32,
    OpVariable = 59,
}
