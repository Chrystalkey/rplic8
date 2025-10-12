use rand::{rngs::StdRng, SeedableRng};

const CHARACTERS: &str = "\0\x01abcdefghijklmnopqrstuvwxyzäöüß-"; // \0 is the "ending" character, \1 is the starting character
const CHAR_COUNT: usize = 33;

struct MarkovModel<const RETAIN: usize> {
    probabilities: [[f32; CHAR_COUNT]; CHAR_COUNT], // "None", abcdefg... see above
    last_n : [char;RETAIN],                                 // array of the last used characters of size RETAIN
    last_idx: usize,
    rng: StdRng,
}

fn index(c: char) -> usize {
    match c {
        '\0' => 0,
        '\x01' => 1,
        'a'..='z' => (c as u32 - 0x61) as usize + 2,
        'ä' => CHAR_COUNT - 5,
        'ö' => CHAR_COUNT - 4,
        'ü' => CHAR_COUNT - 3,
        'ß' => CHAR_COUNT - 2,
        '-' => CHAR_COUNT - 1,
        _ => panic!("Illegal Character found: `{}` ({})", c, c.escape_debug()),
    }
}

impl<const RETAIN: usize> MarkovModel<RETAIN> {
    fn new(input: &str) -> Self {
        assert!(RETAIN > 0, "Must retain more than 0");
        assert!(RETAIN < 4, "T = (m*RETAIN)x(m*RETAIN), please don't make your memory explode");

        let mut occ = [[0usize; CHAR_COUNT]; CHAR_COUNT];
        occ[index('\0')][index('\0')] = 1;
        for line in input.split_whitespace() {
            let mut previous_char = None;
            for c in line.chars() {
                match previous_char {
                    None => occ[index('\x01')][index(c)] += 1,       // this is a character that a name started with
                    Some(pc) => occ[index(pc)][index(c)] += 1,
                }
                previous_char = Some(c);
            }
            // the word is done, and the last char is followed by a terminator
            occ[index(previous_char.unwrap())][index('\0')] += 1;
        }

        let mut probabilities: [[f32;CHAR_COUNT];CHAR_COUNT] = occ
            .iter()
            .map(|&array| {
                let sum = array.into_iter().fold(0, std::ops::Add::add);
                match sum{
                    0 => array.iter().map(|_| 1./CHAR_COUNT as f32).collect::<Vec<f32>>().try_into().unwrap(), // with 0 it is equally distributed
                    _ => array
                    .iter()
                    .map(|&char_occ| char_occ as f32 / sum as f32)
                    .collect::<Vec<f32>>()
                    .try_into().unwrap()
                }
            })
            .collect::<Vec<_>>()
            .try_into().unwrap();
        probabilities[index('\0')][index('\0')] = 1.;
        // println!("");
        // for c in CHARACTERS.chars() {
        //     println!("{}, `{c}` {:.2?}",index(c), probabilities[index(c)]);
        // }
        Self {
            probabilities,
            last_n: ['\x01';RETAIN], // the last used character was the start meta-char
            last_idx: 0,
            rng: StdRng::from_os_rng(),
        }
    }
}

impl<const RETAIN: usize> Iterator for MarkovModel<RETAIN> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        let last_used_character = self.last_n[self.last_idx];
        self.last_idx = (self.last_idx+1) % RETAIN;

        let mut idx = index('\0');
        while idx == index('\0') {
            idx = rand::seq::index::sample_weighted(&mut self.rng, CHAR_COUNT, |i| self.probabilities[index(last_used_character)][i], 1)
            .unwrap().index(0);
        }
        let result = CHARACTERS.chars().nth(idx);

        self.last_n[self.last_idx] = result.unwrap();
        return result;
    }
}

#[cfg(test)]
mod test{
    use crate::markov::MarkovModel;
    use super::{index};

    #[test]
    fn index_function(){
        for (i, c) in super::CHARACTERS.chars().enumerate() {
            assert_eq!(i, index(c), "character `{c}` was at index {i} but index(c) = {}", index(c));
        }
    }

    #[test]
    fn assert_stochastic_mat(){
        let mkv: MarkovModel<1> = MarkovModel::new(include_str!("../names.txt"));
        for (i,l) in mkv.probabilities.iter().enumerate(){
            let sum = l.into_iter().fold(0., |x,y| x+y);
            println!("Error: {}", sum-1.);
            assert!(sum - 1. < f32::EPSILON*10.,
                "Failed in row {}: {:.2?} with a row sum of {}", i, l, sum
            );
        }
    }

    #[test]
    fn generate_10_ret1(){
        let mut mkv: MarkovModel<1> = MarkovModel::new(include_str!("../names.txt"));
        let mut s = String::new();
        for _ in 0..8 {
            s.push_str(&mkv.next().unwrap().to_string());
        }
        println!("Markov Result: `{s}`");
    }
}