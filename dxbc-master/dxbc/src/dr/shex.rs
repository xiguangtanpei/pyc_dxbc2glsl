use d3d11tokenizedprogramformat::*;

use binary::*;

use std::slice;
use std::mem;
use std::marker::PhantomData;

#[repr(u32)]
#[derive(Debug)]
pub enum ConstantBufferIndexPattern {
    Immediate = 0,
    Dynamic = 1,
}

#[repr(u32)]
#[derive(Debug)]
pub enum OperandType {
    Temp = 0,
    Input = 1,
    Output= 2,
    IndexableTemp = 3,
    Immediate32 = 4,
    Immediate64 = 5,
    Sampler = 6,
    Resource = 7,
    ConstantBuffer = 8,
    ImmediateConstantBuffer = 9,
    Label = 10,
    InputPrimitiveId = 11,
    OutputDepth = 12,
    Null = 13,
    Rasterizer = 14,
    OutputCoverageMask = 15,
    Stream = 16,
    FunctionBody = 17,
    FunctionTable = 18,
    Interface = 19,
    FunctionInput = 20,
    FunctionOutput = 21,
    OutputControlPointId = 22,
    InputForkInstanceId = 23,
    InputJoinInstanceId = 24,
    InputControlPoint = 25,
    OutputControlPoint = 26,
    InputPatchConstant = 27,
    InputDomainPoint = 28,
    ThisPointer = 29,
    UnorderedAccessView = 30,
    ThreadGroupSharedMemory = 31,
    InputThreadId = 32,
    InputThreadGroupId = 33,
    InputThreadIdInGroup = 34,
    InputCoverageMask = 35,
    InputThreadIdInGroupFlattened = 36,
    InputGsinstanceid = 37,
    OutputDepthGreaterEqual = 38,
    OutputDepthLessEqual = 39,
    CycleCounter = 40,
}

