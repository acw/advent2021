const TEST_DATA: &str = include_str!("../../data/day10t.txt");
const REAL_DATA: &str = include_str!("../../data/day10a.txt");

#[derive(Debug)]
enum ParseResult {
    IncompleteLine(Vec<char>),
    IllegalCharacter(char),
    UnexpectedClose(char),
    Success,
}

fn flip(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => c,
    }
}

fn parse_line(s: &str) -> ParseResult {
    let mut open_stack = Vec::new();

    for c in s.chars() {
        match c {
            '(' | '[' | '{' | '<' => open_stack.push(flip(c)),
            ')' | ']' | '}' | '>' => match open_stack.pop() {
                None => return ParseResult::UnexpectedClose(c),
                Some(expected) if expected == c => {}
                Some(_) => return ParseResult::UnexpectedClose(c),
            },
            _ => return ParseResult::IllegalCharacter(c),
        }
    }

    if open_stack.is_empty() {
        ParseResult::Success
    } else {
        ParseResult::IncompleteLine(open_stack)
    }
}

fn process_file(file_data: &str) -> (u64, u64) {
    let mut part1_score = 0;
    let mut part2_scores = Vec::new();

    for line in file_data.lines() {
        match parse_line(line) {
            ParseResult::UnexpectedClose(')') => part1_score += 3,
            ParseResult::UnexpectedClose(']') => part1_score += 57,
            ParseResult::UnexpectedClose('}') => part1_score += 1197,
            ParseResult::UnexpectedClose('>') => part1_score += 25137,

            ParseResult::IncompleteLine(stack) => {
                let mut new_score = 0;

                for c in stack.iter().rev() {
                    match c {
                        ')' => new_score = new_score * 5 + 1,
                        ']' => new_score = new_score * 5 + 2,
                        '}' => new_score = new_score * 5 + 3,
                        '>' => new_score = new_score * 5 + 4,
                        _ => new_score *= 5,
                    }
                }

                part2_scores.push(new_score);
            }

            _ => {}
        }
    }

    part2_scores.sort_unstable();

    (part1_score, part2_scores[part2_scores.len() / 2])
}

fn main() {
    println!("Test result: {:?}", process_file(TEST_DATA));
    println!("Real result: {:?}", process_file(REAL_DATA));
}
