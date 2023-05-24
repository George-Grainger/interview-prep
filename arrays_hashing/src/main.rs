mod solutions;

fn main() {
    let to_test = vec![1, 2, 3, 4];
    let output = solutions::contains_duplicate(to_test);
    println!("{}", output)
}
