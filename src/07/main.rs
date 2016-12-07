#![feature(alloc_system)]
extern crate alloc_system;
extern crate arrayvec;
extern crate time;
use arrayvec::ArrayVec;

/// Designates a string slice as either being an `Inner` or `Outer` token.
enum IPToken<'a> {
    Inner(&'a str),
    Outer(&'a str),
}

/// Given an `IPv7` address as input, it will separate the `Inner` tokens from the `Outer` tokens.
struct IPTokenizer<'a> {
    line:    &'a str,
    read:    usize,
    bracket: bool
}

impl<'a> IPTokenizer<'a> {
    fn new(input: &'a str) -> IPTokenizer<'a> {
        IPTokenizer { line: input, read: 0, bracket: false }
    }
}

impl<'a> Iterator for IPTokenizer<'a> {
    type Item = IPToken<'a>;
    fn next(&mut self) -> Option<IPToken<'a>> {
        if self.bracket {
            for (mut id, character) in self.line.chars().skip(self.read).enumerate() {
                if character == ']' {
                    id += self.read;
                    let token = &self.line[self.read..id];
                    self.read = id+1;
                    self.bracket = false;
                    return Some(IPToken::Inner(token));
                }
            }
        } else {
            for (mut id, character) in self.line.chars().skip(self.read).enumerate() {
                if character == '[' {
                    id += self.read;
                    let token = &self.line[self.read..id];
                    self.read = id+1;
                    self.bracket = true;
                    return Some(IPToken::Outer(token));
                }
            }
        }
        if self.read == self.line.len() {
            None
        } else {
            let token = &self.line[self.read..];
            self.read = self.line.len();
            Some(IPToken::Outer(token))
        }
    }
}

/// Check if a string slice contains an **ABBA** pattern
fn contains_abba(input: &str) -> bool {
    let mut pattern = ['\0'; 4];
    let mut chars = input.chars();

    if let (Some(a), Some(b), Some(c), Some(d)) = (chars.next(), chars.next(), chars.next(), chars.next()) {
        pattern[0] = a;
        pattern[1] = b;
        pattern[2] = c;
        pattern[3] = d;
        if pattern[0] == pattern[3] && pattern[1] == pattern[2] && pattern[0] != pattern[1] {
            return true
        }

        for character in chars {
            pattern[0] = pattern[1];
            pattern[1] = pattern[2];
            pattern[2] = pattern[3];
            pattern[3] = character;
            if pattern[0] == pattern[3] && pattern[1] == pattern[2] && pattern[0] != pattern[1] {
                return true
            }
        }
    }
    false
}

/// Meant to be used with `Outer` tokens, it will return all possible **ABA** values for the given token.
struct ABATokenizer<'a> {
    data:    &'a str,
    pattern: ArrayVec<[char; 3]>,
    read:    usize
}

