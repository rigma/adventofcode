use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
pub struct Set {
    pub red: Option<u32>,
    pub green: Option<u32>,
    pub blue: Option<u32>,
}

impl FromStr for Set {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = None;
        let mut green = None;
        let mut blue = None;

        s.split(", ").for_each(|part| {
            let (quantity, kind) = part.split_once(' ').expect("invalid format");
            let quantity = quantity.parse().expect("valid integer");

            match kind {
                "red" => red = Some(quantity),
                "green" => green = Some(quantity),
                "blue" => blue = Some(quantity),
                _ => panic!("unexpected kind"),
            }
        });

        Ok(Self { red, green, blue })
    }
}

pub fn parse_games(input: &str) -> HashMap<u32, Vec<Set>> {
    let mut games = HashMap::new();

    input.lines().for_each(|line| {
        let (id, sets) = line.split_once(": ").expect("invalid game line");
        let id = id
            .strip_prefix("Game ")
            .expect("invalid game ID")
            .parse()
            .expect("invalid ID");
        let sets = sets
            .split("; ")
            .map(|raw| raw.parse::<Set>().unwrap())
            .collect();

        games.insert(id, sets);
    });

    games
}
