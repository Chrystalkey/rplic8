// follows the convention:
pub struct LootItem{
    variant: LootItem,
    name: &'static str,
}


/// contains the max value found on any one person
pub enum LootItemType {
    Knife,
    Amulet,
    Godfigurine,
    Ring,
    Gemstone,
    Necklace,
    Belt,
    Rope,
    Tankard,
    Cup,
    Book,
    Coalpencil,
    Chalk,
    PotionHealing,
    Poison,
}