#[repr(u32)]
#[derive(Debug)]
pub enum NumComponents {
    Zero = 0,
    One = 1,
    Four = 2,
    N = 3,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum ComponentName {
    X = 0,
    Y = 1,
    Z = 2,
    W = 3,
}

impl ComponentName {
    pub fn from_word(word: u8) -> ComponentName {
        match word {
            0 => ComponentName::X,
            1 => ComponentName::Y,
            2 => ComponentName::Z,
            3 => ComponentName::W,
            _ => unreachable!()
        }
    }
}

#[derive(Debug)]
pub struct ComponentSwizzle(pub ComponentName, pub ComponentName, pub ComponentName, pub ComponentName);

#[repr(u32)]
#[derive(Debug)]
pub enum IndexDimension {
    D0 = 0,
    D1 = 1,
    D2 = 2,
    D3 = 3,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum IndexRepresentation {
    Immediate32 = 0,
    Immediate64 = 1,
    Relative = 2,
    Immediate32PlusRelative = 3,
    Immediate64PlusRelative = 4,
}

#[derive(Debug)]
pub enum Immediate<'a> {
    U32(u32),
    U64(u64),
    Relative(OperandToken0<'a>),
    U32Relative(u32, OperandToken0<'a>),
    U64Relative(u64, OperandToken0<'a>),
}

#[repr(u32)]
#[derive(Debug)]
pub enum ComponentSelectMode {
    Mask = 0,
    Swizzle = 1,
    Select1 = 2,
}

bitflags! {
    pub struct ComponentMask: u32 {
        const COMPONENT_MASK_R = D3D10_SB_OPERAND_4_COMPONENT_MASK_R;
        const COMPONENT_MASK_G = D3D10_SB_OPERAND_4_COMPONENT_MASK_G;
        const COMPONENT_MASK_B = D3D10_SB_OPERAND_4_COMPONENT_MASK_B;
        const COMPONENT_MASK_A = D3D10_SB_OPERAND_4_COMPONENT_MASK_A;
    }
}

#[repr(u32)]
#[derive(Debug)]
pub enum NameToken {
    Undefined = 0,
    Position = 1,
    ClipDistance = 2,
    CullDistance = 3,
    RenderTargetArrayIndex = 4,
    ViewportArrayIndex = 5,
    VertexId = 6,
    PrimitiveId = 7,
    InstanceId = 8,
    IsFrontFace = 9,
    SampleIndex = 10,
    FinalQuadUEq0EdgeTessfactor = 11,
    FinalQuadVEq0EdgeTessfactor = 12,
    FinalQuadUEq1EdgeTessfactor = 13,
    FinalQuadVEq1EdgeTessfactor = 14,
    FinalQuadUInsideTessfactor = 15,
    FinalQuadVInsideTessfactor = 16,
    FinalTriUEq0EdgeTessfactor = 17,
    FinalTriVEq0EdgeTessfactor = 18,
    FinalTriWEq0EdgeTessfactor = 19,
    FinalTriinsidetessfactor = 20,
    FinalLineDetailTessfactor = 21,
    FinalLineDensityTessfactor = 22,
}

impl NameToken {
    pub fn from_word(word: u32) -> Self {
        match word {
            0 => NameToken::Undefined,
            1 => NameToken::Position,
            2 => NameToken::ClipDistance,
            3 => NameToken::CullDistance,
            4 => NameToken::RenderTargetArrayIndex,
            5 => NameToken::ViewportArrayIndex,
            6 => NameToken::VertexId,
            7 => NameToken::PrimitiveId,
            8 => NameToken::InstanceId,
            9 => NameToken::IsFrontFace,
            10 => NameToken::SampleIndex,
            11 => NameToken::FinalQuadUEq0EdgeTessfactor,
            12 => NameToken::FinalQuadVEq0EdgeTessfactor,
            13 => NameToken::FinalQuadUEq1EdgeTessfactor,
            14 => NameToken::FinalQuadVEq1EdgeTessfactor,
            15 => NameToken::FinalQuadUInsideTessfactor,
            16 => NameToken::FinalQuadVInsideTessfactor,
            17 => NameToken::FinalTriUEq0EdgeTessfactor,
            18 => NameToken::FinalTriVEq0EdgeTessfactor,
            19 => NameToken::FinalTriWEq0EdgeTessfactor,
            20 => NameToken::FinalTriinsidetessfactor,
            21 => NameToken::FinalLineDetailTessfactor,
            22 => NameToken::FinalLineDensityTessfactor,
            _ => unimplemented!(),
        }
    }
}

#[repr(u32)]
#[derive(Debug)]
pub enum OperandModifier {
    None,
    Neg,
    Abs,
    AbsNeg,
}

impl OperandModifier {
    pub fn from_word(word: u32) -> Self {
        match DECODE_D3D10_SB_OPERAND_MODIFIER(word) {
            0 => OperandModifier::None,
            1 => OperandModifier::Neg,
            2 => OperandModifier::Abs,
            3 => OperandModifier::AbsNeg,
            _ => unreachable!(),
        }
    }
}

#[repr(u32)]
#[derive(Debug)]
pub enum SamplerMode {
    Default,
    Comparison,
    Mono,
}

impl SamplerMode {
    pub fn from_word(word: u32) -> Self {
        match word {
            0 => SamplerMode::Default,
            1 => SamplerMode::Comparison,
            2 => SamplerMode::Mono,
            _ => unreachable!(),
        }
    }
}

#[repr(u32)]
#[derive(Debug)]
pub enum TestBoolean {
    Zero,
    NonZero,
}

impl TestBoolean {
    pub fn from_word(word: u32) -> Self {
        match DECODE_D3D10_SB_INSTRUCTION_TEST_BOOLEAN(word) {
            0 => TestBoolean::Zero,
            1 => TestBoolean::NonZero,
            _ => unreachable!(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OpcodeToken0<'a> {
    // TODO: these should probably be slices
    pub word: *const u32,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> OpcodeToken0<'a> {
    pub fn from_word(word: *const u32) -> Self {
        OpcodeToken0 {
            word,
            _phantom: PhantomData,
        }
    }

    pub fn is_extended(&self) -> bool {
        DECODE_IS_D3D10_SB_OPCODE_EXTENDED(unsafe { *self.word }) != 0
    }

    pub fn is_saturated(&self) -> bool {
        DECODE_IS_D3D10_SB_INSTRUCTION_SATURATE_ENABLED(unsafe { *self.word }) != 0
    }

    pub fn get_test_type(&self) -> TestBoolean {
        TestBoolean::from_word(unsafe { *self.word })
    }

    pub fn get_extended_opcode(&self) -> Option<OpcodeToken1<'a>> {
        if self.is_extended() {
            Some(OpcodeToken1 {
                word: unsafe { self.word.offset(1) },
                _phantom: self._phantom,
            })
        } else {
            None
        }
    }

    pub fn get_opcode_type(&self) -> u32 {
        DECODE_D3D10_SB_OPCODE_TYPE(unsafe { *self.word })
    }

    pub fn get_instruction_length(&self) -> u32 {
        DECODE_D3D10_SB_TOKENIZED_INSTRUCTION_LENGTH(unsafe { *self.word })
    }

    pub fn get_resource_dimension(&self) -> ResourceDimension {
        ResourceDimension::from_word(DECODE_D3D10_SB_RESOURCE_DIMENSION(unsafe { *(self.word.offset(0)) }))
    }

    pub fn get_sampler_mode(&self) -> SamplerMode {
        SamplerMode::from_word(DECODE_D3D10_SB_SAMPLER_MODE(unsafe { *(self.word.offset(0)) }))
    }

    pub fn get_interpolation_mode(&self) -> InterpolationMode {
        InterpolationMode::from_word(DECODE_D3D10_SB_INPUT_INTERPOLATION_MODE(unsafe { *self.word }))
    }
}

impl<'a> fmt::Debug for OpcodeToken0<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("OpcodeToken0")
            .field("Raw", &unsafe { *self.word })
            .field("Type", &self.get_opcode_type())
            .field("InstructionLength", &self.get_instruction_length())
            .field("IsSaturated", &self.is_saturated())
            .field("TestType", &self.get_test_type())
            .field("IsExtended", &self.is_extended())
            .finish()
    }
}

#[repr(u32)]
#[derive(Debug)]
pub enum InterpolationMode {
    Undefined = 0,
    Constant = 1,
    Linear = 2,
    LinearCentroid = 3,
    LinearNoPerspective = 4,
    LinearNoPerspectiveCentroid = 5,
    LinearSample = 6,
    LinearNoPerspectiveSample = 7,
}

impl InterpolationMode {
    pub fn from_word(word: u32) -> Self {
        assert!(word <= InterpolationMode::LinearNoPerspectiveSample as u32);

        unsafe { mem::transmute(word) }
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum ResourceDimension {
    Unknown = 0,
    Buffer = 1,
    Texture1D = 2,
    Texture2D = 3,
    Texture2DMS = 4,
    Texture3D = 5,
    TextureCube = 6,
    Texture1DArray = 7,
    Texture2DArray = 8,
    Texture2DMSArray = 9,
    TextureCubeArray = 10,
    RawBuffer = 11,
    StructuredBuffer = 12
}

impl ResourceDimension {
    pub fn from_word(word: u32) -> Self {
        match word {
            0 => ResourceDimension::Unknown,
            1 => ResourceDimension::Buffer,
            2 => ResourceDimension::Texture1D,
            3 => ResourceDimension::Texture2D,
            4 => ResourceDimension::Texture2DMS,
            5 => ResourceDimension::Texture3D,
            6 => ResourceDimension::TextureCube,
            7 => ResourceDimension::Texture1DArray,
            8 => ResourceDimension::Texture2DArray,
            9 => ResourceDimension::Texture2DMSArray,
            10 => ResourceDimension::TextureCubeArray,
            11 => ResourceDimension::RawBuffer,
            12 => ResourceDimension::StructuredBuffer,
            _ => unreachable!()
        }
    }
}

#[repr(u32)]
#[derive(Debug)]
pub enum ExtendedOpcodeType {
    Empty = 0,
    SampleControls = 1,
    ResourceDim = 2,
    ResourceReturnType = 3,
}

impl ExtendedOpcodeType {
    pub fn from_word(word: u32) -> Self {
        match DECODE_D3D10_SB_EXTENDED_OPCODE_TYPE(word) {
            0 => ExtendedOpcodeType::Empty,
            1 => ExtendedOpcodeType::SampleControls,
            2 => ExtendedOpcodeType::ResourceDim,
            3 => ExtendedOpcodeType::ResourceReturnType,
            _ => unreachable!(),
        }
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum ResourceReturnType {
    Unorm = 1,
    Snorm = 2,
    Sint = 3,
    Uint = 4,
    Float = 5,
    Mixed = 6,
    Double = 7,
    Continued = 8,
    Unused = 9,
}

impl ResourceReturnType {
    pub fn from_word(word: u32) -> Self {
        match word {
            1 => ResourceReturnType::Unorm,
            2 => ResourceReturnType::Snorm,
            3 => ResourceReturnType::Sint,
            4 => ResourceReturnType::Uint,
            5 => ResourceReturnType::Float,
            6 => ResourceReturnType::Mixed,
            7 => ResourceReturnType::Double,
            8 => ResourceReturnType::Continued,
            9 => ResourceReturnType::Unused,
            _ => unreachable!(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ResourceReturnTypeToken0<'a> {
    pub word: *const u32,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> ResourceReturnTypeToken0<'a> {
    pub fn from_word(word: *const u32) -> Self {
        ResourceReturnTypeToken0 {
            word,
            _phantom: PhantomData,
        }
    }

    pub fn get_return_type(&self, name: ComponentName) -> ResourceReturnType {
        ResourceReturnType::from_word(DECODE_D3D10_SB_RESOURCE_RETURN_TYPE(unsafe { *(self.word.offset(0)) }, name as u32))
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OpcodeToken1<'a> {
    pub word: *const u32,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> OpcodeToken1<'a> {
    pub fn from_word(word: *const u32) -> Self {
        OpcodeToken1 {
            word,
            _phantom: PhantomData,
        }
    }

    pub fn get_extended_opcode_type(&self) -> ExtendedOpcodeType {
        ExtendedOpcodeType::from_word(DECODE_D3D10_SB_TOKENIZED_INSTRUCTION_LENGTH(unsafe { *self.word }))
    }

    pub fn get_opcode_modifier(&self) -> u32 {
        DECODE_D3D10_SB_TOKENIZED_INSTRUCTION_LENGTH(unsafe { *self.word })
    }


    pub fn is_extended(&self) -> bool {
        DECODE_IS_D3D10_SB_OPCODE_EXTENDED(unsafe { *self.word }) != 0
    }

    pub fn get_extended_opcode(&self) -> Option<OpcodeToken1<'a>> {
        if self.is_extended() {
            Some(OpcodeToken1 {
                word: unsafe { self.word.offset(1) },
                _phantom: self._phantom,
            })
        } else {
            None
        }
    }
}

impl<'a> fmt::Debug for OpcodeToken1<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("OpcodeToken1")
            .field("Type", &self.get_extended_opcode_type())
            .field("OpcodeModifier", &self.get_opcode_modifier())
            .field("IsExtended", &self.is_extended())
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OperandToken1<'a> {
    pub word: *const u32,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> OperandToken1<'a> {
    pub fn from_word(word: *const u32) -> Self {
        OperandToken1 {
            word,
            _phantom: PhantomData,
        }
    }

    pub fn get_operand_modifier(&self) -> OperandModifier {
        OperandModifier::from_word(unsafe { *self.word })
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OperandToken0<'a> {
    pub word: *const u32,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> OperandToken0<'a> {
    pub fn from_word(word: *const u32) -> Self {
        OperandToken0 {
            word,
            _phantom: PhantomData,
        }
    }

    pub fn parse<'b>(decoder: &mut decoder::Decoder<'b>) -> OperandToken0<'b> {
        let operand = OperandToken0::from_word(decoder.read_u32_address());

        if operand.is_extended() {
            decoder.skip(4);
        }

        let len = match operand.get_index_dimension() {
            IndexDimension::D0 => {
                let ty = operand.get_operand_type();

                match ty {
                    OperandType::Immediate32 | OperandType::Immediate64 => operand.get_num_components_u32(),
                    _ => 0
                }
            }
            IndexDimension::D1 => 1,
            IndexDimension::D2 => 2,
            IndexDimension::D3 => 3,
        };


        for i in 0..len {
            let repr = operand.get_index_representation(i);

            match repr {
                IndexRepresentation::Immediate32 => {
                    decoder.skip(4);
                },
                IndexRepresentation::Immediate64 => {
                    decoder.skip(8);
                },
                IndexRepresentation::Relative => {
                    let _ = OperandToken0::parse(decoder);
                }
                IndexRepresentation::Immediate32PlusRelative => {
                    decoder.skip(4);
                    let _ = OperandToken0::parse(decoder);
                },
                IndexRepresentation::Immediate64PlusRelative => {
                    decoder.skip(8);
                    let _ = OperandToken0::parse(decoder);
                },
            }
        }

        operand
    }

    pub fn len(&self) -> u32 {
        let mut len = 0;
        if self.is_extended() {
            len += 1;
        }

        let dim_len = match self.get_index_dimension() {
            IndexDimension::D0 => {
                let ty = self.get_operand_type();

                match ty {
                    OperandType::Immediate32 | OperandType::Immediate64 => self.get_num_components_u32(),
                    _ => 0
                }
            }
            IndexDimension::D1 => 1,
            IndexDimension::D2 => 2,
            IndexDimension::D3 => 3,
        };


        for i in 0..dim_len {
            let repr = self.get_index_representation(i);

            match repr {
                IndexRepresentation::Immediate32 => {
                    len += 1;
                },
                IndexRepresentation::Immediate64 => {
                    len += 2;
                },
                IndexRepresentation::Relative => {
                    len += OperandToken0::from_word(unsafe { self.word.offset(len as isize) }).len();
                }
                IndexRepresentation::Immediate32PlusRelative => {
                    len += 1;
                    len += OperandToken0::from_word(unsafe { self.word.offset(len as isize) }).len();
                },
                IndexRepresentation::Immediate64PlusRelative => {
                    len += 2;
                    len += OperandToken0::from_word(unsafe { self.word.offset(len as isize) }).len();
                },
            }
        }

        len
    }

    pub fn get_extended_operand(&self) -> Option<OperandToken1<'a>> {
        if self.is_extended() {
            Some(OperandToken1 {
                word: unsafe { self.word.offset(1) },
                _phantom: self._phantom,
            })
        } else {
            None
        }
    }

    fn get_immediate_offset(&self) -> *const u32 {
        if self.is_extended() {
            unsafe { self.word.offset(2) }
        } else {
            unsafe { self.word.offset(1) }
        }
    }

    pub fn get_immediates(&self) -> Vec<Immediate<'a>> {
        let mut immediates = Vec::new();

        let len = match self.get_index_dimension() {
            IndexDimension::D0 => {
                let ty = self.get_operand_type();

                match ty {
                    OperandType::Immediate32 | OperandType::Immediate64 => self.get_num_components_u32(),
                    _ => 0
                }
            },
            IndexDimension::D1 => 1,
            IndexDimension::D2 => 2,
            IndexDimension::D3 => 3,
        };

        for i in 0..len {
            immediates.push(self.get_immediate(i));
        }

        immediates
    }

    pub fn get_immediate(&self, index: u32) -> Immediate<'a> {
        let len = match self.get_index_dimension() {
            IndexDimension::D0 => {
                let ty = self.get_operand_type();

                match ty {
                    OperandType::Immediate32 | OperandType::Immediate64 => self.get_num_components_u32(),
                    _ => 0
                }
            },
            IndexDimension::D1 => 1,
            IndexDimension::D2 => 2,
            IndexDimension::D3 => 3,
        };

        let imm = self.get_immediate_offset();
        let mut offset = 0;

        use self::IndexRepresentation::*;

        for i in 0..len {
            let repr = self.get_index_representation(i);

            if i == index {
                match repr {
                    Immediate32 => {
                        return Immediate::U32(
                            unsafe { *imm.offset(offset as isize) }
                        );
                    },
                    Immediate64 => {
                        return Immediate::U64(
                            unsafe { *(imm.offset(offset as isize) as *const u64) }
                        );
                    },
                    Relative => {
                        return Immediate::Relative(
                            OperandToken0::from_word(unsafe { imm.offset(offset as isize) })
                        );
                    },
                    Immediate32PlusRelative => {
                        return Immediate::U32Relative(
                            unsafe { *imm.offset(offset as isize) },
                            OperandToken0::from_word(unsafe { imm.offset(1 + offset as isize) })
                        );
                    },
                    Immediate64PlusRelative => {
                        return Immediate::U64Relative(
                            unsafe { *(imm.offset(offset as isize) as *const u64) },
                            OperandToken0::from_word(unsafe { imm.offset(2 + offset as isize) })
                        );
                    },
                }
            } else {
                match repr {
                    Immediate32 => {
                        offset += 1;
                    },
                    Immediate64 => {
                        offset += 2;
                    },
                    Relative => {
                        offset += OperandToken0::from_word(unsafe { imm.offset(offset as isize) }).len();
                    },
                    Immediate32PlusRelative => {
                        offset += 1;
                        offset += OperandToken0::from_word(unsafe { imm.offset(offset as isize) }).len();
                    },
                    Immediate64PlusRelative => {
                        offset += 2;
                        offset += OperandToken0::from_word(unsafe { imm.offset(offset as isize) }).len();
                    },
                }
            }
        }

        unreachable!()
    }

    pub fn is_extended(&self) -> bool {
        DECODE_IS_D3D10_SB_OPERAND_EXTENDED(unsafe { *self.word }) != 0
    }

    pub fn get_num_components(&self) -> NumComponents {
        match DECODE_D3D10_SB_OPERAND_NUM_COMPONENTS(unsafe { *self.word }) {
            0 => NumComponents::Zero,
            1 => NumComponents::One,
            2 => NumComponents::Four,
            3 => NumComponents::N,
            _ => unreachable!()
        }
    }

    pub fn get_num_components_u32(&self) -> u32 {
        match DECODE_D3D10_SB_OPERAND_NUM_COMPONENTS(unsafe { *self.word }) {
            0 => 0,
            1 => 1,
            2 => 4,
            _ => unreachable!()
        }
    }

    pub fn get_component_select_mode(&self) -> ComponentSelectMode {
        match DECODE_D3D10_SB_OPERAND_4_COMPONENT_SELECTION_MODE(unsafe { *self.word }) {
            0 => ComponentSelectMode::Mask,
            1 => ComponentSelectMode::Swizzle,
            2 => ComponentSelectMode::Select1,
            _ => unreachable!()
        }
    }

    pub fn get_component_mask(&self) -> ComponentMask {
        ComponentMask::from_bits_truncate(DECODE_D3D10_SB_OPERAND_4_COMPONENT_MASK(unsafe { *self.word }))
    }

    pub fn get_component_swizzle(&self) -> ComponentSwizzle {
        let x = DECODE_D3D10_SB_OPERAND_4_COMPONENT_SWIZZLE_SOURCE(unsafe { *self.word }, D3D10_SB_4_COMPONENT_X) as u8;
        let y = DECODE_D3D10_SB_OPERAND_4_COMPONENT_SWIZZLE_SOURCE(unsafe { *self.word }, D3D10_SB_4_COMPONENT_Y) as u8;
        let z = DECODE_D3D10_SB_OPERAND_4_COMPONENT_SWIZZLE_SOURCE(unsafe { *self.word }, D3D10_SB_4_COMPONENT_Z) as u8;
        let w = DECODE_D3D10_SB_OPERAND_4_COMPONENT_SWIZZLE_SOURCE(unsafe { *self.word }, D3D10_SB_4_COMPONENT_W) as u8;

        ComponentSwizzle(
            ComponentName::from_word(x),
            ComponentName::from_word(y),
            ComponentName::from_word(z),
            ComponentName::from_word(w),
        )
    }

    pub fn get_operand_type(&self) -> OperandType {
        match DECODE_D3D10_SB_OPERAND_TYPE(unsafe { *self.word }) {
            0 => OperandType::Temp,
            1 => OperandType::Input,
            2 => OperandType::Output,
            3 => OperandType::IndexableTemp,
            4 => OperandType::Immediate32,
            5 => OperandType::Immediate64,
            6 => OperandType::Sampler,
            7 => OperandType::Resource,
            8 => OperandType::ConstantBuffer,
            9 => OperandType::ImmediateConstantBuffer,
            10 => OperandType::Label,
            11 => OperandType::InputPrimitiveId,
            12 => OperandType::OutputDepth,
            13 => OperandType::Null,
            14 => OperandType::Rasterizer,
            15 => OperandType::OutputCoverageMask,
            16 => OperandType::Stream,
            17 => OperandType::FunctionBody,
            18 => OperandType::FunctionTable,
            19 => OperandType::Interface,
            20 => OperandType::FunctionInput,
            21 => OperandType::FunctionOutput,
            22 => OperandType::OutputControlPointId,
            23 => OperandType::InputForkInstanceId,
            24 => OperandType::InputJoinInstanceId,
            25 => OperandType::InputControlPoint,
            26 => OperandType::OutputControlPoint,
            27 => OperandType::InputPatchConstant,
            28 => OperandType::InputDomainPoint,
            29 => OperandType::ThisPointer,
            30 => OperandType::UnorderedAccessView,
            31 => OperandType::ThreadGroupSharedMemory,
            32 => OperandType::InputThreadId,
            33 => OperandType::InputThreadGroupId,
            34 => OperandType::InputThreadIdInGroup,
            35 => OperandType::InputCoverageMask,
            36 => OperandType::InputThreadIdInGroupFlattened,
            37 => OperandType::InputGsinstanceid,
            38 => OperandType::OutputDepthGreaterEqual,
            39 => OperandType::OutputDepthLessEqual,
            40 => OperandType::CycleCounter,
            _ => unreachable!(),
        }
    }

    pub fn get_index_dimension(&self) -> IndexDimension {
        match DECODE_D3D10_SB_OPERAND_INDEX_DIMENSION(unsafe { *self.word }) {
            0 => IndexDimension::D0,
            1 => IndexDimension::D1,
            2 => IndexDimension::D2,
            3 => IndexDimension::D3,
            _ => unreachable!(),
        }
    }

    pub fn get_index_representation(&self, index: u32) -> IndexRepresentation {
        match DECODE_D3D10_SB_OPERAND_INDEX_REPRESENTATION(index, unsafe { *self.word }) {
            0 => IndexRepresentation::Immediate32,
            1 => IndexRepresentation::Immediate64,
            2 => IndexRepresentation::Relative,
            3 => IndexRepresentation::Immediate32PlusRelative,
            4 => IndexRepresentation::Immediate64PlusRelative,
            _ => unreachable!(),
        }
    }
}

use std::fmt;

impl<'a> fmt::Debug for OperandToken0<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("OperandToken0")
            .field("Raw", &unsafe { *self.word })
            .field("NumComponents", &self.get_num_components())
            .field("ComponentSelect", &self.get_component_select_mode())
            .field("ComponentMask", &self.get_component_mask())
            .field("ComponentSwizzle", &self.get_component_swizzle())
            .field("OperandType", &self.get_operand_type())
            .field("IndexDimension", &self.get_index_dimension())
            .field("Immediate", &self.get_immediates())
            .field("IsExtended", &self.is_extended())
            .finish()
    }
}

#[derive(Debug)]
pub struct DclGlobalFlags {
    pub global_flags: u32,
}

impl DclGlobalFlags {
    pub fn is_refactoring_allowed(&self) -> bool {
        (self.global_flags & D3D10_SB_GLOBAL_FLAG_REFACTORING_ALLOWED) != 0
    }

    pub fn is_enable_double_precision_float_ops_enabled(&self) -> bool {
        (self.global_flags & D3D11_SB_GLOBAL_FLAG_ENABLE_DOUBLE_PRECISION_FLOAT_OPS) != 0
    }

    pub fn is_force_early_depth_stencil_enabled(&self) -> bool {
        (self.global_flags & D3D11_SB_GLOBAL_FLAG_FORCE_EARLY_DEPTH_STENCIL) != 0
    }

    pub fn is_raw_and_structured_buffers_enabled(&self) -> bool {
        (self.global_flags & D3D11_SB_GLOBAL_FLAG_ENABLE_RAW_AND_STRUCTURED_BUFFERS) != 0
    }

    pub fn is_skip_optimization_enabled(&self) -> bool {
        (self.global_flags & D3D11_1_SB_GLOBAL_FLAG_SKIP_OPTIMIZATION) != 0
    }

    pub fn is_minimum_precision_enabled(&self) -> bool {
        (self.global_flags & D3D11_1_SB_GLOBAL_FLAG_ENABLE_MINIMUM_PRECISION) != 0
    }

    pub fn is_double_extensions_enabled(&self) -> bool {
        (self.global_flags & D3D11_1_SB_GLOBAL_FLAG_ENABLE_DOUBLE_EXTENSIONS) != 0
    }

    pub fn is_shader_extensions_enabled(&self) -> bool {
        (self.global_flags & D3D11_1_SB_GLOBAL_FLAG_ENABLE_SHADER_EXTENSIONS) != 0
    }
}

#[derive(Debug)]
pub struct DclInput<'a> {
    pub operand: OperandToken0<'a>,
}

impl<'a> DclInput<'a> {
    pub fn get_input_register(&self) -> u32 {
        match self.operand.get_immediate(0) {
            Immediate::U32(reg) => reg,
            _ => !0
        }
    }
}

#[derive(Debug)]
pub struct DclInputPs<'a> {
    pub operand: OperandToken0<'a>,
}

impl<'a> DclInputPs<'a> {
    pub fn get_input_register(&self) -> u32 {
        match self.operand.get_immediate(0) {
            Immediate::U32(reg) => reg,
            _ => !0
        }
    }
}

#[derive(Debug)]
pub struct DclOutput<'a> {
    pub operand: OperandToken0<'a>,
}

impl<'a> DclOutput<'a> {
    pub fn get_output_register(&self) -> u32 {
        match self.operand.get_immediate(0) {
            Immediate::U32(reg) => reg,
            _ => !0
        }
    }
}

#[derive(Debug)]
pub struct DclConstantBuffer<'a> {
    pub operand: OperandToken0<'a>,
    access: u32,
}

