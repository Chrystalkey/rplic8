use std::fmt::{Debug, Display};

pub struct Money {
    kreuzer: u64,
}
impl Money {
    pub fn from_kreuzer(kreuzer: u32) -> Self {
        Self {
            kreuzer: kreuzer as u64,
        }
    }
    pub fn from_heller(heller: f32) -> Self {
        Self {
            kreuzer: (heller * 10.).floor() as u64,
        }
    }
    pub fn from_silber(silber: f32) -> Self {
        Self {
            kreuzer: (silber * 100.).floor() as u64,
        }
    }
    pub fn from_dukaten(dukaten: f32) -> Self {
        Self {
            kreuzer: (dukaten * 1000.).floor() as u64,
        }
    }
}
impl Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.kreuzer < 10 {
            write!(f, "{:1} K", self.kreuzer)
        } else if self.kreuzer < 100 {
            write!(f, "{:1} H {:1} K", self.kreuzer / 10, self.kreuzer % 10)
        } else if self.kreuzer < 1000 {
            write!(
                f,
                "{:1} S {:1} H {:1} K",
                self.kreuzer / 100,
                (self.kreuzer % 100) / 10,
                self.kreuzer % 10
            )
        } else {
            write!(
                f,
                "{:1} D {:1} S {:1} H {:1} K",
                self.kreuzer / 1000,
                (self.kreuzer % 1000) / 100,
                (self.kreuzer % 100) / 10,
                self.kreuzer % 10
            )
        }
    }
}
impl Debug for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

pub struct Coins {
    d: u16,
    s: u16,
    h: u16,
    k: u16,
}

impl Coins {
    pub fn new_random(amount: Money) -> Self {
        todo!("Random Coin distribution summing to <amount>")
    }
}

impl From<Coins> for Money {
    fn from(value: Coins) -> Self {
        return Self {
            kreuzer: value.d as u64 * 1000
                + value.s as u64 * 100
                + value.h as u64 * 10
                + value.k as u64,
        };
    }
}
