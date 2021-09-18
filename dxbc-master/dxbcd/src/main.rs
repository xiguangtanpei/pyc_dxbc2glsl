extern crate dxbc;
extern crate term;

use dxbc::dr::*;
use dxbc::dr::shex::*;
use dxbc::binary::*;

use std::mem;

struct DisasmConsumer {
    out: Box<term::StdoutTerminal>,
    indent: u32,
}

const COMMENT_COLOR: term::color::Color = term::color::BRIGHT_BLACK;
const OPCODE_COLOR: term::color::Color = term::color::BLUE;
const IMMEDIATE_COLOR: term::color::Color = term::color::BRIGHT_BLACK;

fn get_name_token_name(mode: NameToken) -> &'static str {
    match mode {
        NameToken::Undefined => "undefined",
        NameToken::Position => "position",
        NameToken::ClipDistance => "clip_distance",
        NameToken::CullDistance => "cull_distance",
        NameToken::RenderTargetArrayIndex => "rendertarget_array_index",
        NameToken::ViewportArrayIndex => "viewport_array_index",
        NameToken::VertexId => "vertex_id",
        NameToken::PrimitiveId => "primitive_id",
        NameToken::InstanceId => "instance_id",
        NameToken::IsFrontFace => "is_front_face",
        NameToken::SampleIndex => "sampleIndex",
        _ => "TODO"
    }
}

fn get_interpolation_mode_name(mode: InterpolationMode) -> &'static str {
    match mode {
        InterpolationMode::Undefined => "undefined",
        InterpolationMode::Constant => "constant",
        InterpolationMode::Linear => "linear",
        InterpolationMode::LinearCentroid => "linear centroid",
        InterpolationMode::LinearNoPerspective => "linear noperspective",
        InterpolationMode::LinearNoPerspectiveCentroid => "linear noperspective centroid",
        InterpolationMode::LinearSample => "linear sample",
        InterpolationMode::LinearNoPerspectiveSample => "linear noperspective sample",
    }
}

fn get_test_boolean_name(test: TestBoolean) -> &'static str {
    match test {
        TestBoolean::Zero => "z",
        TestBoolean::NonZero => "nz",
    }
}

impl DisasmConsumer {
    fn new() -> Self {
        Self {
            out: term::stdout().unwrap(),
            indent: 0,
        }
    }

