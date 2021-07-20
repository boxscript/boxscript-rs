#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Token {
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
    NUM,
    MEM,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Atom {
    pub token: Token,
    pub value: Option<i128>,
}

impl Atom {
    pub fn precedence(&self) -> u8 {
        match self.token {
            Token::OUTPUT | Token::ASSIGN => 1,
            Token::LT | Token::GT | Token::EQ | Token::NE => 2,
            Token::OR => 3,
            Token::XOR => 4,
            Token::AND => 5,
            Token::LSHIFT | Token::RSHIFT => 6,
            Token::ADD | Token::SUB => 7,
            Token::MUL | Token::DIV | Token::MOD => 8,
            Token::MEM | Token::NOT => 9,
            Token::POW => 10,
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
            children: children,
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
            if child.token == Token::NUM {
                output.push(child);
            } else if child.token == Token::POW {
                stack.push(child);
            } else if child.token == Token::LPAREN {
                stack.push(child);
            } else if child.token == Token::RPAREN {
                if !stack.iter().any(|atom| atom.token == Token::LPAREN) {
                    return Err("Unmatched right parenthesis");
                }

                while stack.last().cloned().unwrap().token != Token::LPAREN {
                    output.push(stack.pop().unwrap());
                }

                stack.pop();
            } else {
                for i in 1..11 {
                    if child.precedence() == i {
                        while stack.len() > 0 && stack.last().cloned().unwrap().precedence() >= i {
                            output.push(stack.pop().unwrap());
                        }
                        stack.push(child);
                        break;
                    }
                }
            }
        }

        while stack.len() != 0 {
            output.push(stack.pop().unwrap());
        }

        if output.iter().any(|atom| atom.token == Token::LPAREN) {
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
        use super::{Atom, Token};

        assert_eq!(
            Atom {
                token: Token::ASSIGN,
                value: None,
            }
            .precedence(),
            1
        );

        assert_eq!(
            Atom {
                token: Token::LPAREN,
                value: None,
            }
            .precedence(),
            0
        );

        assert_eq!(
            Atom {
                token: Token::SUB,
                value: None,
            }
            .precedence(),
            7
        );
    }

    #[test]
    fn test_molecule_new() {
        use super::{Atom, Molecule, Token};

        assert_eq!(
            Molecule::new(vec![
                Atom {
                    token: Token::NUM,
                    value: Some(1),
                },
                Atom {
                    token: Token::ADD,
                    value: None,
                },
                Atom {
                    token: Token::NUM,
                    value: Some(1),
                },
            ]),
            Molecule {
                children: vec![
                    Atom {
                        token: Token::NUM,
                        value: Some(1),
                    },
                    Atom {
                        token: Token::ADD,
                        value: None,
                    },
                    Atom {
                        token: Token::NUM,
                        value: Some(1),
                    },
                ],
                sorted_children: None,
            }
        );
    }

    #[test]
    fn test_molecule_sort() {
        use super::{Atom, Molecule, Token};

        assert_eq!(
            Molecule::new(vec![Atom {
                token: Token::LPAREN,
                value: None,
            }])
            .sort(),
            Err("Unmatched left parenthesis")
        );

        assert_eq!(
            Molecule::new(vec![
                Atom {
                    token: Token::LPAREN,
                    value: None,
                },
                Atom {
                    token: Token::NUM,
                    value: Some(3),
                },
                Atom {
                    token: Token::ADD,
                    value: None,
                },
                Atom {
                    token: Token::NUM,
                    value: Some(5),
                },
                Atom {
                    token: Token::RPAREN,
                    value: None,
                },
                Atom {
                    token: Token::MUL,
                    value: None,
                },
                Atom {
                    token: Token::NUM,
                    value: Some(2),
                },
                Atom {
                    token: Token::RPAREN,
                    value: None,
                },
            ])
            .sort(),
            Err("Unmatched right parenthesis")
        );

        let mut molecule = Molecule::new(vec![
            Atom {
                token: Token::LPAREN,
                value: None,
            },
            Atom {
                token: Token::NUM,
                value: Some(3),
            },
            Atom {
                token: Token::ADD,
                value: None,
            },
            Atom {
                token: Token::NUM,
                value: Some(5),
            },
            Atom {
                token: Token::RPAREN,
                value: None,
            },
            Atom {
                token: Token::MUL,
                value: None,
            },
            Atom {
                token: Token::LPAREN,
                value: None,
            },
            Atom {
                token: Token::NUM,
                value: Some(2),
            },
            Atom {
                token: Token::SUB,
                value: None,
            },
            Atom {
                token: Token::NUM,
                value: Some(7),
            },
            Atom {
                token: Token::DIV,
                value: None,
            },
            Atom {
                token: Token::NUM,
                value: Some(9),
            },
            Atom {
                token: Token::RPAREN,
                value: None,
            },
        ]);
        assert_eq!(
            molecule.sort().unwrap().iter().map(|atom| **atom).collect::<Vec<Atom>>(),
            vec![
                Atom {
                    token: Token::NUM,
                    value: Some(3),
                },
                Atom {
                    token: Token::NUM,
                    value: Some(5),
                },
                Atom {
                    token: Token::ADD,
                    value: None,
                },
                Atom {
                    token: Token::NUM,
                    value: Some(2),
                },
                Atom {
                    token: Token::NUM,
                    value: Some(7),
                },
                Atom {
                    token: Token::NUM,
                    value: Some(9),
                },
                Atom {
                    token: Token::DIV,
                    value: None,
                },
                Atom {
                    token: Token::SUB,
                    value: None,
                },
                Atom {
                    token: Token::MUL,
                    value: None,
                },
            ]
        );

        let mut molecule = Molecule::new(vec![
            Atom {
                token: Token::NUM,
                value: Some(1),
            },
            Atom {
                token: Token::POW,
                value: None,
            },
            Atom {
                token: Token::NUM,
                value: Some(2),
            },
            Atom {
                token: Token::POW,
                value: None,
            },
            Atom {
                token: Token::NUM,
                value: Some(3),
            },
        ]);
        assert_eq!(
            molecule.sort().unwrap().iter().map(|atom| **atom).collect::<Vec<Atom>>(),
            vec![
                Atom {
                    token: Token::NUM,
                    value: Some(1),
                },
                Atom {
                    token: Token::NUM,
                    value: Some(2),
                },
                Atom {
                    token: Token::NUM,
                    value: Some(3),
                },
                Atom {
                    token: Token::POW,
                    value: None,
                },
                Atom {
                    token: Token::POW,
                    value: None,
                },
            ]
        );
    }
}
