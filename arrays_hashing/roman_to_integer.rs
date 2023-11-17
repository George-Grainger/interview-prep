fn roman_to_int(mut s: String) -> i32 {
    s.push('\0');
    let mut total = 0;
    let mut skip = false;
    let chars: Vec<char> = s.chars().collect();

    for pair in chars.windows(2) {
        if skip {
            skip = false;
            continue;
        }

        match pair {
            ['C', 'M'] => {
                total += 900;
                skip = true;
            }
            ['C', 'D'] => {
                total += 400;
                skip = true;
            }
            ['X', 'C'] => {
                total += 90;
                skip = true;
            }
            ['X', 'L'] => {
                total += 40;
                skip = true;
            }
            ['I', 'X'] => {
                total += 9;
                skip = true;
            }
            ['I', 'V'] => {
                total += 4;
                skip = true;
            }
            ['M', _] => total += 1000,
            ['D', _] => total += 500,
            ['C', _] => total += 100,
            ['L', _] => total += 50,
            ['X', _] => total += 10,
            ['V', _] => total += 5,
            ['I', _] => total += 1,
            _ => unreachable!("Shouldn't get here"),
        };
    }

    total
}
