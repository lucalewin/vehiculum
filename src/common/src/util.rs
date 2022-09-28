
pub fn vec_write_to_slice(src: &mut Vec<u8>, dst: &mut [u8]) -> std::io::Result<usize> {
    if dst.len() == src.len() {
        dst.copy_from_slice(&src);
        src.clear();
        Ok(dst.len())
    } else if dst.len() > src.len() {
        dst[..src.len()].copy_from_slice(&src);
        let length = src.len();
        src.clear();
        Ok(length)
    } else {
        dst.copy_from_slice(&src[..dst.len()]);
        src.drain(0..dst.len());
        Ok(dst.len())
    }
}
