use super::DiceRoll;
use std::fmt;

#[derive(Clone, PartialEq, Debug)]
pub struct RollResult {
    pub rolls: Vec<DiceRoll>,
}

impl RollResult {
    fn highest(&self) -> DiceRoll {
        self.rolls
            .iter()
            .max_by(|left, right| left.result.cmp(&right.result))
            .map(|r| r.clone())
            .unwrap()
    }

    fn lowest(&self) -> DiceRoll {
        self.rolls
            .iter()
            .min_by(|left, right| left.result.cmp(&right.result))
            .map(|roll| roll.clone())
            .unwrap()
    }

    fn total(&self) -> i32 {
        self.rolls.iter().map(|roll| roll.result).sum()
    }

    fn rolls_as_string(&self) -> String {
        self.rolls
            .iter()
            .map(|roll| roll.result_string())
            .collect::<Vec<String>>()
            .join(", ")
    }

    fn count(&self) -> usize {
        self.rolls.len()
    }
}

impl fmt::Display for RollResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.rolls.len() > 1 {
            let count = self.count();
            let rolls = self.rolls_as_string();
            let highest = self.highest().result_string();
            let lowest = self.lowest().result_string();
            let total = self.total();
            return write!(
                f,
                "{}{} = ({})\nHighest({}), Lowest({}), Total({})",
                count, self.rolls[0].dice, rolls, highest, lowest, total
            );
        } else {
            return write!(
                f,
                "{} = ({})",
                self.rolls[0].dice,
                self.rolls[0].result_string()
            );
        }
    }
}
