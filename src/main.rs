use std::fmt::{Display, Debug};
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashMap;


#[derive(Clone, PartialEq, Eq)]
struct SevenSegmentsDigit {
    a: bool,
    b: bool,
    c: bool,
    d: bool,
    e: bool,
    f: bool,
    g: bool
}

impl SevenSegmentsDigit {

    const Zero:     SevenSegmentsDigit = SevenSegmentsDigit{a:true, b: true, c: true, d: false, e: true, f: true, g: true};
    const One:      SevenSegmentsDigit = SevenSegmentsDigit{a:false, b: false, c: true, d: false, e: false, f: true, g: false};
    const Two:      SevenSegmentsDigit = SevenSegmentsDigit{a:true, b: false, c: true, d: true, e: true, f: false, g: true};
    const Three:    SevenSegmentsDigit = SevenSegmentsDigit{a:true, b: false, c: true, d: true, e: false, f: true, g: true};
    const Four:     SevenSegmentsDigit = SevenSegmentsDigit{a:false, b: true, c: true, d: true, e: false, f: true, g: false};
    const Five:     SevenSegmentsDigit = SevenSegmentsDigit{a:true, b: true, c: false, d: true, e: false, f: true, g: true};
    const Six:      SevenSegmentsDigit = SevenSegmentsDigit{a:true, b: true, c: false, d: true, e: true, f: true, g: true};
    const Seven:    SevenSegmentsDigit = SevenSegmentsDigit{a:true, b: false, c: true, d: false, e: false, f: true, g: false};
    const Eight:    SevenSegmentsDigit = SevenSegmentsDigit{a:true, b: true, c: true, d: true, e: true, f: true, g: true};
    const Nine:     SevenSegmentsDigit = SevenSegmentsDigit{a:true, b: true, c: true, d: true, e: false, f: true, g: true};

}

impl Display for SevenSegmentsDigit {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        let mut res = String::new();

        if self.a {
            res.push('a');
        }
        if self.b {
            res.push('b');
        }
        if self.c {
            res.push('c');
        }
        if self.d {
            res.push('d');
        }
        if self.e {
            res.push('e');
        }
        if self.f {
            res.push('f');
        }
        if self.g {
            res.push('g');
        }

        write!(f, "\"{}\"", res)
    }
}


impl Debug for SevenSegmentsDigit {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        let mut res = String::new();

        if self.a {
            res.push('a');
        }
        if self.b {
            res.push('b');
        }
        if self.c {
            res.push('c');
        }
        if self.d {
            res.push('d');
        }
        if self.e {
            res.push('e');
        }
        if self.f {
            res.push('f');
        }
        if self.g {
            res.push('g');
        }

        write!(f, "\"{}\"", res)
    }
}

impl TryFrom<u8> for SevenSegmentsDigit {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Zero),
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            3 => Ok(Self::Three),
            4 => Ok(Self::Four),
            5 => Ok(Self::Five),
            6 => Ok(Self::Six),
            7 => Ok(Self::Seven),
            8 => Ok(Self::Eight),
            9 => Ok(Self::Nine),
            _ => Err("Value must be in between 0-9")
        }
    }
}

impl TryInto<u8> for SevenSegmentsDigit {
    type Error = &'static str;

    fn try_into(self) -> Result<u8, Self::Error> {

        match self {
            SevenSegmentsDigit::Zero => Ok(0),
            SevenSegmentsDigit::One => Ok(1),
            SevenSegmentsDigit::Two => Ok(2),
            SevenSegmentsDigit::Three => Ok(3),
            SevenSegmentsDigit::Four => Ok(4),
            SevenSegmentsDigit::Five => Ok(5),
            SevenSegmentsDigit::Six => Ok(6),
            SevenSegmentsDigit::Seven => Ok(7),
            SevenSegmentsDigit::Eight => Ok(8),
            SevenSegmentsDigit::Nine => Ok(9),
            _ => Err("Seven segments does not display a digit")
            
        }
    }    
}


struct SeveSegmentsDigitDecoder {

    decode_map: HashMap<char, char>,
    encode_map: HashMap<char, char>
}

impl SeveSegmentsDigitDecoder {

    fn new() -> Self {
        SeveSegmentsDigitDecoder { decode_map: HashMap::new(), encode_map: HashMap::new() }
    }

