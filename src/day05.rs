use aoc2024::{print_day_title, print_part_solution, read_lines};
use color_eyre::Result;
use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
const TEST_INPUT: &str = "assets/test_input_day05";
const INPUT: &str = "assets/input_day05";

type PageRules = HashMap<u32, Page>;
type Updates = Vec<Update>;

enum UpdateKind {
    Correct,
    Incorrect,
}

pub fn main() -> Result<()> {
    print_day_title(5);

    let (page_rules, updates) = read_input(INPUT)?;

    // Part 1
    let sum: u32 = updates
        .iter()
        .filter_map(|update| update.parse_with_rules(&page_rules, UpdateKind::Correct))
        .sum();
    print_part_solution(1, "Sum of correct middle elements", sum);

    // Part 2
    let sum: u32 = updates
        .iter()
        .filter_map(|update| update.parse_with_rules(&page_rules, UpdateKind::Incorrect))
        .sum();
    print_part_solution(2, "Sum of incorrect middle elements", sum);

    Ok(())
}

fn read_input(file: &str) -> Result<(PageRules, Updates)> {
    let lines = read_lines(file)?;

    let mut page_rules: HashMap<u32, Page> = HashMap::default();
    let mut updates: Vec<Update> = Vec::default();

    let mut line_break_occured = false;
    lines.map_while(Result::ok).for_each(|line| {
        if line.is_empty() {
            line_break_occured = true;
            return;
        }

        if line_break_occured {
            updates.push(Update::new(
                line.split(",")
                    .map(str::trim)
                    .map(|part| part.parse::<u32>().expect("failed to parse update"))
                    .collect(),
            ))
        } else {
            let page_rule: Vec<u32> = line
                .split("|")
                .map(str::trim)
                .map(|part| part.parse::<u32>().expect("failed to parse page_rule"))
                .collect();

            if let std::collections::hash_map::Entry::Occupied(mut entry) =
                page_rules.entry(page_rule[0])
            {
                let page = entry.get_mut();
                page.add_dependant_page(page_rule[1]);
            } else {
                page_rules.insert(
                    page_rule[0],
                    Page::with_dependant(page_rule[0], page_rule[1]),
                );
            }
        }
    });
    Ok((page_rules, updates))
}

#[derive(Debug)]
struct Update {
    pages: Vec<u32>,
}

impl Update {
    fn new(pages: Vec<u32>) -> Self {
        Self { pages }
    }

    fn parse_with_rules(&self, page_rules: &PageRules, update_kind: UpdateKind) -> Option<u32> {
        let mut printed_pages: VecDeque<u32> = VecDeque::new();
        let mut update_was_incorrect = false;

        for page in &self.pages {
            if let Some(rule) = page_rules.get(page) {
                if let Some((index, _)) = printed_pages
                    .iter()
                    .enumerate()
                    .find(|(_, item)| rule.printed_before.contains(item))
                {
                    update_was_incorrect = true;
                    printed_pages.insert(index, *page);
                } else {
                    printed_pages.push_back(*page);
                }
            } else {
                printed_pages.push_back(*page);
            }
        }

        let update_len = printed_pages.len();

        if update_len % 2 == 0 {
            panic!("Even number of pages what's the middle?");
        }

        let should_return_some = match update_kind {
            UpdateKind::Correct => !update_was_incorrect,
            UpdateKind::Incorrect => update_was_incorrect,
        };

        should_return_some.then(|| printed_pages[(update_len - 1) / 2])
    }
}

#[derive(Debug)]
struct Page {
    _number: u32,
    printed_before: HashSet<u32>,
}

impl Page {
    fn with_dependant(number: u32, dependant: u32) -> Self {
        Self {
            _number: number,
            printed_before: HashSet::from([dependant]),
        }
    }

    fn add_dependant_page(&mut self, number: u32) {
        self.printed_before.insert(number);
    }
}
