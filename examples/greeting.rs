use std::{fmt::Display, str::FromStr};
use withargs::withargs;

#[withargs]
fn main(greeting: Greeting, name: String) {
    println!("{}, {}!", greeting, name);
}

enum Greeting {
    Hey,
    Hi,
    Howdy,
}

impl FromStr for Greeting {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Greeting::*;

        match s {
            "hey" => Ok(Hey),
            "hi" => Ok(Hi),
            "howdy" => Ok(Howdy),
            _ => Err("that is not a greeting"),
        }
    }
}

impl Display for Greeting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Greeting::*;

        write!(
            f,
            "{}",
            match self {
                Hey => "hey",
                Hi => "hi",
                Howdy => "howdy",
            }
        )
    }
}
