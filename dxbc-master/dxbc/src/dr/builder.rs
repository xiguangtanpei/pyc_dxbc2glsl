use dr::shex::{ResourceDimension, ResourceReturnType};
use dr::{IStatChunk, IOsgnChunk, RdefChunk};

use byteorder::{ByteOrder, LittleEndian};
use checksum;
use d3d11tokenizedprogramformat::*;

use std::{slice, mem};

const DXBC_MAGIC: u32 = 0x43425844;
const RDEF_MAGIC: u32 = 0x46454452;
const RD11_MAGIC: u32 = 0x31314452;
const ISGN_MAGIC: u32 = 0x4e475349;
const OSGN_MAGIC: u32 = 0x4e47534f;
const SHEX_MAGIC: u32 = 0x58454853;
const STAT_MAGIC: u32 = 0x54415453;

pub struct Builder<'a> {
    rdef: Option<RdefChunk<'a>>,
    isgn: Option<IOsgnChunk>,
    osgn: Option<IOsgnChunk>,
    stat: Option<IStatChunk>,
    shex: Option<ShexChunk>,
    code: Vec<u32>,
}

pub struct DxbcModule {
    pub dwords: Vec<u32>,
}

impl DxbcModule {
    pub fn new() -> Self {
        DxbcModule {
            dwords: Vec::new()
        }
    }

    pub fn position(&self) -> usize {
        self.dwords.len()
    }

    pub fn get_u32(&mut self, pos: usize) -> u32 {
        self.dwords[pos]
    }

    pub fn write_u32(&mut self, val: u32) {
        self.dwords.push(val);
    }

    pub fn set_u32_slice(&mut self, offset: usize, val: &[u32]) {
        self.dwords[offset..].copy_from_slice(val);
    }

    pub fn set_u32(&mut self, offset: usize, val: u32) {
        self.dwords[offset] = val;
    }

    pub fn write_str(&mut self, text: &str) -> u32 {
        let mut len = 0;

        // NOTE: fxc pads with 0xABAB.. pattern
        for chunk in text.as_bytes().chunks(4) {
            let data = match chunk {
                &[d, c, b, a] => ((a as u32) << 24) | ((b as u32) << 16) | ((c as u32) << 8) | d as u32,
                &[c, b, a] => ((a as u32) << 16) | ((b as u32) << 8) | (c as u32),
                &[b, a] => ((a as u32) << 8) | (b as u32),
                &[a] => a as u32,
                _ => unreachable!()
            };

            self.write_u32(data);

            len += 4
        }

        // if string is aligned we append padded null-terminator
        if text.len() & 3 == 0 {
            self.write_u32(0);

            len += 4;
        }

        len
    }

    pub fn write_stat(&mut self, stat: &IStatChunk) {
        self.write_u32(STAT_MAGIC);

        let stat_size_pos = self.position();
        self.write_u32(0);
        let chunk_start = self.position();

        self.write_u32(stat.instruction_count);
        self.write_u32(stat.temp_register_count);
        self.write_u32(stat.def_count);
        self.write_u32(stat.dcl_count);
        self.write_u32(stat.float_instruction_count);
        self.write_u32(stat.int_instruction_count);
        self.write_u32(stat.uint_instruction_count);
        self.write_u32(stat.static_flow_control_count);
        self.write_u32(stat.dynamic_flow_control_count);
        self.write_u32(stat.macro_instruction_count);
        self.write_u32(stat.temp_array_count);
        self.write_u32(stat.array_instruction_count);
        self.write_u32(stat.cut_instruction_count);
        self.write_u32(stat.emit_instruction_count);
        self.write_u32(stat.texture_normal_instructions);
        self.write_u32(stat.texture_load_instructions);
        self.write_u32(stat.texture_comp_instructions);
        self.write_u32(stat.texture_bias_instructions);
        self.write_u32(stat.texture_gradient_instructions);

        for _ in 0..18 {
            self.write_u32(0);
        }

        let end_pos = self.position();
        self.set_u32(stat_size_pos, 4 * (end_pos - chunk_start) as u32);
    }

