use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn count_occurrences<T: PartialEq>(list: &[T], eq: T) -> usize {
    list.iter().filter(|it| **it == eq).count()
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
