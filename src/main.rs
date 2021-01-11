mod dice;
mod dice_roll;
mod roll_result;

use dice::Dice;
use dice_roll::DiceRoll;
use nom::branch::alt;
use nom::bytes::complete::take_while;
use nom::character::{is_digit, streaming::char};
use nom::combinator::map_res;
use nom::sequence::{preceded, tuple};
use nom::IResult;
use rand::prelude::*;
use roll_result::RollResult;
use std::env;
use u8;

fn dice_count(text: &[u8]) -> IResult<&[u8], Option<u8>> {
    let (input, count) = take_while(is_digit)(text)?;

    let result = std::str::from_utf8(count)
        .ok()
        .and_then(|as_str| u8::from_str_radix(as_str, 10).ok());

    Ok((input, result))
}

fn str_to_dice(text: &[u8]) -> Result<Dice, &str> {
    std::str::from_utf8(text)
        .map_err(|_| "Couldn't convert array to str")
        .and_then(|v| match v {
            "2" => Ok(Dice::D2),
            "4" => Ok(Dice::D4),
            "6" => Ok(Dice::D6),
            "8" => Ok(Dice::D8),
            "10" => Ok(Dice::D10),
            "12" => Ok(Dice::D12),
            "20" => Ok(Dice::D20),
            "100" => Ok(Dice::D100),
            _ => Err("Failed to parse dice"),
        })
}

fn text_to_dice(text: &[u8]) -> IResult<&[u8], Dice> {
    let dice_parser = preceded(
        alt((char('D'), char('d'))),
        nom::character::complete::digit1,
    );

    map_res(dice_parser, str_to_dice)(text)
}

fn dnd_roll_random<T: Rng>(text: &str, rnd: &mut T) -> Option<RollResult> {
    let input = text.as_bytes();

    let (_, (maybe_count, dice)): (&[u8], (Option<u8>, Dice)) =
        tuple((dice_count, text_to_dice))(input).ok()?;

    let count = maybe_count.unwrap_or(1);

    let rolls = (0..count)
        .map(|_| DiceRoll {
            dice,
            result: dice.roll(rnd),
        })
        .collect();

    Some(RollResult { rolls })
}

fn dnd_roll(text: &str) -> Option<RollResult> {
    let mut rnd = rand::thread_rng();
    dnd_roll_random(text, &mut rnd)
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let arg = args.get(1);
    match arg {
        Some(roll) => {
            println!("Result is {}", dnd_roll(roll).unwrap());
        }
        None => {
            println!("Provide a roll e.g. 2d20");
        }
    }
}

macro_rules! assert_between {
    ($value:expr, $smallest:expr, $biggest:expr) => {{
        assert!($value >= $smallest);
        assert!($value <= $biggest);
    }};
}

#[test]
fn incorrect_values() {
    assert_eq!(dnd_roll("d"), None);
    assert_eq!(dnd_roll("dd"), None);
}

#[test]
fn simple_rolls() {
    let result = dnd_roll("d4").unwrap();
    assert_eq!(result.rolls.len(), 1);
    assert_eq!(result.rolls[0].dice, Dice::D4);
    assert_between!(result.rolls[0].result, 1, 4);

    let result = dnd_roll("d6").unwrap();
    assert_eq!(result.rolls[0].dice, Dice::D6);
    assert_between!(result.rolls[0].result, 1, 6);

    let result = dnd_roll("d8").unwrap();
    assert_eq!(result.rolls[0].dice, Dice::D8);
    assert_between!(result.rolls[0].result, 1, 8);

    let result = dnd_roll("d10").unwrap();
    assert_eq!(result.rolls[0].dice, Dice::D10);
    assert_between!(result.rolls[0].result, 1, 10);

    let result = dnd_roll("d12").unwrap();
    assert_eq!(result.rolls[0].dice, Dice::D12);
    assert_between!(result.rolls[0].result, 1, 12);

    let result = dnd_roll("d20").unwrap();
    assert_eq!(result.rolls[0].dice, Dice::D20);
    assert_between!(result.rolls[0].result, 1, 20);

    let result = dnd_roll("d100").unwrap();
    assert_eq!(result.rolls[0].dice, Dice::D100);
    assert_between!(result.rolls[0].result, 1, 100);
}

#[test]
fn test_uppercase() {
    let result = dnd_roll("D8").unwrap();
    assert_eq!(result.rolls[0].dice, Dice::D8);
}

#[test]
fn test_specify_single_dice() {
    let result = dnd_roll("1d6").unwrap();
    assert_eq!(result.rolls.len(), 1);
}

#[test]
fn test_single_dice_formatting() {
    let mut rnd = StdRng::seed_from_u64(0x1);
    let result = dnd_roll_random("1d6", &mut rnd).unwrap();
    let as_text = format!("{}", result);

    assert_eq!(as_text, "D6 = (5)");
}

#[test]
fn test_multiple_dice() {
    let mut rnd = StdRng::seed_from_u64(0x1);
    let result = dnd_roll_random("2d6", &mut rnd).unwrap();
    assert_eq!(result.rolls.len(), 2);

    let as_text = format!("{}", result);
    assert_eq!(
        as_text,
        "2D6 = (5, 2)\nHighest(5), Lowest(2), Total(7)".to_owned()
    );
}