    pub fn write_rdef(&mut self, rdef: &RdefChunk) {
        self.write_u32(RDEF_MAGIC);
        let rdef_size_pos = self.position();
        self.write_u32(0);
        let chunk_start = self.position();
        self.write_u32(rdef.constant_buffers.len() as u32);
        let constant_buffers_pos = self.position();
        self.write_u32(0);
        self.write_u32(rdef.resource_bindings.len() as u32);
        let resource_bindings_pos = self.position();
        self.write_u32(0);

        let version_tok = (((rdef.shader_ty as u32) << 16) & 0xffff0000) |
                          (((rdef.major as u32) << 4) & 0x000000f0) |
                          (rdef.minor as u32 & 0x0000000f);
        self.write_u32(version_tok);
        self.write_u32(rdef.flags);
        let author_pos = self.position();
        self.write_u32(0);

        if let Some(rd11) = rdef.rd11 {
            self.write_u32(RD11_MAGIC);
            self.write_u32(60);
            self.write_u32(24);
            self.write_u32(32);
            self.write_u32(40);
            self.write_u32(36);
            self.write_u32(12);
            self.write_u32(0);
            // self.write_u32(rd11[0]);
            // self.write_u32(rd11[1]);
            // self.write_u32(rd11[2]);
            // self.write_u32(rd11[3]);
            // self.write_u32(rd11[4]);
            // self.write_u32(rd11[5]);
            // self.write_u32(rd11[6]);
        }

        let constant_buffers_loc = (self.position() - chunk_start) as u32;
        self.set_u32(constant_buffers_pos, constant_buffers_loc);
        for constant_buffer in &rdef.constant_buffers {
            // TODO: 
        }

        let resource_bindings_loc = (self.position() - chunk_start) as u32;
        self.set_u32(resource_bindings_pos, resource_bindings_loc);
        for resource_binding in &rdef.resource_bindings {
            // TODO: 
        }

        let author_loc = 4 * (self.position() - chunk_start) as u32;
        self.set_u32(author_pos, author_loc);
        self.write_str(rdef.author);

        let end_pos = self.position();
        self.set_u32(rdef_size_pos, 4 * (end_pos - chunk_start) as u32);
    }

    pub fn write_iosgn(&mut self, chunk: &IOsgnChunk, magic: u32) {
        self.write_u32(magic);
        let chunk_sz_pos = self.position();
        self.write_u32(0);
        let chunk_start = self.position();
        self.write_u32(chunk.elements.len() as u32);
        // TODO: unknown
        self.write_u32(0x8);

        let mut string_positions = Vec::new();
        for element in &chunk.elements {
            string_positions.push(self.position());
            self.write_u32(0);
            self.write_u32(element.semantic_index);
            self.write_u32(element.semantic_type as u32);
            self.write_u32(element.component_type as u32);
            self.write_u32(element.register);
            let mask_tok = ((element.rw_mask as u32) << 8) | (element.component_mask as u32);
            self.write_u32(mask_tok);
        }

        for (element, pos) in chunk.elements.iter().zip(string_positions) {
            let name_offset = self.position() - chunk_start;
            self.set_u32(pos, 4 * name_offset as u32);
            self.write_str(&element.name);
        }

        let chunk_sz = self.position() - chunk_start;
        self.set_u32(chunk_sz_pos, 4 * chunk_sz as u32);
    }

    pub fn write_isgn(&mut self, chunk: &IOsgnChunk) {
        self.write_iosgn(chunk, ISGN_MAGIC);
    }

    pub fn write_osgn(&mut self, chunk: &IOsgnChunk) {
        self.write_iosgn(chunk, OSGN_MAGIC);
    }

