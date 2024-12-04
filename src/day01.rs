use aoc2024::{count_occurrences, print_part_solution, read_lines, print_day_title};
use color_eyre::Result;
use console::Style;

const INPUT: &str = "assets/input_day01";

pub fn main() -> Result<()> {
    print_day_title(1);

    let (list_1, list_2) = extract_lists(INPUT);

    let distance = calculate_list_distance(&list_1, &list_2).unwrap();
    print_part_solution(1, "The distance between the lists is:", distance);

    let similiarity_score = calculate_similarity_score(&list_1, &list_2);
    print_part_solution(
        2,
        "The similiarity score of the lists is:",
        similiarity_score,
    );
    Ok(())
}

fn calculate_list_distance(list_1: &[i32], list_2: &[i32]) -> Option<i32> {
    if list_1.len() != list_2.len() {
        return None;
    }
    let mut sorted_list_1 = list_1.to_vec();
    let mut sorted_list_2 = list_2.to_vec();
    sorted_list_1.sort_unstable();
    sorted_list_2.sort_unstable();

    Some(
        sorted_list_1
            .iter()
            .zip(sorted_list_2.iter())
            .map(|(a, b)| (b - a).abs())
            .sum(),
    )
}

fn calculate_similarity_score(list_1: &[i32], list_2: &[i32]) -> usize {
    let mut score: usize = 0;

    list_1
        .iter()
        .cloned()
        .for_each(|it| score += it as usize * count_occurrences(list_2, it));

    score
}

fn extract_lists(file: &str) -> (Vec<i32>, Vec<i32>) {
    let lines = read_lines(file).unwrap();

    let mut list_1: Vec<i32> = Vec::default();
    let mut list_2: Vec<i32> = Vec::default();

    lines.map_while(Result::ok).for_each(|line| {
        let ids: Vec<i32> = line
            .split(" ")
            .map(|id| id.trim().parse())
            .filter_map(Result::ok)
            .collect();
        assert_eq!(ids.len(), 2);
        list_1.push(ids[0]);
        list_2.push(ids[1]);
    });
    (list_1, list_2)
}

#[cfg(test)]
mod tests {
    use super::*;
    const LIST_1: [i32; 6] = [3, 4, 2, 1, 3, 3];
    const LIST_2: [i32; 6] = [4, 3, 5, 3, 9, 3];

    #[test]
    fn list_distance() {
        let distance = calculate_list_distance(&LIST_1, &LIST_2);
        assert_eq!(distance, Some(11));
    }

    #[test]
    fn list_similiarity() {
        let similiarity = calculate_similarity_score(&LIST_1, &LIST_2);
        assert_eq!(similiarity, 31)
    }

    #[test]
    fn element_occurrences() {
        let occurrence_1 = count_occurrences(&LIST_2, 1);
        let occurrence_2 = count_occurrences(&LIST_2, 2);
        let occurrence_3 = count_occurrences(&LIST_2, 3);
        let occurrence_4 = count_occurrences(&LIST_2, 4);

        assert_eq!(occurrence_1, 0);
        assert_eq!(occurrence_2, 0);
        assert_eq!(occurrence_3, 3);
        assert_eq!(occurrence_4, 1);
    }

    #[test]
    fn task_1() {
        let (list_1, list_2) = extract_lists(INPUT);

        let distance = calculate_list_distance(&list_1, &list_2).unwrap();
        assert_eq!(distance, 765748)
    }

    #[test]
    fn file_read_correctly() {
        let file_content = std::fs::read_to_string(INPUT).expect("failed to read file");

        let mut by_line = String::default();

        for line in read_lines(INPUT).unwrap().flatten() {
            by_line.push_str(&line);
            by_line.push_str("\n");
        }

        assert_eq!(file_content, by_line);
    }
}
