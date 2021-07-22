use super::interpreter::Runnable;
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
}

#[derive(Clone, Debug, PartialEq)]
pub struct Molecule {
    children: Vec<Atom>,
    sorted_children: Option<Vec<Atom>>,
}

impl Molecule {
    pub fn new(children: Vec<Atom>) -> Molecule {
        Molecule {
            children,
            sorted_children: None,
        }
    }

    pub fn sort(&mut self) -> Result<Vec<Atom>, &str> {
        if self.sorted_children.is_none() {
            let mut output: Vec<Atom> = Vec::new();
            let mut stack: Vec<Atom> = Vec::new();

            for child in &self.children {
                if let Atom::Data(_) = *child {
                    output.push(*child);
                } else if *child == Atom::LeftParen || *child == Atom::Power {
                    // no operators are of higher precedence than exponentiation
                    // exponentiation is also right-associative
                    // so we can just push directly to the stack without looking at output
                    stack.push(*child);
                } else if *child == Atom::RightParen {
                    if !stack.iter().any(|atom| *atom == Atom::LeftParen) {
                        return Err("Unmatched right parenthesis");
                    }

                    while stack.last().cloned().unwrap() != Atom::LeftParen {
                        output.push(stack.pop().unwrap());
                    }

                    stack.pop();
                } else {
                    let precedence = child.precedence();

                    while !stack.is_empty()
                        && stack.last().cloned().unwrap().precedence() >= precedence
                    {
                        output.push(stack.pop().unwrap());
                    }
                    stack.push(*child);
                }
            }

            while !stack.is_empty() {
                output.push(stack.pop().unwrap());
            }

            if output.iter().any(|atom| *atom == Atom::LeftParen) {
                return Err("Unmatched left parenthesis");
            }

            self.sorted_children = Some(output);
        }

        Ok(self.sorted_children.as_ref().unwrap().to_vec())
    }
}

