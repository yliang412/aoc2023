use aoc2023::day02::cube_conundrum;

fn main() -> Result<(), &'static str> {
    let result = cube_conundrum("data/day02.example", 12, 13, 14)?;
    println!("Sum is {:?}", result);
    Ok(())
}
