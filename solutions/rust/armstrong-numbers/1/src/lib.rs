pub fn is_armstrong_number(num: u32) -> bool {
    let mut number: u32 = num;
    let mut result: u32 = 0;
    let length: u32 = num.to_string().len().try_into().unwrap();
    return loop {
        let (quotient, remainder) = (number / 10, number % 10);
        result = result + remainder.pow(length);
        if quotient == 0 {
            break num == result;
        } else {
            number = quotient;
        }
    };
}