    pub fn write_shex(&mut self, chunk: &ShexChunk) {
        self.write_u32(SHEX_MAGIC);
        let chunk_sz_pos = self.position();
        self.write_u32(0);
        let chunk_start = self.position();

        self.write_u32(
            ENCODE_D3D10_SB_TOKENIZED_PROGRAM_VERSION_TOKEN(
                D3D10_SB_VERTEX_SHADER, 5, 0
            )
        );

        let word_sz_pos = self.position();
        self.write_u32(0);

        for instr in &chunk.instructions {
            instr.encode(self);
        }

        let chunk_sz = self.position() - chunk_start;
        self.set_u32(word_sz_pos, chunk_sz as u32);
        self.set_u32(chunk_sz_pos, 4 * chunk_sz as u32);
    }

    pub fn write_opcode(&mut self, op: u32, instruction_len: u32, test: Option<u32>, saturated: bool, extended: &[OpcodeEx]) {
        let mut opcode = ENCODE_D3D10_SB_OPCODE_TYPE(op) |
            ENCODE_D3D10_SB_TOKENIZED_INSTRUCTION_LENGTH(instruction_len) |
            ENCODE_D3D10_SB_INSTRUCTION_SATURATE(saturated as u32);

        if extended.len() != 0 {
            opcode |= ENCODE_D3D10_SB_OPCODE_EXTENDED(1);
        }

        if let Some(test) = test {
            opcode |= ENCODE_D3D10_SB_INSTRUCTION_TEST_BOOLEAN(test as u32);
        }

        self.write_u32(opcode);

        for (idx, &op_ex) in extended.iter().enumerate() {
            self.write_opcode_ex(op_ex, idx != extended.len() - 1);
        }
    }

    pub fn write_opcode_ex(&mut self, opcode: OpcodeEx, more: bool) {
        match opcode {
            OpcodeEx::UvOffset(u, v, w) => {
                self.write_u32(
                    ENCODE_D3D10_SB_EXTENDED_OPCODE_TYPE(D3D10_SB_EXTENDED_OPCODE_SAMPLE_CONTROLS) |
                    ENCODE_IMMEDIATE_D3D10_SB_ADDRESS_OFFSET(D3D10_SB_IMMEDIATE_ADDRESS_OFFSET_U, u) |
                    ENCODE_IMMEDIATE_D3D10_SB_ADDRESS_OFFSET(D3D10_SB_IMMEDIATE_ADDRESS_OFFSET_V, v) |
                    ENCODE_IMMEDIATE_D3D10_SB_ADDRESS_OFFSET(D3D10_SB_IMMEDIATE_ADDRESS_OFFSET_W, w) |
                    if more { ENCODE_D3D10_SB_OPCODE_EXTENDED(1) } else { 0 }
                );
            },
            OpcodeEx::Dimension(dim, stride) => {
                self.write_u32(
                    ENCODE_D3D10_SB_EXTENDED_OPCODE_TYPE(D3D11_SB_EXTENDED_OPCODE_RESOURCE_DIM) |
                    ENCODE_D3D11_SB_EXTENDED_RESOURCE_DIMENSION(dim as u32) |
                    ENCODE_D3D11_SB_EXTENDED_RESOURCE_DIMENSION_STRUCTURE_STRIDE(stride) |
                    if more { ENCODE_D3D10_SB_OPCODE_EXTENDED(1) } else { 0 }
                );
            }
            OpcodeEx::ResourceReturnType(x, y, z, w) => {
                self.write_u32(
                    ENCODE_D3D10_SB_EXTENDED_OPCODE_TYPE(D3D11_SB_EXTENDED_OPCODE_RESOURCE_RETURN_TYPE) |
                    ENCODE_D3D11_SB_EXTENDED_RESOURCE_RETURN_TYPE(x as u32, 0) |
                    ENCODE_D3D11_SB_EXTENDED_RESOURCE_RETURN_TYPE(y as u32, 1) |
                    ENCODE_D3D11_SB_EXTENDED_RESOURCE_RETURN_TYPE(z as u32, 2) |
                    ENCODE_D3D11_SB_EXTENDED_RESOURCE_RETURN_TYPE(w as u32, 3) |
                    if more { ENCODE_D3D10_SB_OPCODE_EXTENDED(1) } else { 0 }
                );
            }
        }
    }

