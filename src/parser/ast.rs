/*

D6

D6+3
3D6

Dis D20

Ad D20

4D4

Ad D20 + 4

2D10 + 1D8

*/

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Operator {
    Plus,
    Minus,
}

#[derive(Debug, PartialEq)]
pub struct Operation {
    pub left: Box<Exp>,
    pub right: Box<Exp>,
    pub operator: Operator,
}

pub type Dice = u8;

#[derive(Debug, PartialEq)]
pub struct MultiDice {
    pub count: i32,
    pub dice: Box<Exp>,
}

#[derive(Debug, PartialEq)]
pub enum Exp {
    Constant(i32),
    Dice(Dice),
    MultiDice(MultiDice),
    Op(Operation),
    // Advantage(Box<Exp>),
    // Disadvantage(Box<Exp>),
}
