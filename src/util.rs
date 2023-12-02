use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Reads file line by line.
pub fn readlines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_readlines() -> Result<(), &'static str> {
        let mut lines = readlines("README.md").expect("error reading file");
        if let Some(Ok(line)) = lines.next() {
            assert_eq!(line, "# Advent of Code 2023");
        } else {
            unreachable!()
        }

        Ok(())
    }
}