impl<'a> DclConstantBuffer<'a> {
    pub fn get_access_pattern(&self) -> ConstantBufferIndexPattern {
        match self.access {
            0 => ConstantBufferIndexPattern::Immediate,
            1 => ConstantBufferIndexPattern::Dynamic,
            _ => unreachable!()
        }
    }

    pub fn get_binding(&self) -> u32 {
        match self.operand.get_immediate(0) {
            Immediate::U32(reg) => reg,
            _ => !0
        }
    }

    pub fn get_size(&self) -> u32 {
        match self.operand.get_immediate(1) {
            Immediate::U32(reg) => reg,
            _ => !0
        }
    }
}

#[derive(Debug)]
pub struct DclResource<'a> {
    pub register: OperandToken0<'a>,
    pub return_type: ResourceReturnTypeToken0<'a>,
}

impl<'a> DclResource<'a> {
    pub fn get_register(&self) -> u32 {
        match self.register.get_immediate(0) {
            Immediate::U32(reg) => reg,
            _ => !0
        }
    }
}

#[derive(Debug)]
pub struct DclSampler<'a> {
    pub operand: OperandToken0<'a>,
}

impl<'a> DclSampler<'a> {
    pub fn get_register(&self) -> u32 {
        match self.operand.get_immediate(0) {
            Immediate::U32(reg) => reg,
            _ => !0
        }
    }
}

