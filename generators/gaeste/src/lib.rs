use generate::RPGenerator;

struct Gaestegenerator;

struct Gast {
    profession: Profession,
    specialty: Specialty,
    teachable_skills: Vec<Skills>
    name: Name,
    special_items: Vec<Items>,
}

struct Configuration{
    
}

impl Iterator for Gaestegenerator {
    type Item = Gast;
    fn next(&mut self) -> Option<Self::Item>{
        None
    }
}
impl RPGenerator for Gaestegenerator {
    type Seed = u64;
    fn seed(&mut self, _s: u64) {
        todo!()
    }
    fn new()
}