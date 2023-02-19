// TODO: design the API better

use num_derive::FromPrimitive;

use crate::types::variables::{OpTypeWrapper, OpEntryPoint};

#[derive(FromPrimitive, Debug, Clone, Copy)]
pub enum ExecutionModel {
    Vertex = 0,
    TessellationControl = 1,
    TessellationEvaluation = 2,
    Geometry = 3,
    Fragment = 4,
    GLCompute = 5,
    Kernel = 6,
}

#[derive(FromPrimitive, Debug, Clone, Copy)]
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

#[derive(Debug)]
pub enum DataType {
    Mat4,
    Float,
    Int,
    Bool,
    Vec3,
    Struct(Vec<DataType>),
}

#[derive(Debug)]
pub struct Interface {
    pub interface: DataType,
}

#[derive(Debug)]
pub struct ShaderMetaInfo {
    pub execution_model: ExecutionModel,
    pub interfaces: Vec<(StorageClass, Interface)>,
}

// TODO: support more complex types for inputs/outputs as the engine development progresses
fn reduce_type(type_wrapper: &OpTypeWrapper) -> Option<Interface> {
    // reduce the tree-like structure into a GLSL-like type string

    match type_wrapper {
        OpTypeWrapper::OpTypeStruct(data) => {
            return None;
        }
        OpTypeWrapper::OpTypeArray(data) => match &data.element_type {
            // ????
            OpTypeWrapper::OpTypeFloat(d) => {
                if d.width == 3 {
                    return Some(Interface {
                        interface: DataType::Vec3,
                    });
                } else {
                    return None;
                }
            }
            _ => return None,
        },
        OpTypeWrapper::OpTypeMatrix(data) => match &data.column_type {
            OpTypeWrapper::OpTypeVector(v) => {
                if v.length == 3 {
                    return Some(Interface {
                        interface: DataType::Mat4,
                    });
                } else {
                    return None;
                }
            }
            _ => return None,
        },
        OpTypeWrapper::OpTypeVector(data) => {
            if data.length != 3 {
                return None;
            }

            match &data.element_type {
                OpTypeWrapper::OpTypeFloat(d) => {
                    if d.width == 32 {
                        // ??????
                        return Some(Interface {
                            interface: DataType::Vec3,
                        });
                    } else {
                        return None;
                    }
                }
                _ => return None,
            }
        }
        OpTypeWrapper::OpTypeFloat(data) => {
            return Some(Interface {
                interface: DataType::Float,
            });
        }
        OpTypeWrapper::OpTypeInt(data) => {
            return Some(Interface {
                interface: DataType::Int,
            });
        }
        OpTypeWrapper::OpTypeBool(data) => {
            return Some(Interface {
                interface: DataType::Bool,
            });
        }
        OpTypeWrapper::OpTypeVoid(data) => {
            return None;
        }
    }
}

pub fn prepare_output(entry_point: &OpEntryPoint) -> ShaderMetaInfo {
    let mut interfaces = Vec::new();

    for interface_raw in &entry_point.interfaces {
        if let Some(interface) = reduce_type(&interface_raw.result_type.type_id) {
            interfaces.push((interface_raw.storage_class, interface));
        } else {
            continue;
        }
    }

    ShaderMetaInfo {
        execution_model: entry_point.execution_model,
        interfaces,
    }
}
