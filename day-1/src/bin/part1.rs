use miette::Context;
use day_1::part1::process;

#[derive!(Debug)]
fn main() {
    let file = include_str!("../../input1.txt");
    let result = process(file).context("process part 1")?;
    println!("{:?}", result);
}
