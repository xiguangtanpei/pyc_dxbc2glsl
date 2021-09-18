extern crate bitflags;
extern crate byteorder;
extern crate rspirv;
extern crate spirv_headers as spirv;
extern crate dxbc;

use rspirv::binary::{Consumer, ParseAction};
use rspirv::mr;
use rspirv::sr;

use dxbc::dr;

use std::slice;

#[derive(Debug, Copy, Clone)]
pub enum TargetVersion {
    V5_0,
    // V5_1,
}

#[derive(Debug)]
pub struct TranslateOptions<'a> {
    pub entrypoint: &'a str,
    pub target: TargetVersion,
}

#[derive(Debug, Clone)]
enum ImageDepth {}

#[derive(Debug, Clone)]
enum SampleMode {}

#[derive(Debug, Copy, Clone)]
struct Bool;
#[derive(Debug, Copy, Clone)]
enum Integer {
    Int16,
    Int32,
    Int64,
    Uint16,
    Uint32,
    Uint64,
}
#[derive(Debug, Copy, Clone)]
enum Float {
    Float16,
    Float32,
    Float64,
}
#[derive(Debug, Clone)]
struct Vector {
    ty: Scalar,
    count: u32,
}
#[derive(Debug, Clone)]
struct Matrix {
    ty: Vector,
    count: u32,
}
#[derive(Debug, Clone)]
struct Array {
    ty: Box<Ty>,
}
#[derive(Debug, Clone)]
struct Structure {
    members: Vec<Ty>
}
#[derive(Debug, Clone)]
struct Image {
    dim: spirv::Dim,
    depth: ImageDepth,
    arrayed: bool,
    multi_sampled: bool,
    sampled: SampleMode,
}
#[derive(Debug, Clone)]
struct Sampler;
#[derive(Debug, Clone)]
struct SampledImage {
    image: Image,
}
#[derive(Debug, Clone)]
struct Pointer {
    storage_class: spirv::StorageClass,
    ty: Box<Ty>,
}

#[derive(Debug, Clone)]
enum Scalar {
    Numerical(Numerical),
    Bool,
}
#[derive(Debug, Clone)]
enum Numerical {
    Integer(Integer),
    Float(Float),
}
#[derive(Debug, Clone)]
enum Aggregate {
    Structure(Structure),
    Array(Array),
}
#[derive(Debug, Clone)]
enum Composite {
    Aggregate(Aggregate),
    Matrix(Matrix),
    Vector(Vector),
}
#[derive(Debug, Clone)]
enum Abstract {
    Void,
    Bool,
    Pointer(Pointer)
}

#[derive(Debug, Clone)]
enum Ty {
    Void,
    Bool,
    Integer(Integer),
    Float(Float),
    Vector(Vector),
    Matrix(Matrix),
    Array(Array),
    Structure(Structure),
    Image(Image),
    Sampler(Sampler),
    SampledImage(SampledImage),
    Aggregate(Aggregate),
    Composite(Composite),
    Pointer(Pointer),
}

