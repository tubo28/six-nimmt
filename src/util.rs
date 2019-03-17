pub fn u64_to_seed_arr(seed: u64, dst: &mut [u8; 32]) {
    for i in 0..8 {
        dst[i] = ((seed >> (i * 8)) & 0xFF) as u8;
    }
}