#[derive(Debug)]
pub struct DclOutputSiv<'a> {
    pub operand: OperandToken0<'a>,
    pub operand_2: OperandToken0<'a>,
}

impl<'a> DclOutputSiv<'a> {
    pub fn get_output_register(&self) -> u32 {
        match self.operand.get_immediate(0) {
            Immediate::U32(reg) => reg,
            _ => !0
        }
    }

    pub fn get_system_name(&self) -> NameToken {
        NameToken::from_word(DECODE_D3D10_SB_NAME(unsafe { *self.operand_2.word }))
    }
}

#[derive(Debug)]
pub struct DclOutputSgv<'a> {
    pub operand: OperandToken0<'a>,
    pub operand_2: OperandToken0<'a>,
}

impl<'a> DclOutputSgv<'a> {
    pub fn get_output_register(&self) -> u32 {
        match self.operand.get_immediate(0) {
            Immediate::U32(reg) => reg,
            _ => !0
        }
    }

    pub fn get_system_name(&self) -> NameToken {
        NameToken::from_word(DECODE_D3D10_SB_NAME(unsafe { *self.operand_2.word }))
    }
}



#[derive(Debug)]
pub struct DclInputPsSiv<'a> {
    pub operand: OperandToken0<'a>,
    pub operand_2: OperandToken0<'a>,
}

