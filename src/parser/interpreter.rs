use super::ast::{Exp, Operator};

#[derive(Debug, PartialEq)]
pub enum Token {
    Dice(i32),
    Constant(i32),
    Op(Operator),
}

pub fn tree_to_rpn(exp: &Exp) -> Vec<Token> {
    // Results always start positive
    let mut result = vec![Token::Op(Operator::Plus)];
    add_to_tokens(exp, &mut result);
    result
}

fn add_to_tokens(exp: &Exp, result: &mut Vec<Token>) {
    match exp {
        Exp::Constant(c) => result.push(Token::Constant(*c)),
        Exp::Dice(d) => result.push(Token::Dice(*d as i32)),
        Exp::MultiDice(md) => {
            for _ in 0..(md.count) {
                add_to_tokens(md.dice.as_ref(), result);
            }
        }
        Exp::Op(o) => {
            add_to_tokens(o.left.as_ref(), result);
            result.push(Token::Op(o.operator));
            add_to_tokens(o.right.as_ref(), result);
        }
    }
}

#[test]
fn constant_is_positive() {
    let result = tree_to_rpn(&Exp::Constant(6));
    assert_eq!(result, vec![Token::Op(Operator::Plus), Token::Constant(6)]);
}

#[test]
fn dice_become_tokens() {
    let result = tree_to_rpn(&Exp::Dice(6));
    assert_eq!(result, vec![Token::Op(Operator::Plus), Token::Dice(6)]);
}

#[test]
fn multidice_become_many_dice() {
    let result = tree_to_rpn(&Exp::MultiDice(super::ast::MultiDice {
        count: 2,
        dice: Box::new(Exp::Dice(6)),
    }));
    assert_eq!(
        result,
        vec![Token::Op(Operator::Plus), Token::Dice(6), Token::Dice(6)]
    );
}

#[test]
fn operators_are_added_to_the_queue() {
    let result = tree_to_rpn(&Exp::Op(super::ast::Operation {
        left: Box::new(Exp::Constant(3)),
        operator: Operator::Minus,
        right: Box::new(Exp::Constant(4)),
    }));

    assert_eq!(
        result,
        vec![
            Token::Op(Operator::Plus),
            Token::Constant(3),
            Token::Op(Operator::Minus),
            Token::Constant(4)
        ]
    );
}
