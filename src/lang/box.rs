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

impl<T: BoxInt> Box<T> {
    // below two methods are used for hierarchies
    pub fn inside(&self, &other: &Box<T>) -> bool {
        self.start.x >= other.start.x
            && self.end.x <= other.end.x
            && self.start.y >= other.start.y
            && self.end.y <= other.end.y
    }

    pub fn contains(&self, &other: &Box<T>) -> bool {
        self.start.x <= other.start.x
            && self.end.x >= other.end.x
            && self.start.y <= other.start.y
            && self.end.y >= other.end.y
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
