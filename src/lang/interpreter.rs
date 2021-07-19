use std::collections::HashMap;

pub trait Runnable {
    fn run(&self, memory: HashMap<i128, i128>) -> i128;
}

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
}
