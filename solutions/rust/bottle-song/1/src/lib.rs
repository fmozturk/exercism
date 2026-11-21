pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut result = String::new();

    for i in 0..take_down {
        if start_bottles - i < 1 {
            break;
        }
        result.push_str(process_bottle(start_bottles - i).as_str());
        result.push_str("\n");
    }
    result
}

fn process_bottle(bottle_number: u32) -> String {
    let numbers: [&str; 11] = ["no", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten"];
    let mut result = String::new();

    (1..=2).for_each(|_| {
    result.push_str(format!("{} green bottle{} hanging on the wall,\n", numbers[bottle_number as usize], s(bottle_number)).as_str());
    });
    result.push_str("And if one green bottle should accidentally fall,\n");
    result.push_str(format!("There'll be {} green bottle{} hanging on the wall.\n",
        numbers[bottle_number as usize - 1].to_lowercase(),
                            s(bottle_number - 1)).as_str());
    result
}

fn s(number_of_bottles: u32) -> String {
    if number_of_bottles == 1 {
        "".to_string()
    } else {
        "s".to_string()
    }
}