impl<'a> DclInputPsSiv<'a> {
    pub fn get_input_register(&self) -> u32 {
        match self.operand.get_immediate(0) {
            Immediate::U32(reg) => reg,
            _ => !0
        }
    }

    pub fn get_system_name(&self) -> NameToken {
        NameToken::from_word(DECODE_D3D10_SB_NAME(unsafe { *self.operand_2.word }))
    }
}

#[derive(Debug)]
pub struct DclInputPsSgv<'a> {
    pub operand: OperandToken0<'a>,
    pub operand_2: OperandToken0<'a>,
}

impl<'a> DclInputPsSgv<'a> {
    pub fn get_input_register(&self) -> u32 {
        match self.operand.get_immediate(0) {
            Immediate::U32(reg) => reg,
            _ => !0
        }
    }

    pub fn get_system_name(&self) -> NameToken {
        NameToken::from_word(DECODE_D3D10_SB_NAME(unsafe { *self.operand_2.word }))
    }
}

#[derive(Debug)]
pub struct DclTemps {
    pub register_count: u32,
}

#[derive(Debug)]
pub struct DclIndexableTemp {
    pub register_index: u32,
    pub register_count: u32,
    pub num_components: u32,
}

#[derive(Debug)]
pub struct Add<'a> {
    pub dst: OperandToken0<'a>,
    pub a: OperandToken0<'a>,
    pub b: OperandToken0<'a>,
}

