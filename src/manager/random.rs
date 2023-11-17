use rand::Rng;

pub fn int_randGen() -> i32 {
    let mut rang = rand::thread_rng();

    return rang.gen_range(1..i32::MAX);
}
