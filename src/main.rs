use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn caesar(word: &str, n: u8) -> String {
    let mut out = String::with_capacity(word.len());

    for ch in word.chars() {
        let ch = match ch {
            'á' => 'a',
            'ä' => 'a',
            'à' => 'a',
            'é' => 'e',
            'è' => 'e',
            'ë' => 'e',
            'ê' => 'e',
            'ç' => 'c',
            'ñ' => 'n',
            'ö' => 'o',
            'ô' => 'o',
            'ó' => 'o',
            'ò' => 'o',
            'í' => 'i',
            'ï' => 'i',
            'ì' => 'i',
            'î' => 'i',
            'ī' => 'i',
            'ü' => 'u',
            'û' | 'ù' | 'ú' | 'ū' => 'u',
            c => c,
        };
        if ch.is_alphabetic() {
            if ch as u8 > 255 - n {
                println!("{ch} {n}");
            }
            if ch as u8 + n < 'a' as u8 {
                panic!("{ch} {n}");
            }
            out.push((((ch as u8 + n - 'a' as u8) % 26) + 'a' as u8) as char);
        } else {
            out.push(ch)
        }
    }

    out
}

fn main() {
    let mut all_words = HashSet::new();
    let path = "data/lang-portuguese.txt";
    let file = File::open(path).unwrap();
    let br = BufReader::new(file);
    for line in br.lines() {
        let line = line.unwrap();
        all_words.insert(line.to_lowercase());
    }

    let file = File::open(path).unwrap();
    let br = BufReader::new(file);
    let mut longest = String::from("");
    for (n, word) in br.lines().enumerate() {
        let word = word.unwrap().to_ascii_lowercase();
        if longest.len() < word.len() {
            for i in 1..26 {
                let caesared = caesar(&word, i);
                if all_words.contains(&caesared) {
                    println!("#{n} {word} -> {caesared} (i = {i})");
                    longest = word.clone();
                }
            }
        }
    }
}
