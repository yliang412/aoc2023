use aoc2023::day01::trebuchet_pt2;

fn main() -> Result<(), &'static str> {
    let sum = trebuchet_pt2("data/day01.input")?;
    println!("Sum is {}", sum);
    Ok(())
}
