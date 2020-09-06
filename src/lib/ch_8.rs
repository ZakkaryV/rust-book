pub mod calculate {
    use crate::util::selection_sort_int;
    use std::collections::HashMap;

    pub fn mean(list: &[i32]) -> f32 {
        list.iter().fold(0, |acc, i| acc + i) as f32 / (list.len() as f32)
    }

    pub fn median(list: &[i32]) -> f32 {
        let unsorted_arr = &mut list.to_owned();
        let sorted = selection_sort_int(unsorted_arr);
        let mid_i = sorted.len() / 2;

        if sorted.len() % 2 == 0 {
            let mid_sums: i32 = sorted[(mid_i - 1)..(mid_i + 1)].iter().sum();

            (mid_sums / 2) as f32
        } else {
            sorted[mid_i] as f32
        }
    }

    pub fn mode(list: &[i32]) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for &v in list.iter() {
            let k = map.entry(v).or_insert(0);
            *k += 1;
        }

        map.into_iter()
            .fold((0, 0), |acc, val| if val.1 > acc.1 { val } else { acc })
            .0
    }
}

pub mod pig_latin {
    use std::collections::HashSet;

    enum CharacterType {
        Consonant,
        Vowel,
        NonLetter,
    }

    fn move_punc_to_end(s: String) -> String {
        let punc: HashSet<char> = ['!', '?', '.'].iter().cloned().collect();
        let mut corrected: Vec<char> = Vec::new();
        let mut deferred_punc: Vec<char> = Vec::new();

        for c in s.chars() {
            if let Some(_p) = punc.get(&c) {
                deferred_punc.push(c)
            } else {
                corrected.push(c);
            }
        }

        corrected.append(&mut deferred_punc);
        String::from(corrected.into_iter().collect::<String>())
    }

    fn character_type(c: &char) -> CharacterType {
        let vowels: HashSet<char> = ['a', 'e', 'i', 'o', 'u'].iter().cloned().collect();

        if !c.is_ascii_punctuation() {
            if let Some(l) = vowels.get(&c.to_ascii_lowercase()) {
                CharacterType::Vowel
            } else {
                CharacterType::Consonant
            }
        } else {
            CharacterType::NonLetter
        }
    }

    pub fn convert(s: &str) -> String {
        let mut pl_s: Vec<String> = Vec::new();
        let mut pl_w: Vec<char> = Vec::new();
        let mut non_chars: Vec<char> = Vec::new();
        let mut consonants: Vec<char> = Vec::new();

        for w in s.split(" ") {
            pl_w.clear();
            non_chars.clear();
            consonants.clear();

            let word: String;

            for (i, c) in w.chars().enumerate() {
                match character_type(&c) {
                    CharacterType::Vowel => {
                        if i == 0 {
                            word = format!("{}-way", w);
                        } else {
                            word =
                                format!("{}-{}ay", &w[i..], consonants.iter().collect::<String>());
                        }

                        pl_w.append(&mut word.chars().collect());
                        break;
                    }
                    CharacterType::Consonant => {
                        consonants.push(c);
                    }
                    CharacterType::NonLetter => {
                        if i == 0 {
                            pl_w.append(&mut w.chars().collect());
                            break;
                        }
                    }
                }
            }

            pl_s.push(move_punc_to_end(pl_w.to_owned().into_iter().collect()));
        }

        pl_s.join(" ") //.trim();
    }
}

#[cfg(test)]
mod math {
    use super::calculate::{mean, median, mode};

    #[test]
    fn calculate_mean() {
        let test_data = vec![2, 2, 3, 3, 3, 3, 4, 5, 6, 7, 7, 7, 8, 8, 9, 10];

        assert_eq!(mean(&test_data), 5.4375);
    }

    #[test]
    fn calculate_median() {
        let odd_list = vec![2, 2, 52, 7, 7, 6, 4, 9, 105, 8, 8, 3, 3, 3, 3];

        assert_eq!(
            median(&odd_list),
            6 as f32,
            "Median is calculated with odd number list"
        );

        let even_list = vec![2, 2, 2, 2, 2, 2, 7, 7, 7, 6, 4, 9, 105, 8, 8, 3, 3, 236];
        assert_eq!(
            median(&even_list),
            5 as f32,
            "Median is calculated with even number list"
        );
    }

    #[test]
    fn calculate_mode() {
        let test_data = vec![2, 2, 52, 7, 7, 6, 4, 9, 105, 8, 8, 3, 3, 3, 3];

        assert_eq!(
            mode(&test_data),
            3,
            "Mode is returned as most commonly recurring value"
        );
    }
}

#[cfg(test)]
mod pig_latin_converter {
    use super::pig_latin;

    #[test]
    fn converts_to_pig_latin() {
        let english_str = "Pig Latin is used in schools to teach language constructs.";

        assert_eq!(
            pig_latin::convert(&english_str),
            "ig-Pay atin-Lay is-way used-way in-way ools-schay o-tay each-tay anguage-lay onstructs-cay."
        );
    }
}
