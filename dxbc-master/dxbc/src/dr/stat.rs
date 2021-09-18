use binary::*;

#[repr(C)]
#[derive(Debug)]
pub struct IStatChunk {
    pub instruction_count: u32,
    pub temp_register_count: u32,
    pub def_count: u32,
    pub dcl_count: u32,
    pub float_instruction_count: u32,
    pub int_instruction_count: u32,
    pub uint_instruction_count: u32,
    pub static_flow_control_count: u32,
    pub dynamic_flow_control_count: u32,
    pub macro_instruction_count: u32,
    pub temp_array_count: u32,
    pub array_instruction_count: u32,
    pub cut_instruction_count: u32,
    pub emit_instruction_count: u32,
    pub texture_normal_instructions: u32,
    pub texture_load_instructions: u32,
    pub texture_comp_instructions: u32,
    pub texture_bias_instructions: u32,
    pub texture_gradient_instructions: u32,
}

impl IStatChunk {
    pub fn parse(decoder: &mut decoder::Decoder) -> Result<IStatChunk, State> {
        Ok(IStatChunk {
            instruction_count: decoder.read_u32(),
            temp_register_count: decoder.read_u32(),
            def_count: decoder.read_u32(),
            dcl_count: decoder.read_u32(),
            float_instruction_count: decoder.read_u32(),
            int_instruction_count: decoder.read_u32(),
            uint_instruction_count: decoder.read_u32(),
            static_flow_control_count: decoder.read_u32(),
            dynamic_flow_control_count: decoder.read_u32(),
            macro_instruction_count: decoder.read_u32(),
            temp_array_count: decoder.read_u32(),
            array_instruction_count: decoder.read_u32(),
            cut_instruction_count: decoder.read_u32(),
            emit_instruction_count: decoder.read_u32(),
            texture_normal_instructions: decoder.read_u32(),
            texture_load_instructions: decoder.read_u32(),
            texture_comp_instructions: decoder.read_u32(),
            texture_bias_instructions: decoder.read_u32(),
            texture_gradient_instructions: decoder.read_u32(),
        })
    }
}