#[derive(Debug)]
pub struct And<'a> {
    pub dst: OperandToken0<'a>,
    pub a: OperandToken0<'a>,
    pub b: OperandToken0<'a>,
}

#[derive(Debug)]
pub struct Mul<'a> {
    pub dst: OperandToken0<'a>,
    pub a: OperandToken0<'a>,
    pub b: OperandToken0<'a>,
}

#[derive(Debug)]
pub struct Mad<'a> {
    pub dst: OperandToken0<'a>,
    pub a: OperandToken0<'a>,
    pub b: OperandToken0<'a>,
    pub c: OperandToken0<'a>,
}

#[derive(Debug)]
pub struct Mov<'a> {
    pub dst: OperandToken0<'a>,
    pub src: OperandToken0<'a>,
}

#[derive(Debug)]
pub struct Itof<'a> {
    pub dst: OperandToken0<'a>,
    pub src: OperandToken0<'a>,
}

#[derive(Debug)]
pub struct Utof<'a> {
    pub dst: OperandToken0<'a>,
    pub src: OperandToken0<'a>,
}

#[derive(Debug)]
pub struct Ftou<'a> {
    pub dst: OperandToken0<'a>,
    pub src: OperandToken0<'a>,
}

#[derive(Debug)]
pub struct Loop;