    pub fn write_operand(&mut self, op: u32, modifier: Modifier, component_mode: NumComponent, immediates: &[Immediate]) {
        debug_assert!(immediates.len() < 4);

        let mut operand = ENCODE_D3D10_SB_OPERAND_TYPE(op);

        if let Modifier::None = modifier {

        } else {
            operand |= ENCODE_D3D10_SB_OPERAND_EXTENDED(1);
        }

        for (idx, imm) in immediates.iter().enumerate() {
            let repr = match imm {
                Immediate::U32(..) => D3D10_SB_OPERAND_INDEX_IMMEDIATE32,
                Immediate::U64(..) => D3D10_SB_OPERAND_INDEX_IMMEDIATE64,
                Immediate::Relative(..) => D3D10_SB_OPERAND_INDEX_RELATIVE,
                Immediate::U32Relative(..) => D3D10_SB_OPERAND_INDEX_IMMEDIATE32_PLUS_RELATIVE,
                Immediate::U64Relative(..) => D3D10_SB_OPERAND_INDEX_IMMEDIATE64_PLUS_RELATIVE,
            };

            operand |= ENCODE_D3D10_SB_OPERAND_INDEX_REPRESENTATION(idx as u32, repr);
        }

        let component_count = match component_mode {
            NumComponent::D0 => D3D10_SB_OPERAND_0_COMPONENT,
            NumComponent::D1 => D3D10_SB_OPERAND_1_COMPONENT,
            NumComponent::D4(..) => D3D10_SB_OPERAND_4_COMPONENT,
        };

        operand |= ENCODE_D3D10_SB_OPERAND_NUM_COMPONENTS(component_count);

        let immediate_arity = match immediates.len() {
            0 => D3D10_SB_OPERAND_INDEX_0D,
            1 => D3D10_SB_OPERAND_INDEX_1D,
            2 => D3D10_SB_OPERAND_INDEX_2D,
            3 => D3D10_SB_OPERAND_INDEX_3D,
            _ => unreachable!()
        };

        operand |= ENCODE_D3D10_SB_OPERAND_INDEX_DIMENSION(immediate_arity);

        match component_mode {
            NumComponent::D0 | NumComponent::D1 => {},
            NumComponent::D4(mode) => match mode {
                ComponentMode::Mask(mask) => {
                    operand |=
                        ENCODE_D3D10_SB_OPERAND_4_COMPONENT_SELECTION_MODE(D3D10_SB_OPERAND_4_COMPONENT_MASK_MODE) |
                        mask as u32;
                },
                ComponentMode::Swizzle(x, y, z, w) => {
                    operand |=
                        ENCODE_D3D10_SB_OPERAND_4_COMPONENT_SELECTION_MODE(D3D10_SB_OPERAND_4_COMPONENT_SWIZZLE_MODE) |
                        ENCODE_D3D10_SB_OPERAND_4_COMPONENT_SWIZZLE(x as u32 >> 5, y as u32 >> 5, z as u32 >> 5, w as u32 >> 5);
                },
                ComponentMode::Select(comp) => {
                    operand |=
                        ENCODE_D3D10_SB_OPERAND_4_COMPONENT_SELECTION_MODE(D3D10_SB_OPERAND_4_COMPONENT_SELECT_1_MODE) |
                        ENCODE_D3D10_SB_OPERAND_4_COMPONENT_SELECT_1(comp as u32 >> 5);
                },
            },
        }

        self.write_u32(operand);

        let operand_modifier = match modifier {
            Modifier::None => None,
            Modifier::Neg => Some(D3D10_SB_OPERAND_MODIFIER_NEG),
            Modifier::Abs => Some(D3D10_SB_OPERAND_MODIFIER_ABS),
            Modifier::AbsNeg => Some(D3D10_SB_OPERAND_MODIFIER_ABSNEG),
        };

        if let Some(modifier) = operand_modifier {
            self.write_u32(
                ENCODE_D3D10_SB_EXTENDED_OPERAND_MODIFIER(modifier)
            );
        }

        for imm in immediates {
            let relative = match imm {
                &Immediate::U32(val) => {
                    self.write_u32(val);
                    None
                }
                &Immediate::U64(val) => {
                    // TODO: u64
                    self.write_u32(val as u32);
                    None
                }
                &Immediate::Relative(ref rel) => {
                    Some(rel)
                }
                &Immediate::U32Relative(val, ref rel) => {
                    self.write_u32(val);
                    Some(rel)
                }
                &Immediate::U64Relative(val, ref rel) => {
                    // TODO: u64
                    self.write_u32(val as u32);
                    Some(rel)
                }
            };

            if let Some(relative) = relative {
                let operand_ty = relative.get_type();

                self.write_operand(operand_ty, relative.modifiers, relative.component_mode, &[]);
            }
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(
                self.dwords.as_ptr() as *const u8,
                self.dwords.len() * mem::size_of::<u32>(),
            )
        }
    }
}

