use super::datatype::BoxInt;
use super::interpreter::{Parser, Runnable, Validator};
use super::math;
use regex::Regex;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Atom<T: BoxInt> {
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
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    InverseModulo,
    LeftParen,
    RightParen,
    Output,
    Data(T),
    Memory,
}

#[derive(Debug, PartialEq)]
pub enum AtomType {
    Number,
    Binary,
    Unary,
}

impl<T: BoxInt> Atom<T> {
    pub fn precedence(&self) -> u8 {
        match self {
            Atom::Output | Atom::Assign => 1,
            Atom::Less | Atom::Greater | Atom::Equal | Atom::NotEqual => 2,
            Atom::Or => 3,
            Atom::Xor => 4,
            Atom::And => 5,
            Atom::LeftShift | Atom::RightShift => 6,
            Atom::Add | Atom::Subtract => 7,
            Atom::Multiply | Atom::Divide | Atom::Modulo | Atom::InverseModulo => 8,
            Atom::Memory | Atom::Not => 9,
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
pub struct Molecule<T: BoxInt> {
    children: Vec<Atom<T>>,
    sorted_children: Option<Vec<Atom<T>>>,
    valid: bool,
}

impl<T: BoxInt> Molecule<T> {
    pub fn new(children: Vec<Atom<T>>) -> Molecule<T> {
        Molecule {
            children,
            sorted_children: None,
            valid: false,
        }
    }

    pub fn sort(
        children: &[Atom<T>],
        sorted: &mut Option<Vec<Atom<T>>>,
    ) -> Result<Vec<Atom<T>>, String> {
        if sorted.is_none() {
            let mut output: Vec<Atom<T>> = Vec::new();
            let mut stack: Vec<Atom<T>> = Vec::new();

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
                        return Err("Missing left parenthesis".to_string());
                    }

                    stack.pop();
                } else {
                    let precedence = child.precedence();

                    if let Atom::Assign = *child {
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
                    return Err("Missing right parenthesis".to_string());
                }

                output.push(stack.pop().unwrap());
            }

            *sorted = Some(output);
        }

        Ok(sorted.as_ref().unwrap().to_vec())
    }
}

impl<T: BoxInt> Parser<Atom<T>> for Molecule<T> {
    fn parse(expr: &str) -> Result<Vec<Atom<T>>, String> {
        lazy_static! {
            static ref NUMBER: Regex = Regex::new(r"^[▄▀]+").unwrap();
            static ref WHITESPACE: Regex = Regex::new(r"^[\s]+").unwrap();
            static ref OTHER: Regex = Regex::new(r"^.").unwrap();
        }

        let mut expr_copy = expr.to_string();
        let mut children: Vec<Atom<T>> = Vec::new();

        while !expr_copy.is_empty() {
            if WHITESPACE.is_match(&expr_copy) {
                expr_copy = WHITESPACE.replace(&expr_copy, "").to_string();
            } else if NUMBER.is_match(&expr_copy) {
                let number = NUMBER.find(&expr_copy).unwrap().as_str();

                if number.chars().count() == 1 {
                    children.push(Atom::Data(T::zero()));
                } else {
                    let digits: String = number
                        .chars()
                        .map(|c| match c {
                            '▀' => '1',
                            '▄' => '0',
                            _ => unreachable!(),
                        })
                        .collect();
                    let val = match T::from_str_radix(&digits[1..], 2) {
                        Ok(x) => x,
                        _ => unreachable!(),
                    };
                    if number.starts_with('▄') {
                        children.push(Atom::Data(T::zero() - val));
                    } else {
                        children.push(Atom::Data(val));
                    }
                }

                expr_copy = NUMBER.replace(&expr_copy, "").to_string();
            } else {
                children.push(match expr_copy.chars().next().unwrap() {
                    '▕' => Atom::LeftParen,
                    '▏' => Atom::RightParen,
                    '▔' => Atom::Not,
                    '▖' => Atom::Modulo,
                    '▗' => Atom::InverseModulo,
                    '▘' => Atom::Multiply,
                    '▝' => Atom::Divide,
                    '▚' => Atom::LeftShift,
                    '▞' => Atom::RightShift,
                    '▐' => Atom::Add,
                    '▌' => Atom::Subtract,
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
                    _ => return Err("Invalid character".to_string()),
                });

                expr_copy = OTHER.replace(&expr_copy, "").to_string();
            }
        }

        Ok(children)
    }
}

impl<T: BoxInt> Validator<Atom<T>> for Molecule<T> {
    fn validate(children: &[Atom<T>], valid: &mut bool) -> Result<(), String> {
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

impl<T: BoxInt> Runnable<T> for Molecule<T> {
    fn run(
        &mut self,
        memory: &mut std::collections::HashMap<T, T>,
        stdout: &mut String,
    ) -> Result<(T, String), String> {
        Molecule::validate(&self.children, &mut self.valid)?;

        let children = Molecule::sort(&self.children, &mut self.sorted_children)?;

        let mut stack: Vec<T> = vec![];
        for child in children {
            if let Atom::Data(num) = child {
                stack.push(num);
            } else if let Atom::Memory | Atom::Not | Atom::Output = child {
                let a = stack.pop().unwrap();

                if let Atom::Memory = child {
                    stack.push(*memory.get(&a).unwrap_or(&T::zero()));
                } else if let Atom::Not = child {
                    stack.push(!a);
                } else if let Atom::Output = child {
                    stack.push(a);

                    if let Some(val) = a.to_u32() {
                        if let Some(chr) = std::char::from_u32(val) {
                            stdout.push(chr);
                        } else {
                            stdout.push('\u{ffff}');
                        }
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
                    Atom::Add => a + b,
                    Atom::Subtract => a - b,
                    Atom::Multiply => a * b,
                    Atom::Divide => math::divide(a, b)?,
                    Atom::Modulo => math::modulo(a, b)?,
                    Atom::InverseModulo => math::inv_modulo(a, b)?,
                    Atom::LeftShift => {
                        a << b
                            .to_usize()
                            .ok_or("Bitwise shifts cannot use signed integers")?
                    }
                    Atom::RightShift => {
                        a >> b
                            .to_usize()
                            .ok_or("Bitwise shifts cannot use signed integers")?
                    }
                    Atom::And => a & b,
                    Atom::Or => a | b,
                    Atom::Xor => a ^ b,
                    Atom::Less => {
                        if a < b {
                            T::one()
                        } else {
                            T::zero()
                        }
                    }
                    Atom::Greater => {
                        if a > b {
                            T::one()
                        } else {
                            T::zero()
                        }
                    }
                    Atom::Equal => {
                        if a == b {
                            T::one()
                        } else {
                            T::zero()
                        }
                    }
                    Atom::NotEqual => {
                        if a != b {
                            T::one()
                        } else {
                            T::zero()
                        }
                    }
                    Atom::Assign => b,
                    _ => unreachable!(),
                });
            }
        }

        Ok((stack.pop().unwrap_or_else(T::zero), stdout.to_string()))
    }
}

#[allow(unused_must_use)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_simple() {
        assert_eq!(
            Molecule::<i8>::new(vec![Atom::Data(2), Atom::Add, Atom::Data(2)])
                .run(&mut std::collections::HashMap::new(), &mut String::new())
                .unwrap(),
            (4, String::new())
        );
    }

