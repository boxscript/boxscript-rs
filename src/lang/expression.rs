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

#[derive(Debug, PartialEq)]
pub struct Molecule<'a> {
    children: Vec<Atom>,
    sorted_children: Option<Vec<&'a Atom>>,
}

impl<'a> Molecule<'a> {
    pub fn new(children: Vec<Atom>) -> Molecule<'a> {
        Molecule {
            children,
            sorted_children: None,
        }
    }

    pub fn sort(&'a mut self) -> Result<Vec<&'a Atom>, &str> {
        if self.sorted_children.is_some() {
            return Ok(self.sorted_children.as_ref().unwrap().to_vec());
        }

        let mut output: Vec<&Atom> = Vec::new();
        let mut stack: Vec<&Atom> = Vec::new();

        for child in &self.children {
            if let Atom::Data(_) = *child {
                output.push(child);
            } else if *child == Atom::Power {
                while !stack.is_empty()
                    && stack.last().cloned().unwrap().precedence() > child.precedence()
                {
                    output.push(stack.pop().unwrap());
                }
                stack.push(child);
            } else if *child == Atom::LeftParen {
                stack.push(child);
            } else if *child == Atom::RightParen {
                if !stack.iter().any(|atom| **atom == Atom::LeftParen) {
                    return Err("Unmatched right parenthesis");
                }

                while *stack.last().cloned().unwrap() != Atom::LeftParen {
                    output.push(stack.pop().unwrap());
                }

                stack.pop();
            } else {
                for i in 1..11 {
                    if child.precedence() == i {
                        while !stack.is_empty() && stack.last().cloned().unwrap().precedence() >= i
                        {
                            output.push(stack.pop().unwrap());
                        }
                        stack.push(child);
                        break;
                    }
                }
            }
        }

        while !stack.is_empty() {
            output.push(stack.pop().unwrap());
        }

        if output.iter().any(|atom| **atom == Atom::LeftParen) {
            return Err("Unmatched left parenthesis");
        }

        self.sorted_children = Some(output);

        Ok(self.sorted_children.as_ref().unwrap().to_vec())
    }
}

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
            .unwrap()
            .iter()
            .map(|atom| **atom)
            .collect::<Vec<Atom>>(),
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
            .unwrap()
            .iter()
            .map(|atom| **atom)
            .collect::<Vec<Atom>>(),
            vec![
                Atom::Data(1),
                Atom::Data(2),
                Atom::Data(3),
                Atom::Power,
                Atom::Power,
            ]
        );
    }
}
