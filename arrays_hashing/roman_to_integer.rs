fn roman_to_int(s: String) -> i32 {
    let mut total = 0;
    let mut last_val: i32 = 0;
    for character in s.chars() {
        let val = match character {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        };
        if val > last_val {
            total -= 2 * last_val;
        }
        last_val = val;
        total += val;
    }
    total
}
