/// Solution for: https://leetcode.com/problems/zigzag-conversion/

fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
        return s;
    }

    let mut inc: i32 = -1;
    let mut i: i32 = 0;
    let mut rows: Vec<Vec<char>> = vec![Vec::new(); num_rows as usize];
    for c in s.chars() {
        if i == 0 || i == num_rows - 1 {
            inc = -inc;
        }
        rows[i as usize].push(c);
        i += inc;
    }

    rows.iter().flatten().collect()
}
