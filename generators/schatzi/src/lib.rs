use std::fmt::{Debug, Display};

use crate::{
    items::{LootStash, PersonLootItems},
    money::Coins,
};

mod items;
mod money;

enum Stratum {
    Rich,
    Normal,
    Poor,
    Beggar,
}

struct PersonConfig {
    stratum: Stratum,
}

struct Schatzi {
    loot_stash: LootStash,
    person_config: PersonConfig,
}

impl Schatzi {
    fn new() -> Self {
        todo!()
    }
}

impl generate::RPGenerator for Schatzi {
    type Seed = u64;
    fn seed(&mut self, s: Self::Seed) {}
}

struct PersonLoot {
    items: PersonLootItems,
    coins: Coins,
    // todo: clothing
}
impl Display for PersonLoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Iterator for Schatzi {
    type Item = PersonLoot;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