    fn crack(&mut self, mut encoded_digits: Vec<String>) {

        //println!("encoded digits {:?}", encoded_digits);

        let i = encoded_digits.iter().position(|s| s.len() == 2).unwrap();
        let encoded_one = encoded_digits[i].clone();
        encoded_digits.remove(i);

        let i = encoded_digits.iter().position(|s| s.len() == 4).unwrap();
        let encoded_four = encoded_digits[i].clone();
        encoded_digits.remove(i);

        let i = encoded_digits.iter().position(|s| s.len() == 3).unwrap();
        let encoded_seven = encoded_digits[i].clone();
        encoded_digits.remove(i);

        let i = encoded_digits.iter().position(|s| s.len() == 7).unwrap();
        let encoded_eight = encoded_digits[i].clone();
        encoded_digits.remove(i);

        self.decode_map.insert(
            encoded_seven.chars().fold('a', |r, c| if encoded_one.contains(c) {r} else {c}), 
            'a'
        );

        self.encode_map.insert(
            'a',
            encoded_seven.chars().fold('a', |r, c| if encoded_one.contains(c) {r} else {c})
        );

        //println!("encoder: {:?}", self.encode_map);
        //println!("decoder: {:?}", self.decode_map);

        let six_segments: Vec<(usize, String)> = encoded_digits.iter().enumerate().filter_map(
            |s| if s.1.len() == 6 {Some((s.0,s.1.clone()))} else {None}).collect();

        let mut encoded_six = String::new();

        for c in encoded_one.chars() {
            for s in &six_segments {
                if !s.1.contains(c) {
                    encoded_six = s.1.clone();
                    encoded_digits.remove(s.0);
                    self.decode_map.insert(c, 'c');
                    self.encode_map.insert('c', c);
                    
                    break;
                }
            }
        }


        //println!("encoder: {:?}", self.encode_map);
        //println!("decoder: {:?}", self.decode_map);

        let f_key = encoded_seven.chars().find(|c| !self.decode_map.contains_key(c)).unwrap();

        self.decode_map.insert(f_key, 'f');
        self.encode_map.insert('f', f_key);


        //println!("encoder: {:?}", self.encode_map);
        //println!("decoder: {:?}", self.decode_map);


        for c in encoded_digits[0].chars() {
            if encoded_digits.iter().fold(true, |mut res, digit| {res &= digit.contains(c); return res})
            && !self.decode_map.contains_key(&c) {
                self.decode_map.insert(c, 'g');
                self.encode_map.insert('g', c);
            }
        }


        //println!("encoder: {:?}", self.encode_map);
        //println!("decoder: {:?}", self.decode_map);

        let mut encoded_three  = String::new();

        for digit in encoded_digits {
            if digit.len() == 5 && digit.contains(self.encode_map[&'c']) && digit.contains(self.encode_map[&'f']) {
                encoded_three = digit.clone();
            }
        }

        for c in encoded_three.chars() {
            if !self.decode_map.contains_key(&c) {
                self.decode_map.insert(c, 'd');
                self.encode_map.insert('d', c);
            }
        }
        

        //println!("encoder: {:?}", self.encode_map);
        //println!("decoder: {:?}", self.decode_map);


        for c in encoded_four.chars() {
            if !self.decode_map.contains_key(&c) {
                self.decode_map.insert(c, 'b');
                self.encode_map.insert('b', c);
            }
        }


        //println!("encoder: {:?}", self.encode_map);
        //println!("decoder: {:?}", self.decode_map);

        for c in encoded_six.chars() {
            if !self.decode_map.contains_key(&c) {
                self.decode_map.insert(c, 'e');
                self.encode_map.insert('e', c);
            }
        }


        //println!("encoder: {:?}", self.encode_map);
        //println!("decoder: {:?}", self.decode_map);

    }

    fn decode(&self, encoded_digit: &String) -> SevenSegmentsDigit {

        let decoded_s = encoded_digit.chars().map(|c| self.decode_map[&c]).collect::<String>();

        SevenSegmentsDigit { 
            a: decoded_s.contains('a'),
            b: decoded_s.contains('b'),
            c: decoded_s.contains('c'),
            d: decoded_s.contains('d'),
            e: decoded_s.contains('e'),
            f: decoded_s.contains('f'),
            g: decoded_s.contains('g') 
        }
    }
}

fn main() {
    let lines = BufReader::new(File::open("input.txt").unwrap()).lines();

    let mut res = 0;

    for line in lines.map(|l| l.unwrap()) {
        let mut first_split = line.split('|');
        let encoded_digits: Vec<String> = first_split.next().unwrap().split_whitespace().map(|d| d.to_string()).collect();
        let encoded_answers: Vec<String> = first_split.next().unwrap().split_whitespace().map(|d| d.to_string()).collect();

        let mut decoder = SeveSegmentsDigitDecoder::new();
        decoder.crack(encoded_digits);


        //println!("encoded answer: {:?}", encoded_answers);

        let decoded_digits = encoded_answers.iter().map(
            |encoded_answer| decoder.decode(encoded_answer)).collect::<Vec<SevenSegmentsDigit>>();

        //println!("decoded digits: {:?}", decoded_digits);

        let decoded_answers: Vec<u8> = decoded_digits.into_iter().map(
            |decoded_answer| decoded_answer.try_into().unwrap()).collect();

        //println!("decoded answers: {:?}", decoded_answers);
        
        decoded_answers.iter().rev().enumerate().for_each(|(i, digit)| res += 10u32.checked_pow(i as u32).unwrap() * (*digit as u32));

        //println!("res {}", res);

    }

    println!("answer part II {}", res);
}