impl<'a> ABATokenizer<'a> {
    fn new(input: &'a str) -> ABATokenizer<'a> {
        ABATokenizer { data: input, read: 0, pattern: ArrayVec::<[char; 3]>::new() }
    }
}

impl<'a> Iterator for ABATokenizer<'a> {
    type Item = ArrayVec<[char; 3]>;
    fn next(&mut self) -> Option<ArrayVec<[char; 3]>> {
        let mut char_iter = self.data.chars().skip(self.read);
        if self.read == 0 {
            if let (Some(a), Some(b), Some(c)) = (char_iter.next(), char_iter.next(), char_iter.next()) {
                self.pattern.push(a);
                self.pattern.push(b);
                self.pattern.push(c);
                self.read += 3;
                if self.pattern[0] == self.pattern[2] && self.pattern[0] != self.pattern[1] {
                    return Some(self.pattern.clone())
                }
            }
        }

        for character in char_iter {
            self.read += 1;
            self.pattern[0] = self.pattern[1];
            self.pattern[1] = self.pattern[2];
            self.pattern[2] = character;
            if self.pattern[0] == self.pattern[2] && self.pattern[0] != self.pattern[1] { return Some(self.pattern.clone()) }
        }

        None
    }
}

/// Meant to be used with `Inner` tokens, this will check to see if an `Inner` token contains
/// the **BAB** version of an `Outer` token's **ABA**.
fn contains_bab(input: &str, aba: &ArrayVec<[char; 3]>) -> bool {
    let mut pattern = ['\0'; 3];
    let mut char_iter = input.chars();
    if let (Some(a), Some(b), Some(c)) = (char_iter.next(), char_iter.next(), char_iter.next()) {
        pattern[0] = a;
        pattern[1] = b;
        pattern[2] = c;
        if pattern[0] == pattern[2] && pattern[0] != pattern[1] && pattern[0] == aba[1] && pattern[1] == aba[0] {
            return true
        } else {
            for character in char_iter {
                pattern[0] = pattern[1];
                pattern[1] = pattern[2];
                pattern[2] = character;
                if pattern[0] == pattern[2] && pattern[0] != pattern[1] && pattern[0] == aba[1] && pattern[1] == aba[0] {
                    return true
                }
            }
        }
    }
    false
}

/// Checks an `IPv7` address's `Inner` and `Outer` tokens to determine if it supports TLS.
/// TLS support means that the `Inner` tokens do not contain any **ABBA** patterns,
/// but that the `Outer` tokens do contain at least one **ABBA** pattern.
fn supports_tls(inner: &[&str], outer: &[&str]) -> bool {
    if !inner.iter().any(|x| contains_abba(x)) {
        outer.iter().any(|x| contains_abba(x))
    } else {
        false
    }
}

/// Checks an `IPv7` address's `Inner` and `Outer` tokens to determine if it supports SSL.
/// SSL support means that there is at least one **BAB** match in an `Inner` token
/// that is a reverse of an `Outer` token's **ABA** match.
fn supports_ssl(inner: &[&str], outer: &[&str]) -> bool {
    for aba in outer.iter().flat_map(|x| ABATokenizer::new(x)) {
        if inner.iter().any(|token| contains_bab(token, &aba) ) { return true }
    }
    false
}


fn main() {
    let inputs = include_str!("input.txt");
    let begin = time::precise_time_ns();
    let (mut tls_supported, mut ssl_supported) = (0, 0);
    for line in inputs.lines() {
        let mut outer: ArrayVec<[&str; 5]> = ArrayVec::new();
        let mut inner: ArrayVec<[&str; 3]> = ArrayVec::new();
        for token in IPTokenizer::new(line) {
            match token {
                IPToken::Inner(content) => { inner.push(content); },
                IPToken::Outer(content) => { outer.push(content); },
            }
        }
        if supports_tls(&inner, &outer) { tls_supported += 1; }
        if supports_ssl(&inner, &outer) { ssl_supported += 1; }
    }
    let end = time::precise_time_ns();
    println!("{} IPs support TLS.\n{} IPs support SSL.", tls_supported, ssl_supported);
    println!("Day 07 Execution Time: {} milliseconds", (end - begin) as f64 / 1_000_000f64)
}

#[test]
fn part_one() {
    let inputs = ["abba[mnop]qrst", "abcd[bddb]xyyx", "aaaa[qwer]tyui", "ioxxoj[asdfgh]zxcvbn"];
    let expected = [true, false, false, true];
    let mut count = 0;
    for line in inputs.iter() {
        let mut outer: ArrayVec<[&str; 5]> = ArrayVec::new();
        let mut inner: ArrayVec<[&str; 3]> = ArrayVec::new();
        for token in IPTokenizer::new(line) {
            match token {
                IPToken::Inner(content) => { inner.push(content); },
                IPToken::Outer(content) => { outer.push(content); },
            }
        }
        println!("#{}", count);
        assert_eq!(expected[count], supports_tls(&inner, &outer));
        count += 1;
    }
}

#[test]
fn part_two() {
    let inputs = ["aba[bab]xyz", "xyx[xyx]xyx", "aaa[kek]eke", "zazbz[bzb]cdb, zazbz[acdc]dfas[fsf]adcd"];
    let expected = [true, false, true, true, true];
    let mut count = 0;
    for line in inputs.iter() {
        let mut outer: ArrayVec<[&str; 5]> = ArrayVec::new();
        let mut inner: ArrayVec<[&str; 3]> = ArrayVec::new();
        for token in IPTokenizer::new(line) {
            match token {
                IPToken::Inner(content) => { inner.push(content); },
                IPToken::Outer(content) => { outer.push(content); },
            }
        }
        assert_eq!(expected[count], supports_ssl(&inner, &outer));
        count += 1;
    }
}