    #[test]
    fn it_detects_bad_outputs() {
        assert_eq!(
            Molecule::<i32>::new(vec![Atom::Output, Atom::Data(55296),])
                .run(&mut std::collections::HashMap::new(), &mut String::new()),
            Ok((55296, "\u{ffff}".to_string()))
        );
    }

    #[test]
    fn it_detects_bad_expressions() {
        assert_eq!(
            Molecule::<i8>::new(vec![Atom::Data(0), Atom::Data(0)])
                .run(&mut std::collections::HashMap::new(), &mut String::new()),
            Err("Malformed expression".to_string())
        );

        assert_eq!(
            Molecule::<i8>::new(vec![Atom::Multiply, Atom::Data(0)])
                .run(&mut std::collections::HashMap::new(), &mut String::new()),
            Err("Malformed expression".to_string())
        );

        assert_eq!(
            Molecule::<i8>::new(vec![Atom::Subtract, Atom::Not])
                .run(&mut std::collections::HashMap::new(), &mut String::new()),
            Err("Malformed expression".to_string())
        );

        assert_eq!(
            Molecule::<i8>::new(vec![Atom::Output, Atom::Memory])
                .run(&mut std::collections::HashMap::new(), &mut String::new()),
            Err("Malformed expression".to_string())
        );

        assert_eq!(
            Molecule::<i8>::new(vec![Atom::Not, Atom::Modulo])
                .run(&mut std::collections::HashMap::new(), &mut String::new()),
            Err("Malformed expression".to_string())
        );

        assert_eq!(
            Molecule::<i8>::new(vec![Atom::Data(0), Atom::Xor])
                .run(&mut std::collections::HashMap::new(), &mut String::new()),
            Err("Malformed expression".to_string())
        );

        assert_eq!(
            Molecule::<i8>::new(vec![Atom::And])
                .run(&mut std::collections::HashMap::new(), &mut String::new()),
            Err("Malformed expression".to_string())
        );

        assert_eq!(
            Molecule::<i8>::new(vec![Atom::Data(0), Atom::And, Atom::Divide])
                .run(&mut std::collections::HashMap::new(), &mut String::new()),
            Err("Malformed expression".to_string())
        );

        assert_eq!(
            Molecule::<i8>::new(vec![Atom::And, Atom::LeftShift, Atom::Data(0)])
                .run(&mut std::collections::HashMap::new(), &mut String::new()),
            Err("Malformed expression".to_string())
        );

        assert_eq!(
            Molecule::<i8>::new(vec![Atom::And, Atom::Not, Atom::Data(0)])
                .run(&mut std::collections::HashMap::new(), &mut String::new()),
            Err("Malformed expression".to_string())
        );

        assert_eq!(
            Molecule::<i8>::new(vec![Atom::And, Atom::Data(0), Atom::Greater])
                .run(&mut std::collections::HashMap::new(), &mut String::new()),
            Err("Malformed expression".to_string())
        );
    }

