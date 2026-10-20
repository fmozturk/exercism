pub fn square_of_sum(n: u32) -> u32 {
    let mut result: u32 = 0;
    for num in 1..=n {
        result = result + num;
        println!("num: {}, result: {}", num, result);
    }
    return result.pow(2);
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut result: u32 = 0;
    for num in 1..=n {
        result = result + num.pow(2);
    }
    return result;
}

pub fn difference(n: u32) -> u32 {
    return square_of_sum(n) - sum_of_squares(n);
}
