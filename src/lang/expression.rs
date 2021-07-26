use super::interpreter::*;
use regex::Regex;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Atom {
    Greater,
    Less,
    Equal,
    NotEqual,
    Assign,
    Not,
    And,
    Or,
    Xor,
    LeftShift,
    RightShift,
    Sum,
    Difference,
    Product,
    Quotient,
    Remainder,
    Power,
    LeftParen,
    RightParen,
    Output,
    Data(i128),
    Memory,
}

#[derive(Debug, PartialEq)]
pub enum AtomType {
    Number,
    Binary,
    Unary,
}

impl Atom {
    pub fn precedence(&self) -> u8 {
        match self {
            Atom::Output | Atom::Assign => 1,
            Atom::Less | Atom::Greater | Atom::Equal | Atom::NotEqual => 2,
            Atom::Or => 3,
            Atom::Xor => 4,
            Atom::And => 5,
            Atom::LeftShift | Atom::RightShift => 6,
            Atom::Sum | Atom::Difference => 7,
            Atom::Product | Atom::Quotient | Atom::Remainder => 8,
            Atom::Memory | Atom::Not => 9,
            Atom::Power => 10,
            _ => 0,
        }
    }

    pub fn form(&self) -> AtomType {
        match self {
            Atom::Output | Atom::Memory | Atom::Not => AtomType::Unary,
            Atom::Data(_) => AtomType::Number,
            _ => AtomType::Binary,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Molecule {
    children: Vec<Atom>,
    sorted_children: Option<Vec<Atom>>,
    valid: bool,
}

impl Molecule {
    pub fn new(children: Vec<Atom>) -> Molecule {
        Molecule {
            children,
            sorted_children: None,
            valid: false,
        }
    }

    pub fn sort(children: &[Atom], sorted: &mut Option<Vec<Atom>>) -> Result<Vec<Atom>, String> {
        if sorted.is_none() {
            let mut output: Vec<Atom> = Vec::new();
            let mut stack: Vec<Atom> = Vec::new();

            for child in children {
                if let Atom::Data(_) = *child {
                    output.push(*child);
                } else if let Atom::LeftParen | Atom::Not | Atom::Memory = *child {
                    stack.push(*child);
                } else if let Atom::RightParen = *child {
                    while !stack.is_empty() && stack.last().cloned().unwrap() != Atom::LeftParen {
                        output.push(stack.pop().unwrap());
                    }

                    if stack.is_empty() {
                        return Err("Malformed expression".to_string());
                    }

                    stack.pop();
                } else {
                    let precedence = child.precedence();

                    if let Atom::Power | Atom::Assign = *child {
                        while !stack.is_empty()
                            && stack.last().cloned().unwrap().precedence() > precedence
                        {
                            output.push(stack.pop().unwrap());
                        }
                    } else {
                        while !stack.is_empty()
                            && stack.last().cloned().unwrap().precedence() >= precedence
                        {
                            output.push(stack.pop().unwrap());
                        }
                    }

                    stack.push(*child);
                }
            }

            while !stack.is_empty() {
                if let Atom::LeftParen = stack.last().cloned().unwrap() {
                    return Err("Malformed expression".to_string());
                }

                output.push(stack.pop().unwrap());
            }

            *sorted = Some(output);
        }

        Ok(sorted.as_ref().unwrap().to_vec())
    }
}

impl Parser<Atom> for Molecule {
    fn parse(expr: &str) -> Result<Vec<Atom>, String> {
        lazy_static! {
            static ref NUMBER: Regex = Regex::new(r"^[▄▀]+").unwrap();
            static ref WHITESPACE: Regex = Regex::new(r"^[\s]+").unwrap();
            static ref OTHER: Regex = Regex::new(r"^.").unwrap();
        }

        let mut expr_copy = expr.to_string();
        let mut children: Vec<Atom> = Vec::new();

        while !expr_copy.is_empty() {
            if WHITESPACE.is_match(&expr_copy) {
                expr_copy = WHITESPACE.replace_all(&expr_copy, "").to_string();
            } else if NUMBER.is_match(&expr_copy) {
                let number = NUMBER.find(&expr_copy).unwrap().as_str();

                if number.chars().count() == 1 {
                    children.push(Atom::Data(0));
                } else {
                    let digits: String = number
                        .chars()
                        .map(|c| match c {
                            '▀' => '1',
                            _ => '0',
                        })
                        .collect();
                    let val = i128::from_str_radix(&digits[1..], 2).unwrap();
                    if number.starts_with('▄') {
                        children.push(Atom::Data(-val));
                    } else {
                        children.push(Atom::Data(val));
                    }
                }

                expr_copy = NUMBER.replace_all(&expr_copy, "").to_string();
            } else {
                children.push(match expr_copy.chars().next().unwrap() {
                    '▕' => Atom::LeftParen,
                    '▏' => Atom::RightParen,
                    '▔' => Atom::Not,
                    '▖' => Atom::Power,
                    '▗' => Atom::Remainder,
                    '▘' => Atom::Product,
                    '▝' => Atom::Quotient,
                    '▚' => Atom::LeftShift,
                    '▞' => Atom::RightShift,
                    '▐' => Atom::Sum,
                    '▌' => Atom::Difference,
                    '▨' => Atom::Less,
                    '▧' => Atom::Greater,
                    '▤' => Atom::Equal,
                    '▥' => Atom::NotEqual,
                    '░' => Atom::And,
                    '▒' => Atom::Xor,
                    '▓' => Atom::Or,
                    '◇' => Atom::Memory,
                    '◈' => Atom::Assign,
                    '▭' => Atom::Output,
                    _ => return Err("Malformed expression".to_string()),
                });

                expr_copy = OTHER.replace_all(&expr_copy, "").to_string();
            }
        }

        Ok(children)
    }
}

impl Validator<Atom> for Molecule {
    fn validate(children: &[Atom], valid: &mut bool) -> Result<(), String> {
        if !*valid {
            let mut token_types: Vec<AtomType> = vec![];
            for child in children {
                if let Atom::LeftParen | Atom::RightParen = *child {
                } else {
                    token_types.push(Atom::form(child));
                }
            }

            if token_types.len() == 1 && token_types[0] != AtomType::Number
                || token_types.len() == 2
                    && (token_types[0] != AtomType::Unary || token_types[1] != AtomType::Number)
            {
                return Err("Malformed expression".to_string());
            }
            *valid = true;

            if token_types.is_empty() {
                return Ok(());
            }

            for i in 0..token_types.len() {
                if i == 0 {
                    *valid &= token_types[i] == AtomType::Number
                        && token_types[i + 1] == AtomType::Binary
                        || token_types[i] == AtomType::Unary
                            && token_types[i + 1] != AtomType::Binary;
                } else if i == token_types.len() - 1 {
                    *valid &= (token_types[i - 1] == AtomType::Binary
                        || token_types[i - 1] == AtomType::Unary)
                        && token_types[i] == AtomType::Number;
                } else {
                    *valid &= match token_types[i] {
                        AtomType::Number => {
                            token_types[i - 1] != AtomType::Number
                                && token_types[i + 1] != AtomType::Number
                        }
                        AtomType::Unary => {
                            token_types[i - 1] != AtomType::Number
                                && token_types[i + 1] != AtomType::Binary
                        }
                        AtomType::Binary => {
                            token_types[i - 1] == AtomType::Number
                                && token_types[i + 1] != AtomType::Binary
                        }
                    };
                }
            }

            if !*valid {
                return Err("Malformed expression".to_string());
            }

            Ok(())
        } else {
            Ok(())
        }
    }
}

impl Runnable for Molecule {
    fn run(
        &mut self,
        memory: &mut HashMap<i128, i128>,
        stdout: &mut String,
    ) -> Result<(i128, String), String> {
        let validity = Molecule::validate(&self.children, &mut self.valid);

        if let Err(msg) = validity {
            return Err(msg);
        }

        let children = Molecule::sort(&self.children, &mut self.sorted_children);

        if let Err(msg) = children {
            return Err(msg);
        }

        let mut stack: Vec<i128> = vec![];
        for child in children.unwrap() {
            if let Atom::Data(num) = child {
                stack.push(num);
            } else if let Atom::Memory | Atom::Not | Atom::Output = child {
                let a = stack.pop().unwrap();

                if let Atom::Memory = child {
                    stack.push(*memory.get(&a).unwrap_or(&0));
                } else if let Atom::Not = child {
                    stack.push(!a);
                } else if let Atom::Output = child {
                    stack.push(a);

                    if let Some(chr) = std::char::from_u32(a as u32) {
                        stdout.push(chr);
                    } else {
                        stdout.push('\u{ffff}');
                    }
                }
            } else {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();

                if let Atom::Assign = child {
                    memory.insert(a, b);
                }

                stack.push(match child {
                    Atom::Sum => a + b,
                    Atom::Difference => a - b,
                    Atom::Product => a * b,
                    Atom::Quotient => a / b,
                    Atom::Remainder => a % b,
                    Atom::LeftShift => a << b,
                    Atom::RightShift => a >> b,
                    Atom::And => a & b,
                    Atom::Or => a | b,
                    Atom::Xor => a ^ b,
                    Atom::Less => (a < b) as i128,
                    Atom::Greater => (a > b) as i128,
                    Atom::Equal => (a == b) as i128,
                    Atom::NotEqual => (a != b) as i128,
                    Atom::Power => (a as f64).powi(b as i32).round() as i128,
                    _ => b,
                });
            }
        }

        Ok((stack.pop().unwrap_or(0), stdout.to_string()))
    }
}

#[allow(unused_must_use)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_simple() {
        assert_eq!(
            Molecule::new(vec![Atom::Data(2), Atom::Sum, Atom::Data(2)])
                .run(&mut HashMap::new(), &mut String::new())
                .unwrap(),
            (4, String::new())
        );
    }

    #[test]
    fn it_detects_bad_outputs() {
        assert_eq!(
            Molecule::new(vec![Atom::Output, Atom::Data(55296),])
                .run(&mut HashMap::new(), &mut String::new()),
            Ok((55296, "\u{ffff}".to_string()))
        );
    }

    #[test]
    fn it_detects_bad_expressions() {
        assert_eq!(
            Molecule::new(vec![Atom::Data(0), Atom::Data(0)])
                .run(&mut HashMap::new(), &mut String::new()),
            Err("Malformed expression".to_string())
        );

        assert_eq!(
            Molecule::new(vec![Atom::Product, Atom::Data(0)])
                .run(&mut HashMap::new(), &mut String::new()),
            Err("Malformed expression".to_string())
        );

        assert_eq!(
            Molecule::new(vec![Atom::Difference, Atom::Not])
                .run(&mut HashMap::new(), &mut String::new()),
            Err("Malformed expression".to_string())
        );

        assert_eq!(
            Molecule::new(vec![Atom::Output, Atom::Memory])
                .run(&mut HashMap::new(), &mut String::new()),
            Err("Malformed expression".to_string())
        );

        assert_eq!(
            Molecule::new(vec![Atom::Not, Atom::Remainder])
                .run(&mut HashMap::new(), &mut String::new()),
            Err("Malformed expression".to_string())
        );

        assert_eq!(
            Molecule::new(vec![Atom::Data(0), Atom::Xor])
                .run(&mut HashMap::new(), &mut String::new()),
            Err("Malformed expression".to_string())
        );

        assert_eq!(
            Molecule::new(vec![Atom::And]).run(&mut HashMap::new(), &mut String::new()),
            Err("Malformed expression".to_string())
        );

        assert_eq!(
            Molecule::new(vec![Atom::Data(0), Atom::And, Atom::Quotient])
                .run(&mut HashMap::new(), &mut String::new()),
            Err("Malformed expression".to_string())
        );

        assert_eq!(
            Molecule::new(vec![Atom::And, Atom::LeftShift, Atom::Data(0)])
                .run(&mut HashMap::new(), &mut String::new()),
            Err("Malformed expression".to_string())
        );

        assert_eq!(
            Molecule::new(vec![Atom::And, Atom::Not, Atom::Data(0)])
                .run(&mut HashMap::new(), &mut String::new()),
            Err("Malformed expression".to_string())
        );
        assert_eq!(
            Molecule::new(vec![Atom::And, Atom::Data(0), Atom::Greater])
                .run(&mut HashMap::new(), &mut String::new()),
            Err("Malformed expression".to_string())
        );

        assert_eq!(
            Molecule::new(vec![Atom::RightParen, Atom::LeftParen])
                .run(&mut HashMap::new(), &mut String::new()),
            Err("Malformed expression".to_string())
        );
    }

    #[test]
    fn it_detects_bad_parentheses() {
        assert_eq!(
            Molecule::new(vec![Atom::LeftParen]).run(&mut HashMap::new(), &mut String::new()),
            Err("Malformed expression".to_string())
        );

        assert_eq!(
            Molecule::new(vec![Atom::RightParen]).run(&mut HashMap::new(), &mut String::new()),
            Err("Malformed expression".to_string())
        );
    }

    #[test]
    fn it_detects_bad_chars() {
        assert_eq!(
            Molecule::parse("a"),
            Err("Malformed expression".to_string())
        );
    }

    #[test]
    fn it_works_many_times() {
        let mut mol = Molecule::new(vec![Atom::Data(2), Atom::Product, Atom::Data(2)]);
        mol.run(&mut HashMap::new(), &mut String::new());
        assert_eq!(
            mol.run(&mut HashMap::new(), &mut String::new()).unwrap(),
            (4, String::new())
        );

        assert_eq!(
            mol.run(&mut HashMap::new(), &mut String::new()).unwrap(),
            mol.run(&mut HashMap::new(), &mut String::new()).unwrap()
        );
    }

    #[test]
    fn it_outputs() {
        assert_eq!(
            Molecule::new(vec![Atom::Output, Atom::Data(48)])
                .run(&mut HashMap::new(), &mut String::new())
                .unwrap(),
            (48, "0".to_string())
        );
    }

    #[test]
    fn it_works_with_memory() {
        let mut hm = HashMap::<i128, i128>::new();
        hm.insert(0, 48);
        assert_eq!(
            Molecule::new(vec![Atom::Output, Atom::Memory, Atom::Data(0)])
                .run(&mut hm, &mut String::new())
                .unwrap(),
            (48, "0".to_string())
        );
        Molecule::new(vec![Atom::Data(0), Atom::Assign, Atom::Data(13)])
            .run(&mut hm, &mut String::new());
        assert_eq!(hm, [(0, 13)].iter().cloned().collect());
        assert_eq!(
            Molecule::new(vec![Atom::Memory, Atom::Data(13)])
                .run(&mut HashMap::new(), &mut String::new())
                .unwrap(),
            (0, String::new())
        );
        hm.insert(0, 48);
        Molecule::new(vec![
            Atom::Data(2),
            Atom::Sum,
            Atom::Data(1),
            Atom::Assign,
            Atom::Data(1),
            Atom::Assign,
            Atom::Data(0),
            Atom::Assign,
            Atom::Data(1),
        ])
        .run(&mut hm, &mut String::new());
        assert_eq!(hm, [(0, 1), (1, 1), (3, 1)].iter().cloned().collect());
    }

    #[test]
    fn it_works_with_memory_with_parsing() {
        let mut hm = HashMap::<i128, i128>::new();
        hm.insert(0, 48);
        assert_eq!(
            Molecule::new(Molecule::parse("▭◇▀").unwrap())
                .run(&mut hm, &mut String::new())
                .unwrap(),
            (48, "0".to_string())
        );
        Molecule::new(Molecule::parse("▀◈▀▀▀▄▀").unwrap()).run(&mut hm, &mut String::new());
        assert_eq!(hm, [(0, 13)].iter().cloned().collect());
        assert_eq!(
            Molecule::new(Molecule::parse("◇▀▀▀▄▀").unwrap())
                .run(&mut HashMap::new(), &mut String::new())
                .unwrap(),
            (0, String::new())
        );
        hm.insert(0, 48);
        Molecule::new(Molecule::parse("▀▀▄▐▀▀◈▀▀◈▀◈▀▀").unwrap()).run(&mut hm, &mut String::new());
        assert_eq!(hm, [(0, 1), (1, 1), (3, 1)].iter().cloned().collect());
    }

    #[test]
    fn it_works_complex() {
        assert_eq!(
            Molecule::new(vec![
                Atom::Not,
                Atom::Data(0),
                Atom::Sum,
                Atom::Data(1),
                Atom::Difference,
                Atom::Data(2),
                Atom::Product,
                Atom::Data(3),
                Atom::Quotient,
                Atom::Data(4),
                Atom::Remainder,
                Atom::Data(5),
                Atom::LeftShift,
                Atom::Data(6),
                Atom::RightShift,
                Atom::Data(7),
                Atom::Xor,
                Atom::Data(8),
                Atom::Or,
                Atom::Data(9),
                Atom::And,
                Atom::Data(10),
                Atom::Power,
                Atom::Data(11),
            ])
            .run(&mut HashMap::new(), &mut String::new())
            .unwrap(),
            (-9, String::new())
        );

        assert_eq!(
            Molecule::new(vec![
                Atom::Data(0),
                Atom::Less,
                Atom::Data(1),
                Atom::Greater,
                Atom::Data(2),
                Atom::Equal,
                Atom::Data(0),
                Atom::NotEqual,
                Atom::Data(-1),
            ])
            .run(&mut HashMap::new(), &mut String::new())
            .unwrap(),
            (1, String::new())
        );

        assert_eq!(
            Molecule::new(vec![
                Atom::LeftParen,
                Atom::LeftParen,
                Atom::Data(0),
                Atom::Sum,
                Atom::Data(2),
                Atom::RightParen,
                Atom::Power,
                Atom::Data(2),
                Atom::Power,
                Atom::Data(2),
                Atom::Difference,
                Atom::Data(8),
                Atom::RightParen,
            ])
            .run(&mut HashMap::new(), &mut String::new())
            .unwrap(),
            (8, String::new())
        );
    }

    #[test]
    fn it_works_complex_with_parsing() {
        assert_eq!(
            Molecule::new(
                Molecule::parse("▔▄▐▀▀▌▀▀▄▘▀▀▀▝▀▀▄▄▗▀▀▄▀▚▀▀▀▄▞▀▀▀▀▒▀▀▄▄▄▓▀▀▄▄▀░▀▀▄▀▄▖▀▀▄▀▀ ")
                    .unwrap()
            )
            .run(&mut HashMap::new(), &mut String::new())
            .unwrap(),
            (-9, String::new())
        );

        assert_eq!(
            Molecule::new(Molecule::parse("▀▄▨▀▀▧▀▀▄▤▀▄▥▄▀").unwrap())
                .run(&mut HashMap::new(), &mut String::new())
                .unwrap(),
            (1, String::new())
        );

        assert_eq!(
            Molecule::new(Molecule::parse("▕▕▀▄▐▀▀▄▏▖▀▀▄▖▀▀▄▌▀▀▄▄▄▏").unwrap())
                .run(&mut HashMap::new(), &mut String::new())
                .unwrap(),
            (8, String::new())
        );
    }
}
