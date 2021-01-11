use rand::Rng;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Dice {
    D2,
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
    D100,
}

impl fmt::Display for Dice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Dice::D2 => "D2",
            Dice::D4 => "D4",
            Dice::D6 => "D6",
            Dice::D8 => "D8",
            Dice::D10 => "D10",
            Dice::D12 => "D12",
            Dice::D20 => "D20",
            Dice::D100 => "D100",
        };
        write!(f, "{}", text)
    }
}

impl Dice {
    pub fn roll<T: Rng>(&self, rnd: &mut T) -> i32 {
        rnd.gen_range(self.min()..self.max() + 1)
    }

    pub fn max(&self) -> i32 {
        match &self {
            Dice::D2 => 2,
            Dice::D4 => 4,
            Dice::D6 => 6,
            Dice::D8 => 8,
            Dice::D10 => 10,
            Dice::D12 => 12,
            Dice::D20 => 20,
            Dice::D100 => 100,
        }
    }

    pub fn min(&self) -> i32 {
        1
    }
}
