#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Atom {
    GT,
    LT,
    EQ,
    NE,
    ASSIGN,
    NOT,
    AND,
    OR,
    XOR,
    LSHIFT,
    RSHIFT,
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    POW,
    LPAREN,
    RPAREN,
    OUTPUT,
    NUM(i128),
    MEM,
}

impl Atom {
    pub fn precedence(&self) -> u8 {
        match self {
            Atom::OUTPUT | Atom::ASSIGN => 1,
            Atom::LT | Atom::GT | Atom::EQ | Atom::NE => 2,
            Atom::OR => 3,
            Atom::XOR => 4,
            Atom::AND => 5,
            Atom::LSHIFT | Atom::RSHIFT => 6,
            Atom::ADD | Atom::SUB => 7,
            Atom::MUL | Atom::DIV | Atom::MOD => 8,
            Atom::MEM | Atom::NOT => 9,
            Atom::POW => 10,
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
            if let Atom::NUM(_) = *child {
                output.push(child);
            } else if *child == Atom::POW {
                while !stack.is_empty()
                    && stack.last().cloned().unwrap().precedence() > child.precedence()
                {
                    output.push(stack.pop().unwrap());
                }
                stack.push(child);
            } else if *child == Atom::LPAREN {
                stack.push(child);
            } else if *child == Atom::RPAREN {
                if !stack.iter().any(|atom| **atom == Atom::LPAREN) {
                    return Err("Unmatched right parenthesis");
                }

                while *stack.last().cloned().unwrap() != Atom::LPAREN {
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

        if output.iter().any(|atom| **atom == Atom::LPAREN) {
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

        assert_eq!(Atom::ASSIGN.precedence(), 1);

        assert_eq!(Atom::LPAREN.precedence(), 0);

        assert_eq!(Atom::SUB.precedence(), 7);
    }

    #[test]
    fn test_molecule_new() {
        use super::{Atom, Molecule};

        assert_eq!(
            Molecule::new(vec![Atom::NUM(1), Atom::ADD, Atom::NUM(1),]),
            Molecule {
                children: vec![Atom::NUM(1), Atom::ADD, Atom::NUM(1),],
                sorted_children: None,
            }
        );
    }

    #[test]
    fn test_molecule_sort() {
        use super::{Atom, Molecule};

        assert_eq!(
            Molecule::new(vec![Atom::LPAREN]).sort(),
            Err("Unmatched left parenthesis")
        );

        assert_eq!(
            Molecule::new(vec![
                Atom::LPAREN,
                Atom::NUM(3),
                Atom::ADD,
                Atom::NUM(5),
                Atom::RPAREN,
                Atom::MUL,
                Atom::NUM(7),
                Atom::RPAREN,
            ])
            .sort(),
            Err("Unmatched right parenthesis")
        );

        let mut molecule = Molecule::new(vec![
            Atom::LPAREN,
            Atom::NUM(3),
            Atom::ADD,
            Atom::NUM(5),
            Atom::RPAREN,
            Atom::MUL,
            Atom::LPAREN,
            Atom::NUM(2),
            Atom::SUB,
            Atom::NUM(7),
            Atom::DIV,
            Atom::NUM(9),
            Atom::RPAREN,
        ]);
        assert_eq!(
            molecule
                .sort()
                .unwrap()
                .iter()
                .map(|atom| **atom)
                .collect::<Vec<Atom>>(),
            vec![
                Atom::NUM(3),
                Atom::NUM(5),
                Atom::ADD,
                Atom::NUM(2),
                Atom::NUM(7),
                Atom::NUM(9),
                Atom::DIV,
                Atom::SUB,
                Atom::MUL,
            ]
        );

        let mut molecule = Molecule::new(vec![
            Atom::NUM(1),
            Atom::POW,
            Atom::NUM(2),
            Atom::POW,
            Atom::NUM(3),
        ]);
        assert_eq!(
            molecule
                .sort()
                .unwrap()
                .iter()
                .map(|atom| **atom)
                .collect::<Vec<Atom>>(),
            vec![
                Atom::NUM(1),
                Atom::NUM(2),
                Atom::NUM(3),
                Atom::POW,
                Atom::POW,
            ]
        );
    }
}