impl<'a> Builder<'a> {
    pub fn new() -> Self {
        Builder {
            rdef: None,
            isgn: None,
            osgn: None,
            shex: None,
            stat: None,
            code: Vec::new(),
        }
    }

    pub fn set_rdef(&mut self, rdef: RdefChunk<'a>) {
        self.rdef = Some(rdef);
    }

    pub fn set_isgn(&mut self, isgn: IOsgnChunk) {
        self.isgn = Some(isgn);
    }

    pub fn set_osgn(&mut self, osgn: IOsgnChunk) {
        self.osgn = Some(osgn);
    }

    pub fn set_shex(&mut self, shex: ShexChunk) {
        self.shex = Some(shex);
    }

    pub fn set_stat(&mut self, stat: IStatChunk) {
        self.stat = Some(stat);
    }

    pub fn set_profile(&mut self) {

    }

    pub fn module(&self) -> Result<DxbcModule, ()> {
        let mut module = DxbcModule::new();

        module.write_u32(DXBC_MAGIC);
        let checksum_pos = module.position();
        module.write_u32(0);
        module.write_u32(0);
        module.write_u32(0);
        module.write_u32(0);

        module.write_u32(1);

        let size_pos = module.position();
        module.write_u32(0);

        let mut chunk_count = 4;
        module.write_u32(chunk_count);
        let chunk_count_pos = module.position();
        for _ in 0..chunk_count {
            module.write_u32(0);
        }


        if let Some(ref rdef) = self.rdef {
            let pos = module.position() * 4;
            module.set_u32(chunk_count_pos, pos as u32);
            module.write_rdef(rdef);
        }

        if let Some(ref isgn) = self.isgn {
            let pos = module.position() * 4;
            module.set_u32(chunk_count_pos + 1, pos as u32);
            module.write_isgn(isgn);
        }

        if let Some(ref osgn) = self.osgn {
            let pos = module.position() * 4;
            module.set_u32(chunk_count_pos + 2, pos as u32);
            module.write_osgn(osgn);
        }

        if let Some(ref shex) = self.shex {
            let pos = module.position() * 4;
            module.set_u32(chunk_count_pos + 3, pos as u32);
            module.write_shex(shex);
        }


        // finally, patch in size and checksum
        let len = 4 * module.dwords.len() as u32;
        module.set_u32(size_pos, len);
        let checksum = ::checksum(module.as_bytes());
        module.set_u32(checksum_pos,     checksum[0]);
        module.set_u32(checksum_pos + 1, checksum[1]);
        module.set_u32(checksum_pos + 2, checksum[2]);
        module.set_u32(checksum_pos + 3, checksum[3]);

        Ok(module)
    }
}

