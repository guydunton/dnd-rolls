mod ast;
mod interpreter;
mod parser;
mod roll_result;

use interpreter::{tree_to_rpn, Token};
use parser::parse;
use rand::prelude::*;
use roll_result::{DiceRoll, RollResult};
use std::env;

fn tokens_to_roll_result<T: Rng>(tokens: &Vec<Token>, rnd: &mut T) -> RollResult {
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

fn dnd_roll(text: &str) -> Option<RollResult> {
    let mut rnd = rand::thread_rng();
    dnd_roll_random(text, &mut rnd)
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let result = args
        .get(1)
        .and_then(|arg| dnd_roll(arg))
        .map(|roll_result| format!("{}", roll_result))
        .unwrap_or("Failed to parse roll. Provide argument e.g. '2d20 + 2'".to_string());

    println!("{}", result);
}
