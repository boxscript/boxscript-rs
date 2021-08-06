use super::interpreter::BoxInt;

#[derive(Clone, Copy)]
pub enum Genus {
    Loop,
    Condition,
    Execution,
    NoOp, // for comments
}

#[derive(Clone, Copy)]
pub struct Loc<T: BoxInt> {
    x: T,
    y: T,
}

#[derive(Clone, Copy)]
pub struct Box<T: BoxInt> {
    start: Loc<T>,
    end: Loc<T>,
    genus: Genus,
}

pub enum Relation {
    Parent,
    Child,
    Other, // cannot guarantee siblingship since they could have different parents
}

impl<T: BoxInt> Box<T> {
    pub fn new(start: Loc<T>, end: Loc<T>, genus: Genus) -> Result<Box<T>, String> {
        if start.x > end.x || start.y > end.y {
            Err(format!(
                "Box start ({}, {}) is greater than end ({}, {})",
                start.x, start.y, end.x, end.y
            ))
        } else {
            Ok(Box { start, end, genus })
        }
    }

    pub fn relationship(&self, other: &Box<T>) -> Relation {
        // self is a ___ of other
        if self.start.x >= other.start.x
            && self.end.x <= other.end.x
            && self.start.y >= other.start.y
            && self.end.y <= other.end.y
        {
            Relation::Child
        } else if self.start.x <= other.start.x
            && self.end.x >= other.end.x
            && self.start.y <= other.start.y
            && self.end.y >= other.end.y
        {
            Relation::Parent
        } else {
            Relation::Other
        }
    }

    // these should be used to determine execution order (i.e. (a)sync)
    pub fn before(&self, &other: &Box<T>) -> bool {
        self.end.y < other.start.y
    }

    pub fn after(&self, &other: &Box<T>) -> bool {
        self.start.y > other.end.y
    }

    pub fn simultaneous(&self, &other: &Box<T>) -> bool {
        self.end.y >= other.start.y && self.start.y <= other.end.y
    }
}