bitflags! {
    pub struct GlobalFlags: u32 {
        const REFACTORING_ALLOWED = 1 << 11;
        const ENABLE_DOUBLE_PRECISION_FLOAT_OPS = 1 << 12;
        const FORCE_EARLY_DEPTH_STENCIL = 1 << 13;
        const ENABLE_RAW_AND_STRUCTURED_BUFFERS = 1 << 14;
        const SKIP_OPTIMIZATION = 1 << 15;
        const ENABLE_MINIMUM_PRECISION = 1 << 16;
        const ENABLE_DOUBLE_EXTENSIONS = 1 << 17;
        const ENABLE_SHADER_EXTENSIONS = 1 << 18;

    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum Semantic {
    Undefined = 0,
    Position = 1,
}

#[derive(Debug, Copy, Clone)]
pub enum Modifier {
    None,
    Neg,
    Abs,
    AbsNeg
}

bitflags! {
    pub struct Component: u8 {
        const X = 0x10;
        const Y = 0x20;
        const Z = 0x40;
        const W = 0x80;
    }
}

pub const X: u8 = 0x10;
pub const Y: u8 = 0x20;
pub const Z: u8 = 0x40;
pub const W: u8 = 0x80;

#[derive(Debug, Copy, Clone)]
pub enum ComponentMode {
    Mask(u8),
    Swizzle(u8, u8, u8, u8),
    Select(u8),
}

#[derive(Debug, Copy, Clone)]
pub enum NumComponent {
    D0,
    D1,
    D4(ComponentMode),
}

#[derive(Debug)]
pub enum Address {
    Constant(u32),
    Relative(IndexOperandType)
}

#[derive(Debug)]
pub enum OperandType {
    Register(u32),
    Input(u32),
    Output(u32),
    Imm32(u32),
    Imm32x2(u32, u32),
    Imm32x3(u32, u32, u32),
    Imm32x4(u32, u32, u32, u32),
    Resource(u32),
    Sampler(u32),
    IndexableRegister(u32, Address),
    ConstantBuffer(u32, Address),
    CustomData(Vec<u32>),
}

#[derive(Debug)]
pub enum Immediate {
    U32(u32),
    U64(u64),
    Relative(Operand),
    U32Relative(u32, Operand),
    U64Relative(u64, Operand),
}

#[derive(Debug)]
pub enum IndexOperandType {
    Register(u32),
    Input(u32),
    Output(u32),
}

#[derive(Debug)]
pub enum Instruction {
    DclGlobalFlags { flags: GlobalFlags },
    DclTemps { count: u32 },
    DclOutputSiv { register: Operand, semantic: Semantic },
    DclInput { register: Operand },
    Add { dest: Operand, a: Operand, b: Operand, saturated: bool },
    Mul { dest: Operand, a: Operand, b: Operand, saturated: bool },
    Ret
}

#[derive(Copy, Clone)]
pub enum OpcodeEx {
    UvOffset(u32, u32, u32),
    Dimension(ResourceDimension, u32),
    ResourceReturnType(ResourceReturnType, ResourceReturnType, ResourceReturnType, ResourceReturnType),
}




impl Instruction {
    fn get_opcode(&self) -> u32 {
        match self {
            Instruction::Add { .. } => D3D10_SB_OPCODE_ADD,
            Instruction::Mul { .. } => D3D10_SB_OPCODE_MUL,
            Instruction::DclGlobalFlags { .. } => D3D10_SB_OPCODE_DCL_GLOBAL_FLAGS,
            Instruction::DclOutputSiv { .. } => D3D10_SB_OPCODE_DCL_OUTPUT_SIV,
            Instruction::DclInput { .. } => D3D10_SB_OPCODE_DCL_INPUT,
            Instruction::DclTemps { .. } => D3D10_SB_OPCODE_DCL_TEMPS,
            Instruction::Ret => D3D10_SB_OPCODE_RET,
        }
    }

    fn encode_opcode(&self, module: &mut DxbcModule) {
        let opcode = self.get_opcode();

        match self {
            Instruction::Add { dest, a, b, saturated } |
            Instruction::Mul { dest, a, b, saturated } => { module.write_opcode(opcode, 0, None, *saturated, &[]); }

            Instruction::DclGlobalFlags { flags } => {
                let opcode_pos = module.position();
                module.write_opcode(opcode, 0, None, false, &[]);

                let opcode = module.get_u32(opcode_pos);
                module.set_u32(opcode_pos, opcode | (flags.bits() & 0x00fff800));
            }
            Instruction::DclOutputSiv { .. } |
            Instruction::DclInput { .. } |
            Instruction::DclTemps { .. } |
            Instruction::Ret => { module.write_opcode(opcode, 0, None, false, &[]); }
            _ => {}
        }

    }

    fn encode(&self, module: &mut DxbcModule) {
        let start = module.position();

        self.encode_opcode(module);

        match self {
            Instruction::Add { dest, a, b, .. } |
            Instruction::Mul { dest, a, b, .. } => {
                dest.encode(module);
                a.encode(module);
                b.encode(module);
            }
            &Instruction::DclTemps { count: val } => {
                module.write_u32(val)
            }
            &Instruction::DclOutputSiv { ref register, semantic } => {
                register.encode(module);
                module.write_u32(semantic as u32)
            }
            &Instruction::DclInput { ref register } => {
                register.encode(module);
            }
            _ => {}
        }

        // patch in instruction length
        let end = module.position();
        let sz = end - start;
        let opcode = module.get_u32(start);

        module.set_u32(start, opcode | ENCODE_D3D10_SB_TOKENIZED_INSTRUCTION_LENGTH(sz as u32));
    }
}

#[derive(Debug)]
pub struct Operand {
    ty: OperandType,
    modifiers: Modifier,
    component_mode: NumComponent
}

impl Operand {
    pub fn new(ty: OperandType, modifiers: Modifier, component_mode: NumComponent) -> Self {
        Operand {
            ty,
            modifiers,
            component_mode
        }
    }

    pub fn register(reg: u32, modifiers: Modifier, component_mode: NumComponent) -> Self {
        Self::new(OperandType::Register(reg), modifiers, component_mode)
    }

    pub fn input(reg: u32, modifiers: Modifier, component_mode: NumComponent) -> Self {
        Self::new(OperandType::Input(reg), modifiers, component_mode)
    }

    pub fn output(reg: u32, modifiers: Modifier, component_mode: NumComponent) -> Self {
        Self::new(OperandType::Output(reg), modifiers, component_mode)
    }

    fn encode(&self, module: &mut DxbcModule) {
        match &self.ty {
            &OperandType::Register(reg) => {
                module.write_operand(D3D10_SB_OPERAND_TYPE_TEMP, self.modifiers, self.component_mode, &[Immediate::U32(reg)])
            },
            &OperandType::Input(reg) => {
                module.write_operand(D3D10_SB_OPERAND_TYPE_INPUT, self.modifiers, self.component_mode, &[Immediate::U32(reg)])
            },
            &OperandType::Output(reg) => {
                module.write_operand(D3D10_SB_OPERAND_TYPE_OUTPUT, self.modifiers, self.component_mode, &[Immediate::U32(reg)])
            },
            OperandType::Imm32(imm) => {},
            OperandType::Imm32x2(imm0, imm1) => {},
            OperandType::Imm32x3(imm0, imm1, imm2) => {},
            OperandType::Imm32x4(imm0, imm1, imm2, imm3) => {},
            OperandType::Resource(reg) => {},
            OperandType::Sampler(reg) => {},
            OperandType::IndexableRegister(reg, index) => {},
            OperandType::ConstantBuffer(reg, index) => {},
            OperandType::CustomData(data) => {},
        }
    }

    fn get_type(&self) -> u32 {
        match &self.ty {
            &OperandType::Register(..) => D3D10_SB_OPERAND_TYPE_TEMP,
            &OperandType::Input(..) => D3D10_SB_OPERAND_TYPE_INPUT,
            &OperandType::Output(..) => D3D10_SB_OPERAND_TYPE_OUTPUT,
            _ => 100000,
        }
    }
}

pub struct ShexChunk {
    instructions: Vec<Instruction>,
}

impl ShexChunk {
    pub fn new() -> Self {
        ShexChunk {
            instructions: Vec::new(),
        }
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }
}
