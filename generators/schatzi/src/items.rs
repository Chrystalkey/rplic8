use std::iter::Iterator;

use crate::money::Coins;

type Gram = u32;

pub struct LootItemRecord {
    handle: String,
    name: String,
    max_pp: u8,
}

pub struct LootStash {
    all_items: Vec<LootItemRecord>,
}
impl LootStash {
    fn new(db_path: &str) -> Self {
        todo!("Load items from Database")
    }
}

pub struct PersonLootItems(Vec<(LootItemRecord, u32)>);

impl Iterator for LootStash {
    type Item = PersonLootItems;
    fn next(&mut self) -> Option<PersonLootItems> {
        todo!()
    }
}
