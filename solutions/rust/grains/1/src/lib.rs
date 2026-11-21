pub fn square(s: u32) -> u64 {
    if s == 0 || s > 64 {
        panic!("Square must be between 1 and 64");
    }

    let mut result: u64 = 1;
    if s > 1 {
        for _ in 2..(s + 1) {
            result = result * 2;
        }
    }

    return result;
}

pub fn total() -> u64 {
    let mut result: u64 = 0;
    let mut i: u32 = 64;

    while i > 0 {
        result += square(i);
        i -= 1;
    }

    return result;
}
