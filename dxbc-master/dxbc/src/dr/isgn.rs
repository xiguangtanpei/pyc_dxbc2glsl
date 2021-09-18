use binary::*;

use std::mem;

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum RegisterComponentType {
    Unknown = 0,
    Uint32 = 1,
    Int32 = 2,
    Float32 = 3,
}

impl RegisterComponentType {
    pub fn from_word(word: u32) -> Self {
        match word {
            0...3 => unsafe { mem::transmute(word) },
            _ => unreachable!()
        }
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum SemanticName {
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
    FinalQuadEdgeTessfactor = 11,
    FinalQuadInsideTessfactor = 12,
    FinalTriEdgeTessfactor = 13,
    FinalTriInsideTessfactor = 14,
    FinalLineDetailTessfactor = 15,
    FinalLineDensityTessfactor = 16,
    Target = 64,
    Depth = 65,
    Coverage = 66,
    DepthGreaterEqual = 67,
    DepthLessEqual = 68,
}

impl SemanticName {
    pub fn from_word(word: u32) -> Self {
        match word {
            0...16 |
            64...68 => unsafe { mem::transmute(word) },
            _ => unreachable!()
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct InputOutputElement {
    pub name: String,
    pub semantic_index: u32,
    pub semantic_type: SemanticName,
    pub component_type: RegisterComponentType,
    pub register: u32,
    pub component_mask: u8,
    pub rw_mask: u8,
}

impl InputOutputElement {
    pub fn parse<'a>(decoder: &mut decoder::Decoder<'a>) -> Result<Self, State> {
        let name_offset = decoder.read_u32();
        let semantic_index = decoder.read_u32();
        let semantic_type = SemanticName::from_word(decoder.read_u32());
        let component_type = RegisterComponentType::from_word(decoder.read_u32());
        let register = decoder.read_u32();
        let component_mask = decoder.read_u8();
        let rw_mask = decoder.read_u8();
        decoder.skip(2);

        let name = decoder.seek(name_offset as usize).string().map_err(|e| State::DecoderError(e))?;

        Ok(Self {
            name,
            semantic_index,
            semantic_type,
            component_type,
            register,
            component_mask,
            rw_mask,
        })
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct IOsgnChunk {
    pub elements: Vec<InputOutputElement>,
}

impl IOsgnChunk {
    pub fn parse<'b>(decoder: &'b mut decoder::Decoder) -> Result<IOsgnChunk, State> {
        let element_count = decoder.read_u32();
        let _unknown = decoder.read_u32();

        let mut elements = Vec::new();
        for _ in 0..element_count {
            elements.push(InputOutputElement::parse(decoder)?);
        }

        Ok(IOsgnChunk {
            elements,
        })
    }
}