impl Runnable for Molecule {
    fn run(
        &mut self,
        memory: &mut HashMap<i128, i128>,
        stdout: &mut String,
    ) -> Result<(i128, String), &str> {
        let children = self.sort();

        if children.is_err() {
            return Err(children.err().unwrap());
        }

        let mut stack: Vec<i128> = vec![];
        for child in children.unwrap() {
            if let Atom::Data(num) = child {
                stack.push(num);
            } else if child == Atom::Memory {
                let a = stack.pop().unwrap();
                stack.push(*memory.get(&a).unwrap_or(&0));
            } else if child == Atom::Not {
                let a = stack.pop().unwrap();
                stack.push(!a);
            } else if child == Atom::Output {
                let a = stack.pop().unwrap();

                if let Some(chr) = std::char::from_u32(a as u32) {
                    stdout.push(chr);
                } else {
                    stdout.push('\u{ffff}');
                }

                stack.push(a);
            } else {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();

                if child == Atom::Assign {
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
    #[test]
    fn test_atom_precedence() {
        use super::Atom;

        assert_eq!(Atom::Assign.precedence(), 1);

        assert_eq!(Atom::LeftParen.precedence(), 0);

        assert_eq!(Atom::Difference.precedence(), 7);
    }

    #[test]
    fn test_molecule_new() {
        use super::{Atom, Molecule};

        assert_eq!(
            Molecule::new(vec![Atom::Data(1), Atom::Sum, Atom::Data(1),]),
            Molecule {
                children: vec![Atom::Data(1), Atom::Sum, Atom::Data(1),],
                sorted_children: None,
            }
        );
    }

    #[test]
    fn test_molecule_sort() {
        use super::{Atom, Molecule};

        assert_eq!(
            Molecule::new(vec![Atom::LeftParen]).sort(),
            Err("Unmatched left parenthesis")
        );

        assert_eq!(
            Molecule::new(vec![
                Atom::LeftParen,
                Atom::Data(3),
                Atom::Sum,
                Atom::Data(5),
                Atom::RightParen,
                Atom::Product,
                Atom::Data(7),
                Atom::RightParen,
            ])
            .sort(),
            Err("Unmatched right parenthesis")
        );

        assert_eq!(
            Molecule::new(vec![
                Atom::LeftParen,
                Atom::Data(3),
                Atom::Sum,
                Atom::Data(5),
                Atom::RightParen,
                Atom::Product,
                Atom::LeftParen,
                Atom::Data(2),
                Atom::Difference,
                Atom::Data(7),
                Atom::Quotient,
                Atom::Data(9),
                Atom::RightParen,
            ])
            .sort()
            .unwrap(),
            vec![
                Atom::Data(3),
                Atom::Data(5),
                Atom::Sum,
                Atom::Data(2),
                Atom::Data(7),
                Atom::Data(9),
                Atom::Quotient,
                Atom::Difference,
                Atom::Product,
            ]
        );

        assert_eq!(
            Molecule::new(vec![
                Atom::Data(1),
                Atom::Power,
                Atom::Data(2),
                Atom::Power,
                Atom::Data(3),
            ])
            .sort()
            .unwrap(),
            vec![
                Atom::Data(1),
                Atom::Data(2),
                Atom::Data(3),
                Atom::Power,
                Atom::Power,
            ]
        );

        assert_eq!(
            Molecule::new(vec![
                Atom::Data(0),
                Atom::LeftShift,
                Atom::Not,
                Atom::Data(1),
                Atom::Xor,
                Atom::Data(2),
                Atom::Or,
                Atom::Data(3),
                Atom::And,
                Atom::Data(4),
            ])
            .sort()
            .unwrap(),
            vec![
                Atom::Data(0),
                Atom::Data(1),
                Atom::Not,
                Atom::LeftShift,
                Atom::Data(2),
                Atom::Xor,
                Atom::Data(3),
                Atom::Data(4),
                Atom::And,
                Atom::Or,
            ]
        );

        let mut molecule = Molecule::new(vec![
            Atom::Data(2),
            Atom::Equal,
            Atom::Data(1),
            Atom::Sum,
            Atom::Data(1),
        ]);
        molecule.sort();
        assert_eq!(
            molecule.sort().unwrap(),
            vec![
                Atom::Data(2),
                Atom::Data(1),
                Atom::Data(1),
                Atom::Sum,
                Atom::Equal,
            ]
        );
    }

    #[test]
    fn test_molecule_run() {
        use super::{Atom, Molecule, Runnable};
        use std::collections::HashMap;

        assert_eq!(
            Molecule::new(vec![
                Atom::LeftParen,
                Atom::Data(3),
                Atom::Sum,
                Atom::Data(5),
                Atom::RightParen,
                Atom::Product,
                Atom::Data(7),
            ])
            .run(&mut HashMap::new(), &mut String::new()),
            Ok((56, String::new()))
        );

        assert_eq!(
            Molecule::new(vec![Atom::Output, Atom::Data(48),])
                .run(&mut HashMap::new(), &mut String::new()),
            Ok((48, "0".to_string()))
        );

        assert_eq!(
            Molecule::new(vec![Atom::Output, Atom::Memory, Atom::Data(0),])
                .run(&mut [(0, 48)].iter().cloned().collect(), &mut String::new()),
            Ok((48, "0".to_string()))
        );

        let mut mem: HashMap<i128, i128> = [(1, 2)].iter().cloned().collect();
        Molecule::new(vec![
            Atom::Data(0),
            Atom::Assign,
            Atom::Data(1),
            Atom::Assign,
            Atom::Data(2),
        ])
        .run(&mut mem, &mut String::new());
        assert_eq!(mem, [(0, 1), (1, 2)].iter().cloned().collect());

        assert_eq!(
            Molecule::new(vec![
                Atom::LeftParen,
                Atom::LeftParen,
                Atom::Data(0),
                Atom::Difference,
                Atom::Data(1),
                Atom::RightParen,
                Atom::Power,
                Atom::Data(2),
                Atom::Sum,
                Atom::Data(3),
                Atom::RightParen,
                Atom::Quotient,
                Atom::Data(2),
                Atom::Remainder,
                Atom::Data(3),
            ])
            .run(&mut HashMap::new(), &mut String::new()),
            Ok((2, String::new()))
        );

        assert_eq!(
            Molecule::new(vec![
                Atom::Data(1),
                Atom::LeftShift,
                Atom::Data(2),
                Atom::Xor,
                Atom::Data(3),
                Atom::RightShift,
                Atom::Data(4),
                Atom::And,
                Atom::Data(5),
                Atom::Or,
                Atom::Not,
                Atom::Data(6),
            ])
            .run(&mut HashMap::new(), &mut String::new()),
            Ok((-3, String::new()))
        );

        assert_eq!(
            Molecule::new(vec![
                Atom::Data(1),
                Atom::Less,
                Atom::Data(0),
                Atom::Greater,
                Atom::Data(3),
                Atom::NotEqual,
                Atom::Data(1),
                Atom::Equal,
                Atom::Data(1),
            ])
            .run(&mut HashMap::new(), &mut String::new()),
            Ok((1, String::new()))
        );

        assert_eq!(
            Molecule::new(vec![Atom::LeftParen,]).run(&mut HashMap::new(), &mut String::new()),
            Err("Unmatched left parenthesis")
        );

        assert_eq!(
            Molecule::new(vec![Atom::Output, Atom::Data(55296),])
                .run(&mut HashMap::new(), &mut String::new()),
            Ok((55296, "\u{ffff}".to_string()))
        );
    }
}
