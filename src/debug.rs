
pub fn debug_hex_output(buf: &Vec<u32>) {
    for op in buf {
        let hex = format!("{:x}", *op);
        let letters: [u8; 4] = unsafe { std::mem::transmute(op.to_le()) };
        let ascii = letters.map(|v| v as char);
        print!(
            "0x{} - {}{}{}{}\n",
            hex, ascii[0], ascii[1], ascii[2], ascii[3],
        );
    }
}