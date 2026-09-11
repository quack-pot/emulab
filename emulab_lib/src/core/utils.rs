pub fn fill_random<T>(slice: &mut [T])
where
    rand::distr::StandardUniform: rand::distr::Distribution<T>,
{
    for item in slice.iter_mut() {
        *item = rand::random();
    }
}