#[derive(Debug)]
pub struct EndLoop;

#[derive(Debug)]
pub struct BreakC<'a> {
    pub src: OperandToken0<'a>,
}

#[derive(Debug)]
pub struct If<'a> {
    pub src: OperandToken0<'a>,
}

#[derive(Debug)]
pub struct SampleL<'a> {
    pub dst: OperandToken0<'a>,
    pub src_address: OperandToken0<'a>,
    pub src_resource: OperandToken0<'a>,
    pub src_sampler: OperandToken0<'a>,
    pub src_lod: OperandToken0<'a>,
}

#[derive(Debug)]
pub struct Sample<'a> {
    pub dst: OperandToken0<'a>,
    pub src_address: OperandToken0<'a>,
    pub src_resource: OperandToken0<'a>,
    pub src_sampler: OperandToken0<'a>,
}

#[repr(C)]
#[derive(Debug)]
pub struct ShexHeader {
    pub minor: u8,
    pub major: u8,
    pub program_type: u16,
    pub instruction_length: u32,
}

impl ShexHeader {
    pub fn parse<'b>(decoder: &'b mut decoder::Decoder) -> Result<Self, State> {
        let version = decoder.read_u8();
        let minor = version & 0x0f;
        let major = version >> 0x4;
        decoder.skip(1);
        let program_type = decoder.read_u16();
        let instruction_length = decoder.read_u32() - 2;

        Ok(ShexHeader {
            minor,
            major,
            program_type,
            instruction_length
        })
    }
}

#[derive(Debug)]
pub struct SparseInstruction<'a> {
    pub opcode: OpcodeToken0<'a>,
    pub operands: Operands<'a>,
}

