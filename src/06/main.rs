extern crate arrayvec;
use arrayvec::ArrayVec;

/// Contains the character as a `key` and it's frequency as the `value`
struct Frequency { key: char, value: u8 }

/// A map of character frequencies
struct FrequencyMap { data: ArrayVec<[Frequency; 26]> }

impl FrequencyMap {
    /// If key exists, increment it, else add it to the map
    fn increment_key(&mut self, key: char) {
        for element in &mut self.data {
            if element.key == key {
                element.value += 1;
                return
            }
        }
    }

    // Collect the most and least characters in the map.
    fn most_and_least_frequent(&self) -> (char, char) {
        let (mut most_frequent, mut least_frequent) = (('a', 0), ('a', 255));
        for element in &self.data {
            if element.value > most_frequent.1 {
                most_frequent   = (element.key, element.value);
            }
            if element.value < least_frequent.1 {
                least_frequent = (element.key, element.value);
            }
        }
        (most_frequent.0, least_frequent.0)
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
        let (most_frequent, least_frequent) = frequency.most_and_least_frequent();
        unmodified[index] = most_frequent;
        modified[index]   = least_frequent;
        frequency.reset();
    }
}

fn main() {
    let inputs = include_str!("input.txt");
    let mut unmodified_message = ['\0'; 8];
    let mut modified_message   = ['\0'; 8];
    get_message(&mut unmodified_message, &mut modified_message, inputs);

    println!("The unmodified message is {}.\n The modified message is {}.\n",
        unmodified_message.iter().cloned().collect::<String>(),
        modified_message.iter().cloned().collect::<String>());
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
