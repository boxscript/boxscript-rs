use super::interpreter::Runnable;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

pub struct Atom {
    pub token: Token,
    pub value: Option<i128>,
}

pub struct Molecule<'mem> {
    children: Vec<Atom>,
    sorted_children: Option<Vec<Atom>>,
}

impl Molecule<'_> {
    pub fn new(children: Vec<Atom>) -> Molecule {
        Molecule {
            children: children,
            sorted_children: None,
        }
    }

    pub fn sort(&mut self) -> Result<()> {
        if self.sorted_children.is_some() {
            return;
        }

        let precedence: HashMap<Token, u8> = [
            (Token::OUTPUT, 1),
            (Token::ASSIGN, 1),
            (Token::LT, 2),
            (Token::GT, 2),
            (Token::EQ, 2),
            (Token::NE, 2),
            (Token::OR, 3),
            (Token::XOR, 4),
            (Token::AND, 5),
            (Token::LSHIFT, 6),
            (Token::RSHIFT, 6),
            (Token::ADD, 7),
            (Token::SUB, 7),
            (Token::MUL, 8),
            (Token::DIV, 8),
            (Token::MOD, 8),
            (Token::MEM, 9),
            (Token::NOT, 9),
            (Token::POW, 10),
        ]
        .iter()
        .cloned()
        .collect();

        let mut output: Vec<Atom> = Vec::new();
        let mut stack: Vec<Atom> = Vec::new();

        for child in self.children {
            if child.token == Token::NUM {
                output.push(child);
            } else if child.token == Token::LPAREN {
                stack.push(child);
            } else if child.token == Token::RPAREN {
                if !stack.into_iter().any(|atom| atom.token == Token::LPAREN) {
                    return Err("Unmatched parenthesis");
                }

                while stack.last().copied().token != Token::LPAREN {
                    output.push(stack.pop().unwrap());
                }

                stack.pop();
            } else {
                for i in 1..11 {
                    if precedence[child.token] == i {
                        while stack.into_iter().any(|atom| precedence[atom.token] >= i)
                            && stack.last().copied().token != Token::LPAREN
                        {
                            output.push(stack.pop().unwrap());
                        }
                        stack.push(child);
                    }
                }
            }
        }

        while stack.size() != 0 {
            output.push(stack.pop().unwrap());
        }

        self.sorted_children = Some(output);
    }
}
