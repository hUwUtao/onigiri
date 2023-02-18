use rand::Rng;
pub fn sig(n: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng(); // create a random number generator
    std::iter::repeat_with(|| rng.gen::<u8>()).take(n).collect()
}

pub fn xor(a : &Vec<u8>, b:&Vec<u8>) -> Vec<u8>{
    let mut result = Vec::new();
    // cycle returns an iterator that endlessly repeats the elements of b
    // take returns an iterator that yields only the first n elements of the iterator
    let n = a.len();
    for (x, y) in a.iter().zip(b.iter().cycle().take(n)) {
        result.push(x ^ y);
    }
    result
}