use num_derive::FromPrimitive;

use super::variables::{IsOpcode, OpCodes};

#[derive(FromPrimitive, Debug, PartialEq, Eq, Hash)]
pub enum Builtin {
    Position = 0,
    PointSize = 1,
    ClipDistance = 3,
    CullDistance = 4,
    VertexId = 5,
    InstanceId = 6,
    PrimitiveId = 7,
    InvocationId = 8,
    Layer = 9,
    ViewportIndex = 10,
    TessLevelOuter = 11,
    TessLevelInner = 12,
    TessCoord = 13,
    PatchVertices = 14,
    FragCoord = 15,
    PointCoord = 16,
    FrontFacing = 17,
    SampleId = 18,
    SamplePosition = 19,
    SampleMask = 20,
    FragDepth = 22,
    HelperInvocation = 23,
    NumWorkgroups = 24,
    WorkgroupSize = 25,
    WorkgroupId = 26,
    LocalInvocationId = 27,
    GlobalInvocationId = 28,
    LocalInvocationIndex = 29,
    WorkDim = 30,
    GlobalSize = 31,
    EnqueuedWorkgroupSize = 32,
    GlobalOffset = 33,
    GlobalLinearId = 34,
    SubgroupSize = 36,
    SubgroupMaxSize = 37,
    NumSubgroups = 38,
    NumEnqueuedSubgroups = 39,
    SubgroupId = 40,
    SubgroupLocalInvocationId = 41,
    VertexIndex = 42,
    InstanceIndex = 43,
    SubgroupEqMaskKHR = 4416,
    SubgroupGeMaskKHR = 4417,
    SubgroupGtMaskKHR = 4418,
    SubgroupLeMaskKHR = 4419,
    SubgroupLtMaskKHR = 4420,
    BaseVertex = 4424,
    BaseInstance = 4425,
    DrawIndex = 4426,
    DeviceIndex = 4438,
    ViewIndex = 4440,
    BaryCoordNoPerspAMD = 4992,
    BaryCoordNoPerspCentroidAMD = 4993,
    BaryCoordNoPerspSampleAMD = 4994,
    BaryCoordSmoothAMD = 4995,
    BaryCoordSmoothCentroidAMD = 4996,
    BaryCoordSmoothSampleAMD = 4997,
    BaryCoordPullModelAMD = 4998,
    FragStencilRefEXT = 5014,
    ViewportMaskNV = 5253,
    SecondaryPositionNV = 5257,
    SecondaryViewportMaskNV = 5258,
    PositionPerViewNV = 5261,
    ViewportMaskPerViewNV = 5262,
    FullyCoveredEXT = 5264,
}

#[derive(FromPrimitive, Debug, PartialEq, Eq, Hash)]
pub enum Decoration {
    RelaxedPrecision = 0,
    SpecId = 1,
    Block = 2,
    BufferBlock = 3,
    RowMajor = 4,
    ColMajor = 5,
    ArrayStride = 6,
    MatrixStride = 7,
    GLSLShared = 8,
    GLSLPacked = 9,
    CPacked = 10,
    BuiltIn = 11,
    NoPerspective = 13,
    Flat = 14,
    Patch = 15,
    Centroid = 16,
    Sample = 17,
    Invariant = 18,
    Restrict = 19,
    Aliased = 20,
    Volatile = 21,
    Constant = 22,
    Coherent = 23,
    NonWritable = 24,
    NonReadable = 25,
    Uniform = 26,
    SaturatedConversion = 28,
    Stream = 29,
    Location = 30,
    Component = 31,
    Index = 32,
    Binding = 33,
    DescriptorSet = 34,
    Offset = 35,
    XfbBuffer = 36,
    XfbStride = 37,
    FuncParamAttr = 38,
    FPRoundingMode = 39,
    FPFastMathMode = 40,
    LinkageAttributes = 41,
    NoContraction = 42,
    InputAttachmentIndex = 43,
    Alignment = 44,
    ExplicitInterpAMD = 4999,
    OverrideCoverageNV = 5248,
    PassthroughNV = 5250,
    ViewportRelativeNV = 5252,
    SecondaryViewportRelativeNV = 5256,
}

pub struct DecorationDataLocation {
    location: u32, // TODO: LiteralNumber
}
pub struct DecorationDataIndex {
    index: u32, // TODO: LiteralNumber
}
pub struct DecorationDataBinding {
    binding_point: u32, // TODO: LiteralNumber
}
pub struct DecorationDataDescriptorSet {
    descriptor_set: u32, // TODO: LiteralNumber
}
pub struct DecorationDataOffset {
    offset: u32, // TODO: LiteralNumber
}

pub enum DecorationData {
    None,
    SpecId(u32),       // TODO
    ArrayStride(u32),  // TODO
    MatrixStride(u32), // TODO
    Stream(u32),       // TODO
    Location(DecorationDataLocation),
    Component(u32), // TODO
    Index(DecorationDataIndex),
    Binding(DecorationDataBinding),
    DescriptorSet(DecorationDataDescriptorSet),
    Offset(DecorationDataOffset),
    XfbBuffer(u32),                   // TODO
    XfbStride(u32),                   // TODO
    FuncParamAttr(u32),               // TODO
    FPRoundingMode(u32),              // TODO
    FPFastMathMode(u32),              // TODO
    LinkageAttributes(u32),           // TODO
    InputAttachmentIndex(u32),        // TODO
    Alignment(u32),                   // TODO
    SecondaryViewportRelativeNV(u32), // TODO
}

pub struct OpDecorate {
    pub target_id: u32,
    pub decoration: Decoration,
    pub decoration_data: DecorationData,
}

impl IsOpcode for OpDecorate {
    fn opcode() -> OpCodes {
        OpCodes::OpDecorate
    }
}

pub struct OpMemberDecorate {
    pub target_id: u32, // OpTypeStruct id
    pub member: u32,
    pub decoration: Decoration,
    pub decoration_data: DecorationData,
}

impl IsOpcode for OpMemberDecorate {
    fn opcode() -> OpCodes {
        OpCodes::OpMemberDecorate
    }
}