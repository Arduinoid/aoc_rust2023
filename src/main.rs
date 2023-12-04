use std::fs;

fn main() {
    let input = fs::read_to_string("./input_data/d1p1.txt").unwrap();
    println!("Grabing first and last numbers from each line to then concat into a number and then some each of them");
    let mut total: u32 = 0;

    for i in input.as_str().split("\n") {
        let mut nums = i.chars().filter(|c| c.is_numeric());
        let Some(first) = nums.next() else { return };
        let mut result = first.to_string();

        if let Some(n) = nums.last() {
            result.push(n)
        } else {
            result.push(first)
        }
        total += result.parse::<u32>().unwrap();
        println!("{result}")
    }
    assert_eq!(total, 54331); // this is the correct answer and is here for reference and refactoring
    println!("total = {total}")
}
