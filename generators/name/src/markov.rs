use std::collections::HashMap;

use rand::{Rng, SeedableRng, rngs::StdRng};

const ALLOWED_CHARACTERS: &str = "abcdefghijklmnopqrstuvwxyzäöüß-"; // \0 is the "ending" character, \1 is the starting character

struct MarkovModel<const NGRAM: usize> {
    ngrams: Vec<String>,
    probabilities: Vec<Vec<f32>>,
    last_ngram: String,
    first_char: bool,
    rng: StdRng,
}

impl MarkovModel<2> {
    fn new(input: &str) -> Self {
        let mut ngrams = vec![];
        let mut occurrences: Vec<Vec<u32>> = vec![];

        for line in input.split_whitespace() {
            let mut previous_bigram: Option<[char; 2]> = None;
            let mut citer = line.chars();
            let mut current_bigram = ['\0', citer.next().unwrap()];
            for c in citer {
                current_bigram[0] = current_bigram[1];
                current_bigram[1] = c;
                // println!("{:?}", current_bigram);

                match previous_bigram {
                    None => (),
                    Some(x) => {
                        let src_ngram_idx =
                            if let Some(ngram_idx) = ngrams.iter().position(|f| *f == x) {
                                ngram_idx
                            } else {
                                ngrams.push(x.clone());
                                occurrences.push(vec![]);
                                ngrams.len() - 1
                            };
                        let dst_ngram_idx = if let Some(ngram_idx) =
                            ngrams.iter().position(|f| *f == current_bigram)
                        {
                            ngram_idx
                        } else {
                            ngrams.push(x.clone());
                            occurrences.push(vec![]);
                            ngrams.len() - 1
                        };
                        if occurrences[src_ngram_idx].len() <= dst_ngram_idx {
                            occurrences[src_ngram_idx].resize(dst_ngram_idx + 1, 0);
                        }
                        occurrences[src_ngram_idx][dst_ngram_idx] += 1;
                    }
                }
                previous_bigram = Some(current_bigram);
            }
        }
        let probabilities: Vec<Vec<f32>> = occurrences
            .iter()
            .map(|array| {
                let sum = array.iter().fold(0, std::ops::Add::add);
                println!("sum: {}", sum);
                array.iter().map(|&el| el as f32 / sum as f32).collect()
            })
            .collect();

        // println!("");
        // for c in ngrams.iter() {
        //     println!("`{}{}`, {:.2?}", c[0], c[1], probabilities[ngrams.iter().position(|f| *f == *c).unwrap()]);
        // }
        Self {
            ngrams: ngrams.iter().map(|x| String::from_iter(x.iter())).collect(),
            probabilities,
            last_ngram: String::new(),
            first_char: true,
            rng: StdRng::from_os_rng(),
        }
    }
}

impl<const RETAIN: usize> Iterator for MarkovModel<RETAIN> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        if self.last_ngram.is_empty() {
            // initialize with a random bigram. the next iteration will look back on the chosen bigram and return a fitting one
            self.last_ngram = self.ngrams[self.rng.random_range(0..self.ngrams.len())].clone();
            println!("First Bigram: {}", self.last_ngram);

            return Some(self.last_ngram.chars().nth(0).unwrap());
        } else if self.first_char {
            self.first_char = false;
            return Some(self.last_ngram.chars().nth(1).unwrap());
        }

        let src_idx = self
            .ngrams
            .iter()
            .position(|f| *f == self.last_ngram)
            .unwrap();
        let idx = rand::seq::index::sample_weighted(
            &mut self.rng,
            self.probabilities[src_idx].len(),
            |i| self.probabilities[src_idx][i],
            1,
        )
        .unwrap()
        .index(0);

        println!(
            "Last Bigram: `{}`@{:3?}, next one: `{}`@{:3?}",
            self.last_ngram, src_idx, self.ngrams[idx], idx
        );
        self.last_ngram = self.ngrams[idx].clone();
        // self.last_n = format!["{}{}", self.ngrams[src_idx].chars().nth(1).unwrap(), self.ngrams[idx].chars().nth(1).unwrap()];

        return Some(self.ngrams[idx].chars().nth(1).unwrap());
    }
}

#[cfg(test)]
mod test {
    use crate::markov::MarkovModel;

    #[test]
    fn generate_10_ret1() {
        let mut mkv: MarkovModel<2> = MarkovModel::new(include_str!("../names.txt"));
        println!("markov bigrams: {}", mkv.ngrams.len());
        todo!();

        let mut s = String::new();
        for _ in 0..8 {
            s.push_str(&mkv.next().unwrap().to_string());
        }
        println!("Markov Result: `{s}`");
    }
}
