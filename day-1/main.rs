use std::fs;

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

const FIRST_LETTERS: [char; 6] = ['o', 't', 'f', 's', 'e', 'n'];

struct DigitFinder {
    position: usize,
    possibilities: Vec<usize>,
}

impl DigitFinder {
    fn new() -> DigitFinder {
        DigitFinder {
            position: 0,
            possibilities: (0..DIGITS.len()).collect(),
        }
    }

    fn look(&mut self, character: char) -> Option<u32> {
        let mut possibilities = Vec::new();

        for index in &self.possibilities {
            let digit = DIGITS[*index];
            if self.position >= digit.len() {
                continue;
            }

            if character == digit.chars().nth(self.position).unwrap() {
                possibilities.push(index.clone());
            }
        }

        self.position += 1;
        match possibilities.len() {
            0 => {
                self.reset();
                if FIRST_LETTERS.contains(&character) {
                    self.look(character);
                }
            }
            1 => {
                let candidate = possibilities[0];
                if DIGITS[candidate].len() == self.position {
                    self.reset();
                    if FIRST_LETTERS.contains(&character) {
                        self.look(character);
                    }
                    return Some((candidate + 1) as u32);
                }

                self.possibilities = possibilities;
            }
            _ => {
                self.possibilities = possibilities;
            }
        }

        None
    }

    fn reset(&mut self) {
        self.position = 0;
        self.possibilities = (0..DIGITS.len()).collect();
    }
}

fn main() {
    let contents = fs::read_to_string("input").expect("Error reading file");
    let mut total = 0;

    for line in contents.lines() {
        let mut first = None;
        let mut last = None;
        let mut digit_finder = DigitFinder::new();

        for chara in line.chars() {
            let digit = if chara.is_digit(10) {
                digit_finder.reset();
                chara.to_digit(10)
            } else {
                digit_finder.look(chara)
            };

            if let Some(digit) = digit {
                if first == None {
                    first = Some(digit);
                    last = Some(digit);
                } else {
                    last = Some(digit);
                }
            }
        }

        println!("{} -> {} {}", line, first.unwrap(), last.unwrap());

        total += first.unwrap() * 10 + last.unwrap();
    }

    println!("Total: {}", total);
}