    #[test]
    fn it_detects_bad_parentheses() {
        assert_eq!(
            Molecule::<i8>::new(vec![Atom::LeftParen])
                .run(&mut std::collections::HashMap::new(), &mut String::new()),
            Err("Missing right parenthesis".to_string())
        );

        assert_eq!(
            Molecule::<i8>::new(vec![Atom::RightParen])
                .run(&mut std::collections::HashMap::new(), &mut String::new()),
            Err("Missing left parenthesis".to_string())
        );

        assert_eq!(
            Molecule::<i8>::new(vec![Atom::RightParen, Atom::LeftParen])
                .run(&mut std::collections::HashMap::new(), &mut String::new()),
            Err("Missing left parenthesis".to_string())
        );
    }

    #[test]
    fn it_detects_bad_chars() {
        assert_eq!(
            Molecule::<i8>::parse("a"),
            Err("Invalid character".to_string())
        );
    }

    #[test]
    fn it_works_many_times() {
        let mut mol = Molecule::<i8>::new(vec![Atom::Data(2), Atom::Multiply, Atom::Data(2)]);
        mol.run(&mut std::collections::HashMap::new(), &mut String::new());
        assert_eq!(
            mol.run(&mut std::collections::HashMap::new(), &mut String::new())
                .unwrap(),
            (4, String::new())
        );

        assert_eq!(
            mol.run(&mut std::collections::HashMap::new(), &mut String::new())
                .unwrap(),
            mol.run(&mut std::collections::HashMap::new(), &mut String::new())
                .unwrap()
        );
    }

    #[test]
    fn it_outputs() {
        assert_eq!(
            Molecule::<i8>::new(vec![Atom::Output, Atom::Data(48)])
                .run(&mut std::collections::HashMap::new(), &mut String::new())
                .unwrap(),
            (48, "0".to_string())
        );
    }