    fn begin_instruction<'a>(&mut self, opcode: OpcodeToken0<'a>, offset: u32, instruction: &str) {
        self.out.fg(COMMENT_COLOR).unwrap();
        write!(self.out, "{:#08x}: ", offset).unwrap();
        self.out.reset().unwrap();

        if self.indent > 0 {
            write!(self.out, "{}", "  ".repeat(self.indent as usize)).unwrap();
        }

        self.out.fg(OPCODE_COLOR).unwrap();
        write!(self.out, "{}", instruction).unwrap();
    }

    fn end_instruction(&mut self) {
        self.out.reset().unwrap();

        write!(self.out, " ").unwrap();
    }

    fn write_instruction<'a>(&mut self, opcode: OpcodeToken0<'a>, offset: u32, instruction: &str) {
        self.begin_instruction(opcode, offset, instruction);
        //if opcode.is_saturated() {
        //    write!(self.out, "_sat").unwrap();
        //}

        let mut ex = opcode.get_extended_opcode();
        while let Some(opcode) = ex {
            // TODO:
            ex = opcode.get_extended_opcode();
        }

        self.end_instruction();
    }

    fn write_resource_return_type<'a>(&mut self, opcode: OpcodeToken0<'a>, return_type: ResourceReturnTypeToken0<'a>) {

        write!(self.out, "(").unwrap();
        write!(self.out, "{:?}, ", return_type.get_return_type(ComponentName::X)).unwrap();
        write!(self.out, "{:?}, ", return_type.get_return_type(ComponentName::Y)).unwrap();
        write!(self.out, "{:?}, ", return_type.get_return_type(ComponentName::Z)).unwrap();
        write!(self.out, "{:?}", return_type.get_return_type(ComponentName::W)).unwrap();
        write!(self.out, ")").unwrap();
    }

    fn write_mask(&mut self, mask: ComponentMask) {
        if mask.contains(ComponentMask::COMPONENT_MASK_R) {
            write!(self.out, "x").unwrap();
        }
        if mask.contains(ComponentMask::COMPONENT_MASK_G) {
            write!(self.out, "y").unwrap();
        }
        if mask.contains(ComponentMask::COMPONENT_MASK_B) {
            write!(self.out, "z").unwrap();
        }
        if mask.contains(ComponentMask::COMPONENT_MASK_A) {
            write!(self.out, "w").unwrap();
        }
    }

    fn write_immediate<'a>(&mut self, imm: Immediate<'a>) {
        match imm {
            Immediate::U32(val) => { write!(self.out, "{}", val).unwrap(); },
            Immediate::U64(val) => { write!(self.out, "{}", val).unwrap(); },
            Immediate::Relative(operand) => {
                self.write_operand(&operand);
            },
            Immediate::U32Relative(val, operand) => {
                write!(self.out, "{} + ", val).unwrap();
                self.write_operand(&operand);
            },
            Immediate::U64Relative(val, operand) => {
                write!(self.out, "{} + ", val).unwrap();
                self.write_operand(&operand);
            },
        }
    }

    fn write_operands<'a>(&mut self, operands: &[OperandToken0<'a>]) {
        let len = operands.len();

        for (idx, operand) in operands.iter().enumerate() {
            self.write_operand(operand);
            if idx + 1 != len {
                write!(self.out, ", ").unwrap();
            }
        }

        writeln!(self.out, "").unwrap();
    }

    fn write_operand<'a>(&mut self, operand: &OperandToken0<'a>) {
        let ty = operand.get_operand_type();

        match ty {
            OperandType::Immediate32 | OperandType::Immediate64 => {
                if let OperandType::Immediate32 = ty {
                    write!(self.out, "l(").unwrap();
                } else {
                    write!(self.out, "d(").unwrap();
                }

                self.out.fg(IMMEDIATE_COLOR).unwrap();
                let mut literals = Vec::new();
                let immediates = operand.get_immediates();
                for imm in immediates {
                    match imm {
                        Immediate::U32(val) => {
                            literals.push(format!("{:.6}", f32::from_bits(val)));
                        },
                        Immediate::U64(val) => {
                            literals.push(format!("{:.6}", f64::from_bits(val)));
                        }
                        _ => {}
                    }
                }
                write!(self.out, "{}", literals.join(", ")).unwrap();
                self.out.reset().unwrap();

                write!(self.out, ")").unwrap();
            }
            _ => {

            }
        }

        if let Some(operand) = operand.get_extended_operand() {
            match operand.get_operand_modifier() {
                OperandModifier::None => {},
                OperandModifier::Neg => {
                    write!(self.out, "-").unwrap();
                },
                OperandModifier::Abs => {
                    write!(self.out, "|").unwrap();
                },
                OperandModifier::AbsNeg => {
                    write!(self.out, "-|").unwrap();
                },
            }
        }

        let prefix = match ty {
            OperandType::Temp => "r",
            OperandType::Input => "v",
            OperandType::Output => "o",
            OperandType::Resource => "t",
            OperandType::Sampler => "s",
            OperandType::ConstantBuffer => "cb",

            OperandType::Immediate32 | OperandType::Immediate64 => { return; }
            _ => ""
        };

        write!(self.out, "{}", prefix).unwrap();

        let dim = operand.get_index_dimension();

        match dim {
            IndexDimension::D1 => {
                self.write_immediate(operand.get_immediate(0));
            },
            IndexDimension::D2 => {
                self.write_immediate(operand.get_immediate(0));
                write!(self.out, "[").unwrap();
                self.write_immediate(operand.get_immediate(1));
                write!(self.out, "]").unwrap();
            },
            _ => {}
        }
        /*match immediate {
            Immediate::U32(vals) => {
                match vals.len() {
                    1 => {
                        write!(self.out, "{}", vals[0]).unwrap();
                    },
                    2 => {
                        write!(self.out, "{}[{}]", vals[0], vals[1]).unwrap();
                    },
                    _ => {}
                }
            },
            _ => {}
        }*/

        fn write_swizzle_component(disasm: &mut DisasmConsumer, val: ComponentName) {
            match val {
                ComponentName::X => {
                    disasm.out.fg(term::color::RED).unwrap();
                    write!(disasm.out, "x").unwrap();
                    disasm.out.reset().unwrap();
                }
                ComponentName::Y => {
                    disasm.out.fg(term::color::GREEN).unwrap();
                    write!(disasm.out, "y").unwrap();
                    disasm.out.reset().unwrap();
                }
                ComponentName::Z => {
                    disasm.out.fg(term::color::CYAN).unwrap();
                    write!(disasm.out, "z").unwrap();
                    disasm.out.reset().unwrap();
                }
                ComponentName::W => {
                    disasm.out.fg(term::color::WHITE).unwrap();
                    write!(disasm.out, "w").unwrap();
                    disasm.out.reset().unwrap();
                }
            }
        }

        match operand.get_component_select_mode() {
            ComponentSelectMode::Mask => {
                let mask = operand.get_component_mask();

                if !mask.is_empty() {
                    write!(self.out, ".").unwrap();
                }

                self.out.fg(term::color::RED).unwrap();
                if mask.contains(ComponentMask::COMPONENT_MASK_R) {
                    write!(self.out, "x").unwrap();
                }
                self.out.fg(term::color::GREEN).unwrap();
                if mask.contains(ComponentMask::COMPONENT_MASK_G) {
                    write!(self.out, "y").unwrap();
                }
                self.out.fg(term::color::CYAN).unwrap();
                if mask.contains(ComponentMask::COMPONENT_MASK_B) {
                    write!(self.out, "z").unwrap();
                }
                self.out.fg(term::color::WHITE).unwrap();
                if mask.contains(ComponentMask::COMPONENT_MASK_A) {
                    write!(self.out, "w").unwrap();
                }
                self.out.reset().unwrap();
            }
            ComponentSelectMode::Swizzle => {
                write!(self.out, ".").unwrap();

                let swizzle = operand.get_component_swizzle();

                write_swizzle_component(self, swizzle.0);
                write_swizzle_component(self, swizzle.1);
                write_swizzle_component(self, swizzle.2);
                write_swizzle_component(self, swizzle.3);
            }
            ComponentSelectMode::Select1 => {
                write!(self.out, ".").unwrap();

                let swizzle = operand.get_component_swizzle();
                write_swizzle_component(self, swizzle.0);
            }
        }

        if let Some(operand) = operand.get_extended_operand() {
            match operand.get_operand_modifier() {
                OperandModifier::None => {},
                OperandModifier::Neg => {},
                OperandModifier::Abs | OperandModifier::AbsNeg => {
                    write!(self.out, "|").unwrap();
                },
            }
        }
    }
}

