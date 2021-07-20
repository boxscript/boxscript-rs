use super::interpreter::Runnable;
use std::collections::HashMap;

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

#[derive(Clone, Copy)]
pub struct Atom {
    pub token: Token,
    pub value: Option<i128>,
}

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

    fn precedence(atom: &Atom) -> u8 {
        match atom.token {
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

    pub fn sort(&'a mut self) -> Result<bool,&str> {
        if self.sorted_children.is_some() {
            return Ok(true);
        }

        let mut output: Vec<&Atom> = Vec::new();
        let mut stack: Vec<&Atom> = Vec::new();

        for child in &self.children {
            if child.token == Token::NUM {
                output.push(child);
            } else if child.token == Token::LPAREN {
                stack.push(child);
            } else if child.token == Token::RPAREN {
                if !stack.iter().any(|atom| atom.token == Token::LPAREN) {
                    return Err("Unmatched parenthesis");
                }

                while stack.last().cloned().unwrap().token != Token::LPAREN {
                    output.push(stack.pop().unwrap());
                }

                stack.pop();
            } else {
                for i in 1..11 {
                    if Molecule::precedence(&child) == i {
                        while stack.iter().any(|atom| Molecule::precedence(&atom) >= i)
                            && stack.last().cloned().unwrap().token != Token::LPAREN
                        {
                            output.push(stack.pop().unwrap());
                        }
                        stack.push(child);
                    }
                }
            }
        }

        while stack.len() != 0 {
            output.push(stack.pop().unwrap());
        }

        self.sorted_children = Some(output);

        Ok(false)
    }
}
