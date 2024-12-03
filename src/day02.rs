use aoc2024::{print_part_solution, read_lines};
use color_eyre::Result;

const INPUT: &str = "assets/input_day02";

#[derive(PartialEq)]
enum Direction {
    Increasing,
    Decreasing,
    Invalid,
}

pub fn main() -> Result<()> {
    let result = check_all_reports(INPUT)?;
    print_part_solution(1, "The amount of safe reports is:", 202);
    print_part_solution(2, "The amount of safe reports is:", result);

    Ok(())
}

fn check_all_reports(file: &str) -> Result<i32> {
    let lines = read_lines(file)?;

    let mut safe = 0;

    lines.map_while(Result::ok).for_each(|line| {
        let report: Vec<i32> = line
            .split(" ")
            .filter(|part| !part.trim().is_empty())
            .map(|part| part.parse::<i32>().expect("failed to parse report"))
            .collect();

        if check_report(&report, 0) {
            safe += 1;
        }
    });

    Ok(safe)
}

fn check_report(report: &[i32], elements_removed: i8) -> bool {
    if report.len() < 2 {
        return false;
    }

    let mut bad_transition: Option<(usize, usize)> = None;

    let direction = get_direction(report);
    if direction == Direction::Invalid {
        return false;
    }

    for (i, pair) in report.windows(2).enumerate() {
        if let [a, b] = pair {
            let mut valid_transition = true;

            if a == b {
                valid_transition = false;
            }
            if (a - b).abs() > 3 {
                valid_transition = false;
            }

            if direction == Direction::Increasing && a > b {
                valid_transition = false;
            }
            if direction == Direction::Decreasing && a < b {
                valid_transition = false;
            }

            if !valid_transition {
                if elements_removed > 0 {
                    return false;
                }
                bad_transition = Some((i, i + 1));
                break;
            }
        }
    }

    return if let Some(bad_transition) = bad_transition {
        check_without_bad_transition(report, bad_transition)
    } else {
        true
    };
}

fn check_without_bad_transition(report: &[i32], index: (usize, usize)) -> bool {
    let (a, b) = index;

    let index_one_removed = check_without_element(report, a);
    let index_two_removed = check_without_element(report, b);

    index_one_removed || index_two_removed
}

fn check_without_element(report: &[i32], index: usize) -> bool {
    let mut report_without_level = report.to_vec();

    report_without_level.remove(index);

    check_report(&report_without_level, 1)
}

fn get_direction(report: &[i32]) -> Direction {
    let mut asc = 0;
    let mut desc = 0;
    let mut eq = 0;
    for pair in report.windows(2) {
        if let [a, b] = pair {
            if a > b {
                desc += 1;
            } else if a < b {
                asc += 1;
            } else if a == b {
                eq += 1;
            }
        }
    }

    return if eq > 1 || asc > 1 && desc > 1 {
        Direction::Invalid
    } else if asc > desc {
        Direction::Increasing
    } else if desc > asc {
        Direction::Decreasing
    } else {
        // what if 3 elements - not in inpuut!
        Direction::Invalid
    };
}

#[cfg(test)]
mod test {
    const LIST_1: [i32; 5] = [7, 6, 4, 2, 1];
    const LIST_2: [i32; 5] = [1, 2, 7, 8, 9];
    const LIST_3: [i32; 5] = [9, 7, 6, 2, 1];
    const LIST_4: [i32; 5] = [1, 3, 2, 4, 5];
    const LIST_5: [i32; 5] = [8, 6, 4, 4, 1];
    const LIST_6: [i32; 5] = [1, 3, 6, 7, 9];

    const LISTS: [[i32; 5]; 6] = [LIST_1, LIST_2, LIST_3, LIST_4, LIST_5, LIST_6];
    use super::*;

    #[test]
    fn test_all() {
        let mut safe = 0;
        for list in LISTS {
            if check_report(&list, 0) {
                safe += 1
            }
        }
        assert_eq!(safe, 4);
    }

    #[test]
    fn test_1() {
        let list = [1, 2, 3, 4, 5];
        assert_eq!(check_report(&list, 0), true);
    }

    #[test]
    fn test_2() {
        let list = [1, 1, 3, 4, 5];
        assert_eq!(check_report(&list, 0), true);
    }

    #[test]
    fn test_3() {
        let list = [1, 1, 1, 4, 5];
        assert_eq!(check_report(&list, 0), false);
    }

    #[test]
    fn test_4() {
        let list = [1, 2, 2, 4, 5];
        assert_eq!(check_report(&list, 0), true);
    }

    #[test]
    fn test_5() {
        let list = [1, 2, 2, 4, 3];
        assert_eq!(check_report(&list, 0), false);
    }

    #[test]
    fn test_6() {
        let list = []; // No levels
        assert_eq!(check_report(&list, 0), false);
    }

    #[test]
    fn test_7() {
        let list = [1]; // Single level
        assert_eq!(check_report(&list, 0), false);
    }

    #[test]
    fn test_8() {
        let list = [1, 2, 2, 3]; // Removing either "2" could make it safe
        assert_eq!(check_report(&list, 0), true);
    }

    #[test]
    fn test_9() {
        let list = [1, 5, 6, 9]; // Invalid due to 1 -> 5
        assert_eq!(check_report(&list, 0), true);
    }

    #[test]
    fn test_10() {
        let list = [1, 2, 3, 2, 5]; // Removing "3" or "2" might make it safe
        assert_eq!(check_report(&list, 0), true);
    }

    #[test]
    fn test_11() {
        let list = [9, 8, 6, 6, 5]; // Removing one "6" might make it safe
        assert_eq!(check_report(&list, 0), true);
    }

    #[test]
    fn test_12() {
        let list = [1, 3, 2, 4, 5]; // Removing "2" might make it safe
        assert_eq!(check_report(&list, 0), true);
    }

    #[test]
    fn test_report() {
        assert_eq!(check_report(&LIST_1, 0), true);
        assert_eq!(check_report(&LIST_2, 0), false);
        assert_eq!(check_report(&LIST_3, 0), false);
        assert_eq!(check_report(&LIST_4, 0), true);
        assert_eq!(check_report(&LIST_5, 0), true);
        assert_eq!(check_report(&LIST_6, 0), true);
    }

    #[test]
    fn test_input() {
        assert_eq!(check_all_reports(INPUT).unwrap(), 271);
    }
}
