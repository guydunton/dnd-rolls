mod ast;
mod interpreter;
mod parser;
mod roll_result;

use interpreter::{tree_to_rpn, Token};
use parser::parse;
use rand::Rng;
use roll_result::{DiceRoll, RollResult};
use serde_json::json;

use wasm_bindgen::prelude::*;

fn tokens_to_roll_result<T: Rng>(tokens: &[Token], rnd: &mut T) -> RollResult {
    let mut result = RollResult::new();

    // Create temp queue for tokens
    let mut dice_queue = vec![];
    let mut constant_queue = vec![];

    // Reverse through the tokens
    for token in tokens.iter().rev() {
        // If the token is a dice or constant push into queue
        match token {
            Token::Dice(d) => dice_queue.push(DiceRoll::new(*d, rnd.gen_range(1..d + 1))),
            Token::Constant(c) => constant_queue.push(*c),
            Token::Op(o) => {
                match o {
                    ast::Operator::Plus => {
                        // Push queues into plus result
                        result.add_add_dice(&dice_queue[..]);
                        result.add_add_constants(&constant_queue[..]);
                    }
                    ast::Operator::Minus => {
                        // Push queues into sub result
                        result.add_sub_dice(&dice_queue[..]);
                        result.add_sub_constants(&constant_queue[..]);
                    }
                }
                dice_queue = vec![];
                constant_queue = vec![];
            }
        }
    }

    result
}

fn dnd_roll_random<T: Rng>(text: &str, rnd: &mut T) -> Option<RollResult> {
    // Parse the text into an expression
    let expression = parse(text)?;

    // Convert tree to reverse polish notation
    let tokens = tree_to_rpn(&expression);

    // Create a roll result
    Some(tokens_to_roll_result(&tokens, rnd))
}

pub fn dnd_roll(text: &str) -> Option<RollResult> {
    let mut rnd = rand::thread_rng();
    dnd_roll_random(text, &mut rnd)
}

#[wasm_bindgen]
pub fn dnd_roll_str(text: &str) -> Option<String> {
    dnd_roll(text).map(|result| format!("{}", result))
}

#[wasm_bindgen]
pub fn dnd_roll_json(text: &str) -> Option<String> {
    dnd_roll(text).map(|result| {
        let add_dice: Vec<serde_json::Value> = result
            .get_add_dice_types()
            .iter()
            .map(|dice| {
                json!({
                    "dice": dice,
                    "rolls": result.get_add_dice_results(*dice)
                })
            })
            .collect();

        let sub_dice: Vec<serde_json::Value> = result
            .get_sub_dice_types()
            .iter()
            .map(|dice| {
                json!({
                    "dice": dice,
                    "rolls": result.get_sub_dice_results(*dice)
                })
            })
            .collect();
        json!({
            "add_dice": add_dice,
            "sub_dice": sub_dice,
            "modifier": result.total_modifiers(),
            "total": result.total(),
        })
        .to_string()
    })
}
