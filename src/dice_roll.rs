use super::Dice;
use colored::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct DiceRoll {
    pub dice: Dice,
    pub result: i32,
}

impl DiceRoll {
    pub fn result_string(&self) -> String {
        let text = format!("{}", self.result);

        if self.result == self.dice.max() {
            format!("{}", text.green())
        } else if self.result == self.dice.min() {
            format!("{}", text.red())
        } else {
            text
        }
    }
}
