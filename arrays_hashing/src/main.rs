mod solutions;
mod tests;

use solutions::*;

fn main() {
    let output = is_anagram("test".to_string(), "etst".to_string());
    println!("{}", output)
}