impl Consumer for DisasmConsumer {
    fn initialize(&mut self) -> Action {
        self.out.fg(term::color::WHITE).unwrap();

        Action::Continue
    }

    fn finalize(&mut self) -> Action {
        self.out.reset().unwrap();

        Action::Continue
    }

    fn consume_header(&mut self, header: &dxbc::dr::DxbcHeader) -> Action {
        Action::Continue
    }

    fn consume_rdef(&mut self, rdef: &dxbc::dr::RdefChunk) -> Action {
        self.out.fg(COMMENT_COLOR).unwrap();
        writeln!(self.out, "//").unwrap();
        writeln!(self.out, "// Generated by {}", rdef.author).unwrap();
        writeln!(self.out, "//").unwrap();
        writeln!(self.out, "//").unwrap();
        writeln!(self.out, "// Buffer Definitions:").unwrap();

        writeln!(self.out, "//").unwrap();
        for cb in &rdef.constant_buffers {
            writeln!(self.out, "// cbuffer {}", cb.name).unwrap();
            writeln!(self.out, "// {{").unwrap();
            writeln!(self.out, "// }}").unwrap();
        }
        writeln!(self.out, "//").unwrap();

        writeln!(self.out, "//").unwrap();
        writeln!(self.out, "// Resource Bindings:").unwrap();
        writeln!(self.out, "//").unwrap();
        writeln!(self.out, "// Name                                 Type  Format         Dim      HLSL Bind  Count").unwrap();
        writeln!(self.out, "// ------------------------------ ---------- ------- ----------- -------------- ------").unwrap();

        for bind in &rdef.resource_bindings {
            // writeln!(self.out, "// {:30} {:10}", bind.name, return_type, bind.).unwrap();
        }
        writeln!(self.out, "//").unwrap();
        writeln!(self.out, "//").unwrap();

        self.out.reset().unwrap();

        Action::Continue
    }

