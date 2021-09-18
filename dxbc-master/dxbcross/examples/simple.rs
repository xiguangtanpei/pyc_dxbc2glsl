extern crate dxbcross;
extern crate rspirv;
extern crate pretty_hex;

use rspirv::binary::Disassemble;
use pretty_hex::PrettyHex;

fn main() {
    let spirv = include_bytes!("shader.spirv");

    let module = dxbcross::SpirvModule::from_bytes(spirv);
    let dxbc = module.translate_entrypoint("vs", dxbcross::TargetVersion::V5_0);

    let mut loader = rspirv::mr::Loader::new();
    rspirv::binary::parse_bytes(&spirv[..], &mut loader).unwrap();
    let module = loader.module();

    println!("{}", module.disassemble());
    //println!("{:#?}", module);
    let bytes = unsafe { std::slice::from_raw_parts(dxbc.as_ptr() as _, dxbc.len() * 4) };
    println!("{:?}", bytes.hex_dump());

    use std::io::Write;
    use std::fs::File;
    File::create("..\\dxbcd\\assembled.dxbc").unwrap().write_all(bytes).unwrap();
}

