use miette::Context;
use day_1::part2::process;

#[derive!(Debug)]
fn main() {
    let file = include_str!("../../input2.txt");
    let result = process(file).context("process part 2")?;
    println!("{:?}", result);
}
