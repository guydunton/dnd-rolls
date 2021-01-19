use crate::ast::{Exp, MultiDice, Operation, Operator};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::space0,
    sequence::delimited,
};
use nom::{
    character::{complete::char, is_digit},
    combinator::map_opt,
};
use nom::{sequence::tuple, IResult};

fn str_to_u8(str: &[u8]) -> Option<u8> {
    let as_text = std::str::from_utf8(str).ok()?;
    as_text.parse().ok()
}

fn dice_parse(text: &[u8]) -> IResult<&[u8], Exp> {
    let str_to_dice = |str| str_to_u8(str).map(|num| Exp::Dice(num));
    let (input, _) = alt((char('D'), char('d')))(text)?;
    map_opt(take_while(is_digit), str_to_dice)(input)
}

fn constant_parse(text: &[u8]) -> IResult<&[u8], Exp> {
    let str_to_constant = |s| str_to_u8(s).map(|v| Exp::Constant(v as i32));
    map_opt(take_while(is_digit), str_to_constant)(text)
}

fn multidice_parse(text: &[u8]) -> IResult<&[u8], Exp> {
    let (input, count) = map_opt(take_while(is_digit), str_to_u8)(text)?;
    let (input, dice) = dice_parse(input)?;

    Ok((
        input,
        Exp::MultiDice(MultiDice {
            count: count as i32,
            dice: Box::new(dice),
        }),
    ))
}

fn to_operation(text: &[u8]) -> Option<Operator> {
    let as_str = std::str::from_utf8(text).ok()?;
    match as_str {
        "+" => Some(Operator::Plus),
        "-" => Some(Operator::Minus),
        _ => None,
    }
}

fn operation_parser(text: &[u8]) -> IResult<&[u8], Operator> {
    let char_parser = alt((tag("+"), tag("-")));
    let all_parser = delimited(space0, char_parser, space0);
    map_opt(all_parser, to_operation)(text)
}

fn op_parser(text: &[u8]) -> IResult<&[u8], Exp> {
    let (input, (left, operator, right)) = tuple((element_parser, operation_parser, parser))(text)?;

    Ok((
        input,
        Exp::Op(Operation {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }),
    ))
}

fn element_parser(text: &[u8]) -> IResult<&[u8], Exp> {
    alt((multidice_parse, dice_parse, constant_parse))(text)
}

fn parser(text: &[u8]) -> IResult<&[u8], Exp> {
    alt((op_parser, element_parser))(text)
}

pub fn parse(text: &str) -> Option<Exp> {
    parser(text.as_bytes()).ok().map(|(_, dice)| dice)
}

#[test]
fn create_dice() {
    assert_eq!(parse("D20"), Some(Exp::Dice(20)));
    assert_eq!(parse("d20"), Some(Exp::Dice(20)));
    assert_eq!(parse("d4"), Some(Exp::Dice(4)));

    assert_eq!(parse("xfjhgdr"), None);
}

#[test]
fn create_multidice() {
    assert_eq!(
        parse("2D10"),
        Some(Exp::MultiDice(MultiDice {
            count: 2,
            dice: Box::new(Exp::Dice(10))
        }))
    );

    assert_eq!(
        parse("4d4"),
        Some(Exp::MultiDice(MultiDice {
            count: 4,
            dice: Box::new(Exp::Dice(4))
        }))
    );
}

#[test]
fn parse_add_op() {
    let expected = Some(Exp::Op(Operation {
        left: Box::new(Exp::Dice(6)),
        right: Box::new(Exp::Dice(4)),
        operator: Operator::Plus,
    }));

    assert_eq!(parse("d6+d4"), expected);
}

#[test]
fn parse_multipart_op() {
    let expected = Some(Exp::Op(Operation {
        left: Box::new(Exp::Dice(2)),
        operator: Operator::Plus,
        right: Box::new(Exp::Op(Operation {
            left: Box::new(Exp::Dice(4)),
            operator: Operator::Plus,
            right: Box::new(Exp::Dice(6)),
        })),
    }));

    assert_eq!(parse("d2+d4+d6"), expected);
}

#[test]
fn parse_add_op_with_space() {
    let expected = Some(Exp::Op(Operation {
        left: Box::new(Exp::Dice(2)),
        operator: Operator::Plus,
        right: Box::new(Exp::Dice(2)),
    }));

    assert_eq!(parse("d2 + d2"), expected);
}

#[test]
fn parse_minus_op() {
    let expected = Some(Exp::Op(Operation {
        left: Box::new(Exp::Dice(20)),
        operator: Operator::Minus,
        right: Box::new(Exp::Dice(4)),
    }));
    assert_eq!(parse("d20 - d4"), expected);
}

#[test]
fn parse_op_and_multidice() {
    let expected = Some(Exp::Op(Operation {
        left: Box::new(Exp::MultiDice(MultiDice {
            count: 2,
            dice: Box::new(Exp::Dice(20)),
        })),
        operator: Operator::Plus,
        right: Box::new(Exp::Dice(4)),
    }));

    assert_eq!(parse("2D20 + d4"), expected);
}

#[test]
fn parse_constant() {
    assert_eq!(parse("10"), Some(Exp::Constant(10)));

    let expected = Some(Exp::Op(Operation {
        left: Box::new(Exp::Dice(20)),
        operator: Operator::Plus,
        right: Box::new(Exp::Constant(4)),
    }));
    assert_eq!(parse("d20 + 4"), expected);
}
