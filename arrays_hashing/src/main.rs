mod solutions;
mod tests;

use solutions::contains_duplicate;

fn main() {
    let to_test = vec![1, 2, 3, 4];
    let output = contains_duplicate(to_test);
    println!("{}", output)
}
