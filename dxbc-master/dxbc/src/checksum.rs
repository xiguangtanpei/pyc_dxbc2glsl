use md5;

pub fn checksum(module: &[u8]) -> [u32; 4] {
    let mut cxt = md5::Context::new();

    let module = &module[0x14..];
    let bits = (module.len() * 8) as u32;
    let full_chunk_sz = module.len() as u32 & 0xffffffc0;
    cxt.consume(&module[..full_chunk_sz as usize]);

    let mut last_chunk_sz = module.len() as u32 - full_chunk_sz;
    let mut padding_sz = 64 - last_chunk_sz;
    let last_data = &module[full_chunk_sz as usize..];

    if last_chunk_sz >= 56 {
        cxt.consume(&last_data[..last_chunk_sz as usize]);
        cxt.consume(&md5::PADDING[..padding_sz as usize]);

        let mut input = [0u32; 16];
        unsafe {
            ::std::ptr::copy(cxt.input.as_ptr(), input.as_mut_ptr() as *mut u8, 64);
        }

        input[0] = bits;
        input[15] = (bits >> 2) | 1;

        md5::transform(&mut cxt.buffer, &input);
    } else {
        cxt.consume(unsafe { ::std::slice::from_raw_parts(&bits as *const u32 as *const u8, 4) });

        if last_chunk_sz != 0 {
            cxt.consume(&last_data[..last_chunk_sz as usize]);
        }

        last_chunk_sz += 4;
        padding_sz -= 4;

        let start = last_chunk_sz as usize;
        let len = start + padding_sz as usize;

        cxt.input[start..len].copy_from_slice(&md5::PADDING[..padding_sz as usize]);

        let mut input = [0u32; 16];
        unsafe {
            ::std::ptr::copy(cxt.input.as_ptr(), input.as_mut_ptr() as *mut u8, 64);
        }

        input[15] = (bits >> 2) | 1;

        md5::transform(&mut cxt.buffer, &input);
    }

    cxt.buffer
}

