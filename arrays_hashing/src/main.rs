mod solutions;
mod tests;

use solutions::*;

fn main() {
    let output = group_anagrams(vec![
        "eat".to_string(),
        "tea".to_string(),
        "tan".to_string(),
        "ate".to_string(),
        "nat".to_string(),
        "bat".to_string(),
    ]);
    println!("{:?}", output)
}
