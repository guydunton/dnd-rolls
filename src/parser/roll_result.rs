use colored::*;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct DiceRoll {
    dice: i32,
    result: i32,
}

impl DiceRoll {
    pub fn new(dice: i32, result: i32) -> DiceRoll {
        DiceRoll { dice, result }
    }

    pub fn result_to_string(&self) -> String {
        let as_string = self.result.to_string();
        if self.result == self.dice {
            return format!("{}", as_string.green());
        } else if self.result == 1 {
            return format!("{}", as_string.red());
        }

        as_string
    }
}

#[derive(Debug)]
pub struct RollResult {
    add_dice: Vec<DiceRoll>,
    sub_dice: Vec<DiceRoll>,
    add_constants: Vec<i32>,
    sub_constants: Vec<i32>,
}

impl RollResult {
    pub fn new() -> RollResult {
        RollResult {
            add_constants: vec![],
            add_dice: vec![],
            sub_dice: vec![],
            sub_constants: vec![],
        }
    }

    pub fn add_add_dice(&mut self, new_dice: &[DiceRoll]) {
        self.add_dice.extend_from_slice(new_dice);
    }

    pub fn add_add_constants(&mut self, new_constants: &[i32]) {
        self.add_constants.extend_from_slice(new_constants);
    }

    pub fn add_sub_dice(&mut self, new_dice: &[DiceRoll]) {
        self.sub_dice.extend_from_slice(new_dice);
    }

    pub fn add_sub_constants(&mut self, new_constants: &[i32]) {
        self.sub_constants.extend_from_slice(new_constants);
    }

    fn total_modifiers(&self) -> Option<i32> {
        Some(
            self.add_constants.iter().sum::<i32>()
                + self.sub_constants.iter().map(|c| c * -1).sum::<i32>(),
        )
        .filter(|&val| val != 0)
    }

    fn total(&self) -> i32 {
        let add_dice_total: i32 = self.add_dice.iter().map(|dice| dice.result).sum();
        let sub_dice_total: i32 = self.sub_dice.iter().map(|dice| dice.result).sum();

        add_dice_total - sub_dice_total + self.total_modifiers().unwrap_or(0)
    }

    fn print_dice(dice_rolls: &[DiceRoll]) -> Option<String> {
        // Just check that there are some dice
        if dice_rolls.is_empty() {
            return None;
        }

        // Get a list of all the unique dice
        let mut unique_dice: Vec<i32> = dice_rolls.iter().map(|dice| dice.dice).collect();
        unique_dice.dedup();

        // Go through the list and grab all the results
        let result = unique_dice
            .iter()
            .map(|&dice| {
                // Add to string e.g. D4(1, 2, 4)
                (
                    dice,
                    dice_rolls
                        .iter()
                        .filter(|roll| roll.dice == dice)
                        .map(|roll| roll.result_to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                )
            })
            // Join all strings e.g. D4(1, 2, 4), D20(16, 2)
            .map(|(dice, results)| format!("D{}({})", dice, results))
            .collect::<Vec<String>>()
            .join(", ");

        Some(result)
    }

    fn print_add_dice(&self) -> Option<String> {
        Self::print_dice(&self.add_dice)
    }

    fn print_sub_dice(&self) -> Option<String> {
        Self::print_dice(&self.sub_dice)
    }
}

impl Display for RollResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lines = [
            self.print_add_dice()
                .map(|dice| format!("[add dice]: {}", dice)),
            self.print_sub_dice()
                .map(|dice| format!("[sub dice]: {}", dice)),
            self.total_modifiers()
                .map(|modifiers| format!("[modifiers]: {}", modifiers)),
            Some(format!("[total]: {}", self.total())),
        ]
        .iter()
        .filter_map(|val| val.clone())
        .collect::<Vec<String>>()
        .join("\n");

        write!(f, "{}", lines)
    }
}