    fn consume_isgn(&mut self, isgn: &dxbc::dr::IOsgnChunk) -> Action {
        self.out.fg(COMMENT_COLOR).unwrap();

        writeln!(self.out, "//").unwrap();
        writeln!(self.out, "// Input signature:").unwrap();
        writeln!(self.out, "//").unwrap();
        writeln!(self.out, "// Name                 Index   Mask Register SysValue  Format   Used").unwrap();
        writeln!(self.out, "// -------------------- ----- ------ -------- -------- ------- ------").unwrap();

        for elem in &isgn.elements {
            writeln!(
                self.out,
                "// {:20} {:5} {:6} {:8} {:8?} {:7} {:6}",
                elem.name,
                elem.semantic_index,
                elem.component_mask,
                elem.register,
                elem.semantic_type,
                match elem.component_type {
                    RegisterComponentType::Unknown => "NONE",
                    RegisterComponentType::Uint32 => "uint",
                    RegisterComponentType::Int32 => "int",
                    RegisterComponentType::Float32 => "float",
                },
                elem.rw_mask,
            ).unwrap();
        }
        writeln!(self.out, "//").unwrap();
        writeln!(self.out, "//").unwrap();

        self.out.reset().unwrap();

        Action::Continue
    }

    fn consume_osgn(&mut self, osgn: &dxbc::dr::IOsgnChunk) -> Action {
        self.out.fg(COMMENT_COLOR).unwrap();

        writeln!(self.out, "//").unwrap();
        writeln!(self.out, "// Output signature:").unwrap();
        writeln!(self.out, "//").unwrap();
        writeln!(self.out, "// Name                 Index   Mask Register SysValue  Format   Used").unwrap();
        writeln!(self.out, "// -------------------- ----- ------ -------- -------- ------- ------").unwrap();

        for elem in &osgn.elements {
            writeln!(
                self.out,
                "// {:20} {:5} {:6} {:8} {:8?} {:7} {:6}",
                elem.name,
                elem.semantic_index,
                elem.component_mask,
                elem.register,
                elem.semantic_type,
                match elem.component_type {
                    RegisterComponentType::Unknown => "NONE",
                    RegisterComponentType::Uint32 => "uint",
                    RegisterComponentType::Int32 => "int",
                    RegisterComponentType::Float32 => "float",
                },
                elem.rw_mask,
            ).unwrap();
        }
        writeln!(self.out, "//").unwrap();
        writeln!(self.out, "//").unwrap();

        self.out.reset().unwrap();

        Action::Continue
    }

    fn consume_shex(&mut self, osgn: &dxbc::dr::ShexHeader) -> Action {

        Action::Continue
    }

