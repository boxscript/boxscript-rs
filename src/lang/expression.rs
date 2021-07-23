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

    pub fn validate<'a>(children: &'a [Atom], valid: &mut bool) -> Result<(), &'a str> {
        if !*valid {
            let mut list: Vec<AtomType> = vec![];
            for child in children {
                if let Atom::LeftParen | Atom::RightParen = child {
                } else {
                    list.push(Atom::form(child));
                }
            }

            if list.len() == 1 && list[0] != AtomType::Number
                || list.len() == 2 && (list[0] != AtomType::Unary || list[1] != AtomType::Number)
            {
                return Err("Malformed expression");
            }
            *valid = true;

            for i in 0..list.len() {
                if i == 0 {
                    *valid &= list[i] == AtomType::Number && list[i + 1] == AtomType::Binary
                        || list[i] == AtomType::Unary && list[i + 1] != AtomType::Binary;
                } else if i == list.len() - 1 {
                    *valid &= (list[i - 1] == AtomType::Binary || list[i - 1] == AtomType::Unary)
                        && list[i] == AtomType::Number;
                } else {
                    *valid &= match list[i] {
                        AtomType::Number => {
                            list[i - 1] != AtomType::Number && list[i + 1] != AtomType::Number
                        }
                        AtomType::Unary => {
                            list[i - 1] != AtomType::Number && list[i + 1] != AtomType::Binary
                        }
                        AtomType::Binary => {
                            list[i - 1] == AtomType::Number && list[i + 1] != AtomType::Binary
                        }
                    };
                }
            }

            if !*valid {
                return Err("Malformed expression");
            }

            Ok(())
        } else {
            Ok(())
        }
    }

    pub fn sort<'a>(
        children: &'a [Atom],
        sorted: &'a mut Option<Vec<Atom>>,
    ) -> Result<Vec<Atom>, &'a str> {
        if sorted.is_none() {
            let mut output: Vec<Atom> = Vec::new();
            let mut stack: Vec<Atom> = Vec::new();

            for child in children {
                if let Atom::Data(_) = *child {
                    output.push(*child);
                } else if let Atom::LeftParen | Atom::Power | Atom::Not | Atom::Memory = *child {
                    // no operators are of higher precedence than exponentiation
                    // exponentiation is also right-associative
                    // so we can just push directly to the stack without looking at output
                    // unary prefix operators are also pushed to stack
                    stack.push(*child);
                } else if let Atom::RightParen = *child {
                    while !stack.is_empty() && stack.last().cloned().unwrap() != Atom::LeftParen {
                        output.push(stack.pop().unwrap());
                    }

                    if stack.is_empty() || stack.last().cloned().unwrap() != Atom::LeftParen {
                        return Err("Malformed expression");
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
                if let Atom::LeftParen = stack.last().cloned().unwrap() {
                    return Err("Malformed expression");
                }

                output.push(stack.pop().unwrap());
            }

            *sorted = Some(output);
        }

        Ok(sorted.as_ref().unwrap().to_vec())
    }
}

impl Runnable for Molecule {
    fn run(
        &mut self,
        memory: &mut HashMap<i128, i128>,
        stdout: &mut String,
    ) -> Result<(i128, String), &str> {
        let validity = Molecule::validate(&self.children, &mut self.valid);

        if validity.is_err() {
            return Err(validity.err().unwrap());
        }
        let children = Molecule::sort(&self.children, &mut self.sorted_children);

        if children.is_err() {
            return Err(children.err().unwrap());
        }

        let mut stack: Vec<i128> = vec![];
        for child in children.unwrap() {
            if let Atom::Data(num) = child {
                stack.push(num);
            } else if let Atom::Memory | Atom::Not | Atom::Output = child {
                if stack.is_empty() {
                    return Err("Malformed expression");
                }

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
                if stack.len() < 2 {
                    return Err("Malformed expression");
                }
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

        if stack.len() > 1 {
            return Err("Malformed expression");
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
    fn it_detects_bad_chars() {
        assert_eq!(
            Molecule::new(vec![Atom::Output, Atom::Data(55296),])
                .run(&mut HashMap::new(), &mut String::new()),
            Ok((55296, "\u{ffff}".to_string()))
        );
    }
}
