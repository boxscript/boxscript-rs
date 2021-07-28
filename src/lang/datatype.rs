use num_traits::{PrimInt, Signed, ToPrimitive};

pub trait BoxInt: PrimInt + Signed + ToPrimitive + std::hash::Hash + std::fmt::Display {}

impl BoxInt for i8 {}
impl BoxInt for i16 {}
impl BoxInt for i32 {}
impl BoxInt for i64 {}
impl BoxInt for i128 {}