    fn consume_instruction(&mut self, offset: u32, instruction: dxbc::dr::SparseInstruction) -> Action {
        use dxbc::dr::Operands::*;

        let opcode = instruction.opcode;

        match instruction.operands {
            DclGlobalFlags(flags) => {
                self.write_instruction(opcode, offset, "dcl_globalFlags");

                if flags.is_refactoring_allowed() {
                    write!(self.out, "{}", "refactoringAllowed").unwrap();
                }
                writeln!(self.out, "").unwrap();
            }
            DclInput(input) => {
                self.write_instruction(opcode, offset, "dcl_input");
                match input.operand.get_operand_type() {

                    OperandType::Input => { write!(self.out, "v{}.", input.get_input_register()).unwrap(); }
                    OperandType::InputCoverageMask => {
                        write!(self.out, "vCoverage").unwrap();
                    }
                    _ => { write!(self.out, "TODO").unwrap(); }
                };
                self.write_mask(input.operand.get_component_mask());
                writeln!(self.out, "").unwrap();
            }
            DclInputPs(input) => {
                self.write_instruction(opcode, offset, "dcl_input_ps");

                write!(self.out, "{} ", get_interpolation_mode_name(opcode.get_interpolation_mode())).unwrap();

                match input.operand.get_operand_type() {

                    OperandType::Input => { write!(self.out, "v{}.", input.get_input_register()).unwrap(); }
                    OperandType::InputCoverageMask => {
                        write!(self.out, "vCoverage").unwrap();
                    }
                    _ => { write!(self.out, "TODO").unwrap(); }
                };
                self.write_mask(input.operand.get_component_mask());
                writeln!(self.out, "").unwrap();
            }
            DclInputPsSiv(input) => {
                self.write_instruction(opcode, offset, "dcl_input_ps_siv");
                write!(self.out, "{} ", get_interpolation_mode_name(opcode.get_interpolation_mode())).unwrap();
                match input.operand.get_operand_type() {

                    OperandType::Input => { write!(self.out, "v{}.", input.get_input_register()).unwrap(); }
                    OperandType::InputCoverageMask => {
                        write!(self.out, "vCoverage").unwrap();
                    }
                    _ => { write!(self.out, "TODO").unwrap(); }
                };
                self.write_mask(input.operand.get_component_mask());

                write!(self.out, " {}", get_name_token_name(input.get_system_name())).unwrap();

                writeln!(self.out, "").unwrap();
            }
            DclInputPsSgv(input) => {
                self.write_instruction(opcode, offset, "dcl_input_ps_sgv");
                write!(self.out, "{} ", get_interpolation_mode_name(opcode.get_interpolation_mode())).unwrap();
                match input.operand.get_operand_type() {

                    OperandType::Input => { write!(self.out, "v{}.", input.get_input_register()).unwrap(); }
                    OperandType::InputCoverageMask => {
                        write!(self.out, "vCoverage").unwrap();
                    }
                    _ => { write!(self.out, "TODO").unwrap(); }
                };
                self.write_mask(input.operand.get_component_mask());

                write!(self.out, " {}", get_name_token_name(input.get_system_name())).unwrap();
                writeln!(self.out, "").unwrap();
            }
            DclOutput(output) => {
                self.write_instruction(opcode, offset, "dcl_output");
                write!(self.out, "o{}.", output.get_output_register()).unwrap();
                self.write_mask(output.operand.get_component_mask());
                writeln!(self.out, "").unwrap();
            }
            DclConstantBuffer(cb) => {
                self.write_instruction(opcode, offset, "dcl_constantbuffer");

                writeln!(self.out, "CB{}[{}], {:?}", cb.get_binding(), cb.get_size(), cb.get_access_pattern()).unwrap();
            }
            DclResource(resource) => {
                self.begin_instruction(opcode, offset, "dcl_resource");
                write!(self.out, "{}", match opcode.get_resource_dimension() {
                    ResourceDimension::Texture1D => "_texture1d",
                    ResourceDimension::Texture2D => "_texture2d",
                    ResourceDimension::Texture3D => "_texture3d",
                    ResourceDimension::TextureCube => "_texturecube",
                    ResourceDimension::Texture2DMS => "_texture2dms",
                    _ => "",
                }).unwrap();
                self.end_instruction();

                // TODO: resource dim

                self.write_resource_return_type(opcode, resource.return_type);
                writeln!(self.out, " t{}", resource.get_register()).unwrap();
            }
            DclSampler(sampler) => {
                self.write_instruction(opcode, offset, "dcl_sampler");

                write!(self.out, "s{}, ", sampler.get_register()).unwrap();
                writeln!(self.out, "{:?}", opcode.get_sampler_mode()).unwrap();

                // TODO: mode
            }
            DclTemps(temps) => {
                self.write_instruction(opcode, offset, "dcl_temps");

                writeln!(self.out, "{}", temps.register_count).unwrap();
            }
            DclIndexableTemp(temps) => {
                self.write_instruction(opcode, offset, "dcl_indexableTemp");

                writeln!(self.out, "X{}[{}], {}", temps.register_index, temps.register_count, temps.num_components).unwrap();
            }
            DclOutputSiv(siv) => {
                self.write_instruction(opcode, offset, "dcl_output_siv");
                write!(self.out, "o{}.", siv.get_output_register()).unwrap();
                self.write_mask(siv.operand.get_component_mask());
                writeln!(self.out, ", {:?}", siv.get_system_name()).unwrap();
            },
            Add(add) => {
                self.write_instruction(opcode, offset, "add");
                self.write_operands(&[add.dst, add.a, add.b]);
            },
            And(and) => {
                self.write_instruction(opcode, offset, "and");
                self.write_operands(&[and.dst, and.a, and.b]);
            },
            Mul(mul) => {
                self.write_instruction(opcode, offset, "mul");
                self.write_operands(&[mul.dst, mul.a, mul.b]);
            },
            Mad(mad) => {
                self.write_instruction(opcode, offset, "mad");
                self.write_operands(&[mad.dst, mad.a, mad.b, mad.c]);
            }
            Mov(mov) => {
                self.write_instruction(opcode, offset, "mov");
                self.write_operands(&[mov.dst, mov.src]);
            }
            Itof(itof) => {
                self.write_instruction(opcode, offset, "itof");
                self.write_operands(&[itof.dst, itof.src]);
            }
            Utof(utof) => {
                self.write_instruction(opcode, offset, "utof");
                self.write_operands(&[utof.dst, utof.src]);
            }
            Ftou(ftou) => {
                self.write_instruction(opcode, offset, "ftou");
                self.write_operands(&[ftou.dst, ftou.src]);
            }
            If(i) => {
                self.begin_instruction(opcode, offset, "if");
                write!(self.out, "_{}", get_test_boolean_name(opcode.get_test_type())).unwrap();
                self.end_instruction();
                self.write_operands(&[i.src]);

                self.indent += 1;
            }
            Else => {
                self.indent -= 1;
                self.write_instruction(opcode, offset, "else");
                self.indent += 1;

                writeln!(self.out, "").unwrap();
            }
            EndIf => {
                self.indent -= 1;
                self.write_instruction(opcode, offset, "endif");

                writeln!(self.out, "").unwrap();
            }

            Loop => {
                self.write_instruction(opcode, offset, "loop");
                self.indent += 1;

                writeln!(self.out, "").unwrap();
            }
            EndLoop => {
                self.indent -= 1;
                self.write_instruction(opcode, offset, "endloop");

                writeln!(self.out, "").unwrap();
            }
            Break => {
                self.write_instruction(opcode, offset, "break");

                writeln!(self.out, "").unwrap();
            }
            BreakC(breakc) => {
                self.begin_instruction(opcode, offset, "breakc");
                write!(self.out, "_{}", get_test_boolean_name(opcode.get_test_type())).unwrap();
                self.end_instruction();

                self.write_operands(&[breakc.src]);
            }
            Sample(sample) => {
                self.write_instruction(opcode, offset, "sample");

                self.write_operands(&[sample.dst, sample.src_address, sample.src_resource, sample.src_sampler]);
            }
            SampleL(sample) => {
                self.write_instruction(opcode, offset, "sample_l");

                self.write_operands(&[sample.dst, sample.src_address, sample.src_resource, sample.src_sampler, sample.src_lod]);
            }
            Ret => {
                self.write_instruction(opcode, offset, "ret");
                writeln!(self.out, "").unwrap();
            }
            _ => {
                println!("  {:?}", instruction);
            }
        }

        self.out.reset().unwrap();

        Action::Continue
    }
}

fn main() {
    let mut shader_bytes = include_bytes!("..\\assembled.dxbc");

    let start = 0x4;
    println!("Real Checksum: {:?}", unsafe { ::std::slice::from_raw_parts(&shader_bytes[start..(start+16)] as *const _ as *const u32, 4) });
    println!("???? Checksum: {:?}", dxbc::checksum(shader_bytes));

    let mut consumer = DisasmConsumer::new();
    let mut parser = Parser::new(shader_bytes, &mut consumer);

    parser.parse().unwrap();
}
