pub fn fill_random<T, const N: usize>(slice: &mut [T; N])
where
    rand::distr::StandardUniform: rand::distr::Distribution<T>,
{
    for item in slice.iter_mut() {
        *item = rand::random();
    }
}

pub fn blank_rgba(width: u32, height: u32) -> Vec<u8> {
    let image_size_bytes: usize = (width * height * 4) as usize;
    assert!(
        image_size_bytes > 0,
        "Image must have side lengths greater than zero!"
    );

    let mut rgba = vec![0u8; image_size_bytes];
    for idx in (3..image_size_bytes).step_by(4) {
        rgba[idx] = 255u8;
    }

    return rgba;
}

pub fn blank_fixed_rgba<const BYTES: usize>() -> Box<[u8; BYTES]> {
    assert!(BYTES > 0, "Image must have side lengths greater than zero!");
    assert!(
        BYTES % 4 == 0,
        "RGBA image byte size must be divisible by four!"
    );

    let mut rgba: Box<[u8; BYTES]> = vec![0u8; BYTES]
        .into_boxed_slice()
        .try_into()
        .expect("incorrect buffer size");

    for idx in (3..BYTES).step_by(4) {
        rgba[idx] = 255u8;
    }

    return rgba;
}
