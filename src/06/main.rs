#![feature(alloc_system)]
extern crate alloc_system;
extern crate arrayvec;
extern crate time;

use arrayvec::ArrayVec;

/// Contains the character as a `key` and it's frequency as the `value`
struct Frequency { key: char, value: u8 }

/// A map of character frequencies
struct FrequencyMap { data: ArrayVec<[Frequency; 26]> }

impl FrequencyMap {
    /// Increment a given character in the map.
    /// The character's index is guaranteed to be the character's integer representation minus 97.
    fn increment_key(&mut self, key: char) { self.data[(key as u8 - 97) as usize].value += 1; }

    /// Collect the most frequent character in the map.
    fn most_frequent(&self) -> char {
        self.data.iter().fold(('a', 0), |acc, x| if x.value > acc.1 { (x.key, x.value) } else { acc }).0
    }

    /// Collect the least frequent character in the map.
    fn least_frequent(&self) -> char {
        self.data.iter().fold(('a', 255), |acc, x| if x.value < acc.1 { (x.key, x.value) } else { acc }).0
    }

    // Reset the values on the map
    fn reset(&mut self) { for element in &mut self.data { element.value = 0; }}
}

fn get_message(unmodified: &mut [char; 8], modified: &mut [char; 8], inputs: &str) {
    let mut frequency = FrequencyMap {
        data: (b'a'..b'z' + 1).map(|c| Frequency { key: c as char, value: 0 }).collect::<ArrayVec<[_; 26]>>()
    };

    for index in 0..8 {
        for message in inputs.lines() {
            if let Some(character) = message.chars().nth(index) { frequency.increment_key(character); }
        }
        unmodified[index] = frequency.most_frequent();
        modified[index]   = frequency.least_frequent();
        frequency.reset();
    }
}

fn main() {
    let inputs = include_str!("input.txt");

    let begin = time::precise_time_ns();
    let mut unmodified_message = ['\0'; 8];
    let mut modified_message   = ['\0'; 8];
    get_message(&mut unmodified_message, &mut modified_message, inputs);
    let end = time::precise_time_ns();

    println!("The unmodified message is {}.\nThe modified message is {}.\n",
        unmodified_message.iter().cloned().collect::<String>(),
        modified_message.iter().cloned().collect::<String>());

    println!("Day 06 Execution Time: {} milliseconds", (end - begin) as f64 / 1_000_000f64);

}

#[test]
fn part_one() {
    let inputs = r#"eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar"#;

    let mut unmodified_message = ['\0'; 8];
    let mut modified_message = ['\0'; 8];
    get_message(&mut unmodified_message, &mut modified_message, inputs);
    for (actual, expected) in unmodified_message.iter().zip(['e', 'a', 's', 't', 'e', 'r'].iter()) {
        assert_eq!(actual, expected);
    }
}