#[derive(Debug)]
pub enum Operands<'a> {
    DclGlobalFlags(DclGlobalFlags),
    DclInput(DclInput<'a>),
    DclInputPs(DclInputPs<'a>),
    DclOutput(DclOutput<'a>),
    DclConstantBuffer(DclConstantBuffer<'a>),
    DclResource(DclResource<'a>),
    DclSampler(DclSampler<'a>),
    DclOutputSiv(DclOutputSiv<'a>),
    DclOutputSgv(DclOutputSgv<'a>),
    DclInputPsSiv(DclInputPsSiv<'a>),
    DclInputPsSgv(DclInputPsSgv<'a>),
    DclTemps(DclTemps),
    DclIndexableTemp(DclIndexableTemp),
    Add(Add<'a>),
    And(And<'a>),
    Mul(Mul<'a>),
    Mad(Mad<'a>),
    Mov(Mov<'a>),
    Itof(Itof<'a>),
    Utof(Utof<'a>),
    Ftou(Ftou<'a>),
    If(If<'a>),
    Else,
    EndIf,
    Loop,
    EndLoop,
    Break,
    BreakC(BreakC<'a>),
    Sample(Sample<'a>),
    SampleL(SampleL<'a>),
    Ret,
    Unknown
}

impl<'a> SparseInstruction<'a> {
    pub fn parse<'b>(decoder: &'b mut decoder::Decoder) -> SparseInstruction<'b> {
        let opcode = OpcodeToken0::from_word(decoder.read_u32_address());
        let ty = opcode.get_opcode_type();
        let len = opcode.get_instruction_length();

        let mut ex = opcode.get_extended_opcode();
        while let Some(opc) = ex {
            // println!("{:?}", opcode);
            // println!("{:?}", opc);
            ex = opc.get_extended_opcode();
            decoder.skip(4);
        }

        let operands = match ty {
            D3D10_SB_OPCODE_DCL_GLOBAL_FLAGS => {
                Operands::DclGlobalFlags(DclGlobalFlags {
                    global_flags: DECODE_D3D10_SB_GLOBAL_FLAGS(unsafe { *opcode.word }),
                })
            }
            D3D10_SB_OPCODE_DCL_INPUT => {
                Operands::DclInput(DclInput {
                    operand: OperandToken0::parse(decoder),
                })
            }
            D3D10_SB_OPCODE_DCL_INPUT_PS => {
                Operands::DclInputPs(DclInputPs {
                    operand: OperandToken0::parse(decoder),
                })
            }
            D3D10_SB_OPCODE_DCL_INPUT_PS_SIV => {
                Operands::DclInputPsSiv(DclInputPsSiv {
                    operand: OperandToken0::parse(decoder),
                    operand_2: OperandToken0::parse(decoder),
                })
            }
            D3D10_SB_OPCODE_DCL_INPUT_PS_SGV => {
                Operands::DclInputPsSgv(DclInputPsSgv {
                    operand: OperandToken0::parse(decoder),
                    operand_2: OperandToken0::parse(decoder),
                })
            }
            D3D10_SB_OPCODE_DCL_OUTPUT => {
                Operands::DclOutput(DclOutput {
                    operand: OperandToken0::parse(decoder),
                })
            }
            D3D10_SB_OPCODE_DCL_CONSTANT_BUFFER => {
                Operands::DclConstantBuffer(DclConstantBuffer {
                    operand: OperandToken0::parse(decoder),
                    access: DECODE_D3D10_SB_CONSTANT_BUFFER_ACCESS_PATTERN(unsafe { *opcode.word }),
                })
            }
            D3D10_SB_OPCODE_DCL_RESOURCE => {
                Operands::DclResource(DclResource {
                    register: OperandToken0::parse(decoder),
                    return_type: ResourceReturnTypeToken0::from_word(decoder.read_u32_address()),
                })
            }
            D3D10_SB_OPCODE_DCL_SAMPLER => {
                Operands::DclSampler(DclSampler {
                    operand: OperandToken0::parse(decoder),
                })
            }
            D3D10_SB_OPCODE_DCL_TEMPS => {
                Operands::DclTemps(DclTemps {
                    register_count: decoder.read_u32(),
                })
            }
            D3D10_SB_OPCODE_DCL_INDEXABLE_TEMP => {
                Operands::DclIndexableTemp(DclIndexableTemp {
                    register_index: decoder.read_u32(),
                    register_count: decoder.read_u32(),
                    num_components: decoder.read_u32(),
                })
            }
            D3D10_SB_OPCODE_DCL_OUTPUT_SIV => {
                Operands::DclOutputSiv(DclOutputSiv {
                    operand: OperandToken0::parse(decoder),
                    operand_2: OperandToken0::parse(decoder),
                })
            }
            D3D10_SB_OPCODE_DCL_OUTPUT_SGV => {
                Operands::DclOutputSgv(DclOutputSgv {
                    operand: OperandToken0::parse(decoder),
                    operand_2: OperandToken0::parse(decoder),
                })
            }
            D3D10_SB_OPCODE_ADD => {
                Operands::Add(Add {
                    dst: OperandToken0::parse(decoder),
                    a: OperandToken0::parse(decoder),
                    b: OperandToken0::parse(decoder),
                })
            }
            D3D10_SB_OPCODE_AND => {
                Operands::And(And {
                    dst: OperandToken0::parse(decoder),
                    a: OperandToken0::parse(decoder),
                    b: OperandToken0::parse(decoder),
                })
            }
            D3D10_SB_OPCODE_MUL => {
                Operands::Mul(Mul {
                    dst: OperandToken0::parse(decoder),
                    a: OperandToken0::parse(decoder),
                    b: OperandToken0::parse(decoder),
                })
            }
            D3D10_SB_OPCODE_MAD => {
                Operands::Mad(Mad {
                    dst: OperandToken0::parse(decoder),
                    a: OperandToken0::parse(decoder),
                    b: OperandToken0::parse(decoder),
                    c: OperandToken0::parse(decoder),
                })
            }
            D3D10_SB_OPCODE_MOV => {
                Operands::Mov(Mov {
                    dst: OperandToken0::parse(decoder),
                    src: OperandToken0::parse(decoder),
                })
            }
            D3D10_SB_OPCODE_ITOF => {
                Operands::Itof(Itof {
                    dst: OperandToken0::parse(decoder),
                    src: OperandToken0::parse(decoder),
                })
            }
            D3D10_SB_OPCODE_UTOF => {
                Operands::Utof(Utof {
                    dst: OperandToken0::parse(decoder),
                    src: OperandToken0::parse(decoder),
                })
            }
            D3D10_SB_OPCODE_FTOU => {
                Operands::Ftou(Ftou {
                    dst: OperandToken0::parse(decoder),
                    src: OperandToken0::parse(decoder),
                })
            }
            D3D10_SB_OPCODE_IF => {
                Operands::If(If {
                    src: OperandToken0::parse(decoder),
                })
            }
            D3D10_SB_OPCODE_ELSE => {
                Operands::Else
            }
            D3D10_SB_OPCODE_ENDIF => {
                Operands::EndIf
            }
            D3D10_SB_OPCODE_LOOP => {
                Operands::Loop
            }
            D3D10_SB_OPCODE_ENDLOOP => {
                Operands::EndLoop
            }
            D3D10_SB_OPCODE_BREAK => {
                Operands::Break
            }
            D3D10_SB_OPCODE_BREAKC => {
                Operands::BreakC(BreakC {
                    src: OperandToken0::parse(decoder),
                })
            }
            D3D10_SB_OPCODE_SAMPLE => {
                Operands::Sample(Sample {
                    dst: OperandToken0::parse(decoder),
                    src_address: OperandToken0::parse(decoder),
                    src_resource: OperandToken0::parse(decoder),
                    src_sampler: OperandToken0::parse(decoder),
                })
            }
            D3D10_SB_OPCODE_SAMPLE_L => {
                Operands::SampleL(SampleL {
                    dst: OperandToken0::parse(decoder),
                    src_address: OperandToken0::parse(decoder),
                    src_resource: OperandToken0::parse(decoder),
                    src_sampler: OperandToken0::parse(decoder),
                    src_lod: OperandToken0::parse(decoder),
                })
            }
            D3D10_SB_OPCODE_RET => {
                Operands::Ret
            }
            _ => {
                if len > 1 {
                    decoder.skip(4 * (len as usize  - 1));
                }

                Operands::Unknown
            }
        };

        SparseInstruction {
            opcode,
            operands,
        }
    }
}
