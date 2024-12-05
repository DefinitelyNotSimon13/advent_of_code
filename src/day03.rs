use aoc2024::{print_day_title, print_part_solution};

use color_eyre::Result;
use std::iter::Peekable;

#[allow(dead_code)]
const TEST_INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
const INPUT: &str = "assets/input_day03";

#[derive(PartialEq, Debug, Clone)]
enum Token {
    Keyword(Keyword),
    BlockOpen,
    BlockClose,
    Seperator,
    NumLiteral(i32),
    Unknown,
}

#[derive(PartialEq, Debug, Clone)]
enum Keyword {
    Multiply,
    Do,
    Dont,
}


pub fn main() -> Result<()> {
    print_day_title(3);
    print_part_solution(1, "The solution has been lost", "--");
    let sum = calculate_input(INPUT)?;
    print_part_solution(2, "Sum of operations:", sum);

    Ok(())
}

fn calculate_input(file: &str) -> Result<i32> {
    let mut sum = 0;
    let input: String = std::fs::read_to_string(file)?;

    let mut tokenizer = Tokenizer::new(&input);
    tokenizer.tokenize();

    let mut nice = Parser::new(tokenizer.read_tokens.clone());
    nice.parse();

    sum += calculate_operations(nice.operations).unwrap();

    Ok(sum)
}

fn calculate_operations(operations: Vec<Operation>) -> Option<i32> {
    let mut sum = 0;
    for op in operations {
        match op.op_type {
            Keyword::Multiply => sum += op.num_1? * op.num_2?,
            _ => todo!("Unimplemented keyword"),
        }
    }

    Some(sum)
}

#[derive(Debug)]
struct Operation {
    op_type: Keyword,
    num_1: Option<i32>,
    num_2: Option<i32>,
}

impl Operation {
    fn new(op_type: Keyword) -> Self {
        Self {
            op_type,
            num_1: None,
            num_2: None,
        }
    }

    fn set_num_1(&mut self, num: i32) {
        self.num_1 = Some(num)
    }

    fn set_num_2(&mut self, num: i32) {
        if self.num_1.is_none() {
            panic!("Tried to set num_2 with num_1 as None")
        }
        self.num_2 = Some(num)
    }

    fn no_num_set(&self) -> bool {
        self.num_1.is_none() && self.num_2.is_none()
    }

    fn only_num_1_set(&self) -> bool {
        self.num_1.is_some() && self.num_2.is_none()
    }

    fn is_valid_mul(&self) -> bool {
        self.num_1.is_some() && self.num_2.is_some()
    }
}

