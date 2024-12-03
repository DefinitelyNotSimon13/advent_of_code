use std::fmt::Display;
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

pub fn print_part_solution<T: Display>(part: i8, support_text: &str, solution: T) {
    println!(
        "\t{} {}\t {}",
        AocStyles::part_number().apply_to(format!("Part {}:", part)),
        support_text,
        AocStyles::solution().apply_to(solution),
    );
}

mod AocStyles {
    use console::Style;
    pub fn part_number() -> Style {
        Style::new().dim()
    }
    pub fn solution() -> Style {
        Style::new().cyan().bold()
    }
}
