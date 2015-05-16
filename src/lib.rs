extern crate regex;
use regex::{Captures, Regex};
use std::ascii::AsciiExt;

pub trait Casey {
    /// "snake_case"
    fn to_snakecase(&self) -> String;

    /// "Upper_Snake_Case"
    fn to_upper_snakecase(&self) -> String;

    /// "SCREAMING_SNAKE_CASE"
    fn to_screaming_snakecase(&self) -> String;

    /// "camelCase"
    fn to_camelcase(&self) -> String;

    /// "UpperCamelCase"
    fn to_upper_camelcase(&self) -> String;

    /// "hyphen-case"
    fn to_hyphencase(&self) -> String;

    /// "Upper-Hyphen-Case"
    fn to_upper_hyphencase(&self) -> String;

    /// "SCREAMING-HYPHEN-CASE"
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