impl Ty {
    fn scalar(&self) -> Option<Scalar> {
        match self {
            &Ty::Integer(int) => Some(Scalar::Numerical(Numerical::Integer(int))),
            &Ty::Float(flt) => Some(Scalar::Numerical(Numerical::Float(flt))),
            &Ty::Bool => Some(Scalar::Bool),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Metadata {
    // holds a (optional) Vec of decorations for each result id
    decorations: Vec<Option<Vec<sr::Decoration>>>,
    // holds a (optional) Vec of types for each result id
    types: Vec<Option<Ty>>,
}

impl Metadata {
    fn conv_types(module: &mr::Module) -> Vec<Option<Ty>> {
        let upper_bound = module.header.as_ref().map(|h| h.bound as usize).unwrap();
        let mut types = vec![None; upper_bound];

        for instr in &module.types_global_values {
            let result_id = instr.result_id.unwrap();

            match instr.class.opcode {
                spirv::Op::TypeVoid => {
                    types[result_id as usize] = Some(Ty::Void);
                }
                spirv::Op::TypeInt => {
                    // TODO: sign
                    types[result_id as usize] = Some(Ty::Integer(Integer::Int32));
                }
                spirv::Op::TypeFloat => {
                    types[result_id as usize] = Some(Ty::Float(Float::Float32));
                }
                spirv::Op::TypeVector => {
                    let id = if let mr::Operand::IdRef(id) = instr.operands[0] {
                        id
                    } else {
                        unimplemented!()
                    };
                    let count = if let mr::Operand::LiteralInt32(count) = instr.operands[1] {
                        count
                    } else {
                        unimplemented!()
                    };
                    let ty = types[id as usize].clone().unwrap().scalar().unwrap();
                    types[result_id as usize] = Some(Ty::Vector(Vector { ty, count }));
                }
                spirv::Op::TypeMatrix => {
                }
                spirv::Op::TypePointer => {
                    let storage_class = if let mr::Operand::StorageClass(class) = instr.operands[0] {
                        class
                    } else {
                        unimplemented!()
                    };
                    let id = if let mr::Operand::IdRef(id) = instr.operands[1] {
                        id
                    } else {
                        unimplemented!()
                    };
                    let ty = Box::new(types[id as usize].clone().unwrap());
                    types[result_id as usize] = Some(Ty::Pointer(Pointer { storage_class, ty }));
                }
                _ => {}
            }
        }

        types
    }

    fn conv_decorations(module: &mr::Module) -> Vec<Option<Vec<sr::Decoration>>> {
        let upper_bound = module.header.as_ref().map(|h| h.bound as usize).unwrap();
        let mut decorations: Vec<Option<Vec<sr::Decoration>>> = vec![None; upper_bound];

        for inst in &module.annotations {
            if inst.class.opcode != spirv::Op::Decorate {
                continue;
            }

            let id = if let mr::Operand::IdRef(id) = inst.operands[0] {
                id
            } else {
                unimplemented!()
            };

            let decoration = if let mr::Operand::Decoration(decoration) = inst.operands[1] {
                decoration
            } else {
                unimplemented!()
            };

            let decoration = match decoration {
                spirv::Decoration::RelaxedPrecision => sr::Decoration::RelaxedPrecision,
                spirv::Decoration::Block => sr::Decoration::Block,
                spirv::Decoration::BufferBlock => sr::Decoration::BufferBlock,
                spirv::Decoration::RowMajor => sr::Decoration::RowMajor,
                spirv::Decoration::ColMajor => sr::Decoration::ColMajor,
                spirv::Decoration::GLSLShared => sr::Decoration::GLSLShared,
                spirv::Decoration::GLSLPacked => sr::Decoration::GLSLPacked,
                spirv::Decoration::CPacked => sr::Decoration::CPacked,
                spirv::Decoration::NoPerspective => sr::Decoration::NoPerspective,
                spirv::Decoration::Flat => sr::Decoration::Flat,
                spirv::Decoration::Patch => sr::Decoration::Patch,
                spirv::Decoration::Centroid => sr::Decoration::Centroid,
                spirv::Decoration::Sample => sr::Decoration::Sample,
                spirv::Decoration::Invariant => sr::Decoration::Invariant,
                spirv::Decoration::Restrict => sr::Decoration::Restrict,
                spirv::Decoration::Aliased => sr::Decoration::Aliased,
                spirv::Decoration::Volatile => sr::Decoration::Volatile,
                spirv::Decoration::Constant => sr::Decoration::Constant,
                spirv::Decoration::Coherent => sr::Decoration::Coherent,
                spirv::Decoration::NonReadable => sr::Decoration::NonReadable,
                spirv::Decoration::Uniform => sr::Decoration::Uniform,
                spirv::Decoration::SaturatedConversion => sr::Decoration::SaturatedConversion,
                spirv::Decoration::NoContraction => sr::Decoration::NoContraction,
                spirv::Decoration::ExplicitInterpAMD => sr::Decoration::ExplicitInterpAMD,
                spirv::Decoration::OverrideCoverageNV => sr::Decoration::OverrideCoverageNV,
                spirv::Decoration::PassthroughNV => sr::Decoration::PassthroughNV,
                spirv::Decoration::ViewportRelativeNV => sr::Decoration::ViewportRelativeNV,
                spirv::Decoration::NonUniformEXT => sr::Decoration::NonUniformEXT,
                spirv::Decoration::BuiltIn => {
                    let builtin = if let mr::Operand::BuiltIn(builtin) = inst.operands[2] {
                        builtin
                    } else {
                        unimplemented!()
                    };

                    sr::Decoration::BuiltIn(builtin)
                }
                spirv::Decoration::Location => {
                    let loc = if let mr::Operand::LiteralInt32(loc) = inst.operands[2] {
                        loc
                    } else {
                        unimplemented!()
                    };

                    sr::Decoration::Location(loc)
                }

                // TODO:
                _ => unimplemented!()
            };

            let mut decorations = &mut decorations[id as usize];
            if let Some(decorations) = decorations {
                decorations.push(decoration);
            } else {
                *decorations = Some(vec![decoration]);
            }
        }

        decorations
    }

    fn from_module(module: &mr::Module) -> Self {
        let decorations = Self::conv_decorations(module);
        let types = Self::conv_types(module);

        Metadata {
            decorations,
            types,
        }
    }

    fn get_decorations(&self, id: u32) -> &[sr::Decoration] {
        if let Some(decorations) = &self.decorations[id as usize] {
            &decorations
        } else {
            &[]
        }
    }

    fn get_type(&self, id: u32) -> Option<&Ty> {
        self.types[id as usize].as_ref()
    }
}

pub struct SpirvModule {
    module: rspirv::mr::Module,
    meta: Metadata,
}

impl SpirvModule {
    fn conv_variable(&self, ty: &Ty, type_id: u32) -> dr::InputOutputElement {
        let (component_type, component_count) = match ty {
            Ty::Vector(Vector { ty, count }) => {
                match ty {
                    Scalar::Numerical(Numerical::Integer(Integer::Uint32)) => (dr::RegisterComponentType::Uint32, count),
                    Scalar::Numerical(Numerical::Integer(Integer::Int32)) => (dr::RegisterComponentType::Int32, count),
                    Scalar::Numerical(Numerical::Float(Float::Float32)) => (dr::RegisterComponentType::Float32, count),
                    _ => unimplemented!()
                }
            }
            _ => unimplemented!()
        };

        let mut elem = dr::InputOutputElement {
            name: String::from("TEXCOORD"),
            semantic_index: 0,
            semantic_type: dr::SemanticName::Undefined,
            component_type,
            register: 0,
            component_mask: 0,
            rw_mask: 0,
        };

        for decoration in self.meta.get_decorations(type_id) {
            match decoration {
                sr::Decoration::BuiltIn(builtin) => {
                    match builtin {
                        spirv::BuiltIn::Position => {
                            elem.semantic_type = dr::SemanticName::Position;
                        }
                        // TODO:
                        _ => {}
                    }
                },
                &sr::Decoration::Location(location) => {
                    // TODO:
                    elem.semantic_index = location;
                }
                _ => {}
            }
        }

        println!("{:#?}", elem);

        elem
    }

    // TODO: result
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut loader = rspirv::mr::Loader::new();
        rspirv::binary::parse_bytes(bytes, &mut loader).unwrap();

        let module = loader.module();
        let meta = Metadata::from_module(&module);

        println!("{:#?}", meta);
        SpirvModule {
            module,
            meta,
        }
    }

    fn find_function(&self, id: &mr::Operand) -> Option<&mr::Function> {
        let id = if let &mr::Operand::IdRef(id) = id {
            id
        } else {
            return None;
        };

        self.module.functions.iter().find(|f| {
            if let Some(def) = &f.def {
                if let Some(result_id) = def.result_id {
                    result_id == id
                } else {
                    false
                }
            } else {
                false
            }
        })
    }

    fn add_io_element(&self, inputs: &mut Vec<dr::InputOutputElement>, outputs: &mut Vec<dr::InputOutputElement>, variable_id: u32) {
        for decl in &self.module.types_global_values {
            if decl.class.opcode == spirv::Op::Variable {
                let id = if let Some(id) = decl.result_id {
                    id
                } else {
                    unimplemented!()
                };

                if variable_id != id { continue; }

                let ty_id = if let Some(id) = decl.result_type {
                    id
                } else {
                    unimplemented!()
                };
                let ty = self.meta.get_type(ty_id).unwrap();

                if let Ty::Pointer(Pointer { storage_class, ty }) = &ty {
                    let elem = self.conv_variable(&ty, id);

                    match storage_class {
                        spirv::StorageClass::Input => { inputs.push(elem); }
                        spirv::StorageClass::Output => { outputs.push(elem); }
                        _ => {}
                    }
                }
            }
        }
    }

    fn get_iosgn(&self, entrypoint: &mr::Instruction, globals: &[mr::Instruction], decorations: &[mr::Instruction]) -> (dr::IOsgnChunk, dr::IOsgnChunk) {
        let mut isgn = dr::IOsgnChunk {
            elements: Vec::new()
        };
        let mut osgn = dr::IOsgnChunk {
            elements: Vec::new()
        };

        // go through all inputs/outputs of the entrypoint and append to our
        // chunks
        for operand in &entrypoint.operands[3..] {
            match operand {
                &mr::Operand::IdRef(id) => {
                    self.add_io_element(
                        &mut isgn.elements,
                        &mut osgn.elements,
                        id
                    );
                }
                _ => {}
            }
        }

        (isgn, osgn)
    }

    pub fn translate_entrypoint(&self, entrypoint: &str, target: TargetVersion) -> Vec<u32> {
        let entrypoint = self.module.entry_points.iter().find(|e| {
            if let mr::Operand::LiteralString(ref name) = e.operands[2] {
                entrypoint == name
            } else {
                false
            }
        }).unwrap();

        let function = self.find_function(&entrypoint.operands[1]).unwrap();

        println!("{:#?}", function.basic_blocks);

        let mut builder = dr::Builder::new();

        builder.set_rdef(dr::RdefChunk {
            constant_buffers: Vec::new(),
            resource_bindings: Vec::new(),
            shader_ty: 1,
            minor: 0,
            major: 5,
            flags: 0,
            author: &"DXBCross 0",
            rd11: Some([0u32; 7]),
        });

        let (isgn, osgn) = self.get_iosgn(entrypoint, &self.module.types_global_values, &self.module.annotations);

        builder.set_isgn(isgn);
        builder.set_osgn(osgn);

        let mut shex = dr::ShexChunk::new();
        shex.add_instruction(dr::Instruction::DclGlobalFlags {
            flags: dr::GlobalFlags::REFACTORING_ALLOWED,
        });


        builder.set_shex(shex);

        builder.module().unwrap().dwords
    }
}