#[derive(Debug)]
struct Parser {
    tokens: Vec<Token>,
    last_token: Token,
    current_operation: Option<Operation>,
    operations: Vec<Operation>,
    do_mode: bool,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Self {
            tokens,
            last_token: Token::Unknown,
            current_operation: None,
            operations: vec![],
            do_mode: true,
        }
    }

    fn parse(&mut self) {
        // valid block:
        // Keyword(Multiply),BlockOpen,NumLiteral,Seperator,
        // NumLiteral,BlockClose
        for token in self.tokens.clone().into_iter() {
            if let Some(operation) = self.current_operation.as_ref() {
                if operation.num_1 == Some(8) && operation.num_2 == Some(5) {
                    panic!("I am evil");
                }
            }
            let cur_token = token.clone();
            if self.current_operation.is_none() {
                if let Token::Keyword(keyword) = token {
                    self.current_operation = Some(Operation::new(keyword))
                }
                self.last_token = cur_token;
                continue;
            }
            match token {
                Token::Keyword(keyword) => self.current_operation = Some(Operation::new(keyword)),
                Token::BlockOpen => self.parse_block_open(),
                Token::BlockClose => self.parse_block_close(),
                Token::Seperator => self.parse_seperator(),
                Token::NumLiteral(num) => self.parse_num_literal(num),
                Token::Unknown => self.current_operation = None,
            }
            self.last_token = cur_token;
            // println!("Reading complete:");
            // println!("Current Operation: {:#?}", self.current_operation);
        }
    }

    fn current_op_only_num_1(&self) -> bool {
        self.current_operation
            .as_ref()
            .map(|op| op.only_num_1_set())
            .unwrap_or(false)
    }

    fn current_op_no_num(&self) -> bool {
        self.current_operation
            .as_ref()
            .map(|op| op.no_num_set())
            .unwrap_or(false)
    }

    fn parse_block_open(&mut self) {
        if !matches!(self.last_token, Token::Keyword(_)) {
            self.current_operation = None;
        }
    }

    fn parse_block_close(&mut self) {
        if let Some(operation) = self.current_operation.take() {
            match operation.op_type {
                Keyword::Multiply if self.do_mode && operation.is_valid_mul() => {
                    self.operations.push(operation)
                }
                Keyword::Do => self.do_mode = true,
                Keyword::Dont => self.do_mode = false,
                _ => self.current_operation = None,
            }
        }
    }

    fn parse_seperator(&mut self) {
        if !matches!(self.last_token, Token::NumLiteral(_)) || !self.current_op_only_num_1() {
            self.current_operation = None;
        }
    }

    fn parse_num_literal(&mut self, num: i32) {
        match self.last_token {
            Token::BlockOpen if self.current_op_no_num() => {
                if let Some(operation) = self.current_operation.as_mut() {
                    operation.set_num_1(num);
                } else {
                    panic!("Tried to modify a non existing cur op")
                }
            }
            Token::Seperator if self.current_op_only_num_1() => {
                if let Some(operation) = self.current_operation.as_mut() {
                    operation.set_num_2(num);
                } else {
                    panic!("Tried to modify a non existing cur op")
                }
            }
            _ => {
                self.current_operation = None;
            }
        }
    }
}

struct Tokenizer<'a> {
    phrase: Peekable<std::str::Chars<'a>>,
    pub read_tokens: Vec<Token>,
}

impl Tokenizer<'_> {
    fn new(phrase: &str) -> Tokenizer<'_> {
        Tokenizer {
            phrase: phrase.chars().peekable(),
            read_tokens: Vec::default(),
        }
    }

    fn tokenize(&mut self) {
        while let Some(char) = self.phrase.next() {
            self.read_tokens.push(match char {
                '(' => Token::BlockOpen,
                ')' => Token::BlockClose,
                ',' => Token::Seperator,
                char if char.is_ascii_digit() => {
                    let mut literal: String = char.to_string();
                    while let Some(next) = self.phrase.next_if(|&x| x.is_ascii_digit()) {
                        if literal.len() == 3 {
                            panic!("Tried to create a NumLiteral with len > 3");
                        }
                        literal.push(next);
                    }
                    Token::NumLiteral(literal.parse().expect("failed to parse NumLiteral"))
                }
                _ => match self.match_keyword(char) {
                    Some(keyword) => match keyword {
                        Keyword::Multiply => {
                            self.phrase.next();
                            self.phrase.next();
                            Token::Keyword(keyword)
                        }
                        Keyword::Do => {
                            self.phrase.next();
                            Token::Keyword(keyword)
                        }
                        Keyword::Dont => {
                            self.phrase.next();
                            self.phrase.next();
                            self.phrase.next();
                            self.phrase.next();
                            Token::Keyword(keyword)
                        }
                    },
                    None => Token::Unknown,
                },
            });
        }
    }

    fn match_keyword(&self, first_char: char) -> Option<Keyword> {
        let cloned_iter = self.phrase.clone();
        let keyword: &str = &(first_char.to_string() + &cloned_iter.take(6).collect::<String>());
        match keyword {
            k if k.starts_with("mul") => Some(Keyword::Multiply),
            k if k.starts_with("don't") => Some(Keyword::Dont),
            k if k.starts_with("do") => Some(Keyword::Do),
            _ => None,
        }
    }
}