    #[test]
    fn it_works_with_memory() {
        let mut hm = std::collections::HashMap::<i8, i8>::new();
        hm.insert(0, 48);
        assert_eq!(
            Molecule::<i8>::new(vec![Atom::Output, Atom::Memory, Atom::Data(0)])
                .run(&mut hm, &mut String::new())
                .unwrap(),
            (48, "0".to_string())
        );
        Molecule::<i8>::new(vec![Atom::Data(0), Atom::Assign, Atom::Data(13)])
            .run(&mut hm, &mut String::new());
        assert_eq!(hm, [(0, 13)].iter().cloned().collect());
        assert_eq!(
            Molecule::<i8>::new(vec![Atom::Memory, Atom::Data(13)])
                .run(&mut std::collections::HashMap::new(), &mut String::new())
                .unwrap(),
            (0, String::new())
        );
        hm.insert(0, 48);
        Molecule::<i8>::new(vec![
            Atom::Data(2),
            Atom::Add,
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
        let mut hm = std::collections::HashMap::<i8, i8>::new();
        hm.insert(0, 48);
        assert_eq!(
            Molecule::<i8>::new(Molecule::parse("▭◇▀").unwrap())
                .run(&mut hm, &mut String::new())
                .unwrap(),
            (48, "0".to_string())
        );
        Molecule::<i8>::new(Molecule::parse("▀◈▀▀▀▄▀").unwrap()).run(&mut hm, &mut String::new());
        assert_eq!(hm, [(0, 13)].iter().cloned().collect());
        assert_eq!(
            Molecule::<i8>::new(Molecule::parse("◇▀▀▀▄▀").unwrap())
                .run(&mut std::collections::HashMap::new(), &mut String::new())
                .unwrap(),
            (0, String::new())
        );
        hm.insert(0, 48);
        Molecule::<i8>::new(Molecule::parse("▀▀▄▐▀▀◈▀▀◈▀◈▀▀").unwrap())
            .run(&mut hm, &mut String::new());
        assert_eq!(hm, [(0, 1), (1, 1), (3, 1)].iter().cloned().collect());
    }

    #[test]
    fn it_works_complex() {
        assert_eq!(
            Molecule::<i16>::new(vec![
                Atom::Not,
                Atom::Data(0),
                Atom::Add,
                Atom::Data(1),
                Atom::Subtract,
                Atom::Data(2),
                Atom::Multiply,
                Atom::Data(3),
                Atom::Divide,
                Atom::Data(4),
                Atom::Modulo,
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
                Atom::InverseModulo,
                Atom::Data(11),
            ])
            .run(&mut std::collections::HashMap::new(), &mut String::new())
            .unwrap(),
            (-1, String::new())
        );

        assert_eq!(
            Molecule::<i8>::new(vec![
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
            .run(&mut std::collections::HashMap::new(), &mut String::new())
            .unwrap(),
            (1, String::new())
        );

        assert_eq!(
            Molecule::<i8>::new(vec![
                Atom::LeftParen,
                Atom::LeftParen,
                Atom::Data(0),
                Atom::Add,
                Atom::Data(2),
                Atom::RightParen,
                Atom::Modulo,
                Atom::Data(2),
                Atom::Modulo,
                Atom::Data(2),
                Atom::Subtract,
                Atom::Data(8),
                Atom::RightParen,
            ])
            .run(&mut std::collections::HashMap::new(), &mut String::new())
            .unwrap(),
            (-8, String::new())
        );
    }

    #[test]
    fn it_works_complex_with_parsing() {
        assert_eq!(
            Molecule::<i8>::new(
                Molecule::parse("▔▄▐▀▀▌▀▀▄▘▀▀▀▝▀▀▄▄▗▀▀▄▀▚▀▀▀▄▞▀▀▀▀▒▀▀▄▄▄▓▀▀▄▄▀░▀▀▄▀▄▖▀▀▄▀▀ ")
                    .unwrap()
            )
            .run(&mut std::collections::HashMap::new(), &mut String::new())
            .unwrap(),
            (-1, String::new())
        );

        assert_eq!(
            Molecule::<i8>::new(Molecule::parse("▀▄▨▀▀▧▀▀▄▤▀▄▥▄▀").unwrap())
                .run(&mut std::collections::HashMap::new(), &mut String::new())
                .unwrap(),
            (1, String::new())
        );

        assert_eq!(
            Molecule::<i8>::new(Molecule::parse("▕▕▀▄▐▀▀▄▏▖▀▀▄▖▀▀▄▌▀▀▄▄▄▏").unwrap())
                .run(&mut std::collections::HashMap::new(), &mut String::new())
                .unwrap(),
            (-8, String::new())
        );
    }
}
