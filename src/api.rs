
pub fn backend_response(ctype: &str, data: &Vec<u8>) -> ! {
    unsafe {
        asm!("out 0x0, eax",
            in("eax") 0xFFFF,
            in("rdi") ctype.as_ptr(),
            in("rsi") ctype.len(),
            in("rdx") data.as_ptr(),
            in("rcx") data.len(),
            options(noreturn)
        );
    }
}
