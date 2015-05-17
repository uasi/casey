extern crate regex;

use regex::{Captures, Regex};
use std::ascii::AsciiExt;

pub trait Casey {
    /// Returns the snake_case equivalent of this string.
    fn to_snakecase(&self) -> String;

    /// Returns the Upper_Snake_Case equivalent of this string.
    fn to_upper_snakecase(&self) -> String;

    /// Returns the SCREAMING_SNAKE_CASE equivalent of this string.
    fn to_screaming_snakecase(&self) -> String;

    /// Returns the camelCase equivalent of this string.
    fn to_camelcase(&self) -> String;

    /// Returns the UpperCamelCase equivalent of this string.
    fn to_upper_camelcase(&self) -> String;

    /// Returns the hyphen-case equivalent of this string.
    fn to_hyphencase(&self) -> String;

    /// Returns the Upper-Hyphen-Case equivalent of this string.
    fn to_upper_hyphencase(&self) -> String;

    /// Returns the SCREAMING-HYPHEN-CASE equivalent of this string.
    fn to_screaming_hyphencase(&self) -> String;
}

impl Casey for str {
    fn to_snakecase(&self) -> String {
        let re = Regex::new(r"([A-Z\d+]+)([A-Z][a-z])").unwrap();
        let s = re.replace_all(self, "$1_$2");
        let re = Regex::new(r"([a-z\d])([A-Z])").unwrap();
        let s = re.replace_all(&s, "$1_$2");
        s.replace("-", "_").to_ascii_lowercase()
    }

    fn to_upper_snakecase(&self) -> String {
        let re = Regex::new(r"(^|_)([a-z])").unwrap();
        re.replace_all(&self.to_snakecase(), |cap: &Captures| {
            let c1 = cap.at(1).unwrap();
            let c2 = cap.at(2).unwrap();
            vec![c1, &c2.to_ascii_uppercase()].concat()
        })
    }

    fn to_screaming_snakecase(&self) -> String {
        (&self.to_snakecase()).to_ascii_uppercase()
    }

    fn to_camelcase(&self) -> String {
        let re = Regex::new(r"_([a-z])").unwrap();
        re.replace_all(&self.to_snakecase(), |cap: &Captures| {
            let c1 = cap.at(1).unwrap();
            c1.to_ascii_uppercase()
        })
    }

    fn to_upper_camelcase(&self) -> String {
        let re = Regex::new(r"(?:^|_)([a-z])").unwrap();
        re.replace_all(&self.to_snakecase(), |cap: &Captures| {
            let c1 = cap.at(1).unwrap();
            c1.to_ascii_uppercase()
        })
    }

    fn to_hyphencase(&self) -> String {
        self.to_snakecase().replace("_", "-")
    }

    fn to_upper_hyphencase(&self) -> String {
        self.to_upper_snakecase().replace("_", "-")
    }

    fn to_screaming_hyphencase(&self) -> String {
        self.to_screaming_snakecase().replace("_", "-")
    }
}

#[test]
fn test_to_snakecase() {
    assert_eq!(&"FooBARBaz-QUUX42".to_snakecase(), "foo_bar_baz_quux42");
}

#[test]
fn test_to_upper_snakecase() {
    assert_eq!(&"FooBARBaz-QUUX42".to_upper_snakecase(), "Foo_Bar_Baz_Quux42");
}

#[test]
fn test_to_screaming_snakecase() {
    assert_eq!(&"FooBARBaz-QUUX42".to_screaming_snakecase(), "FOO_BAR_BAZ_QUUX42");
}

#[test]
fn test_to_camelcase() {
    assert_eq!(&"foo_BarBaz-QUUX42".to_camelcase(), "fooBarBazQuux42");
}

#[test]
fn test_to_upper_camelcase() {
    assert_eq!(&"foo_BarBaz-QUUX42".to_upper_camelcase(), "FooBarBazQuux42");
}

#[test]
fn test_to_hyphencase() {
    assert_eq!(&"FooBARBaz-QUUX42".to_hyphencase(), "foo-bar-baz-quux42");
}

#[test]
fn test_to_upper_hyphencase() {
    assert_eq!(&"FooBARBaz-QUUX42".to_upper_hyphencase(), "Foo-Bar-Baz-Quux42");
}
