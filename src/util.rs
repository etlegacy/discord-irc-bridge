use lazy_static::lazy_static;
use regex::Regex;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher as _};

#[inline]
pub fn hash<T: Hash>(t: &T) -> u64 {
    let mut h = DefaultHasher::new();
    t.hash(&mut h);
    h.finish()
}

#[inline]
pub fn colorize(s: &str) -> u64 {
    hash(&s) % 16
}

pub fn remove_formatting<'t>(s: &'t str) -> std::borrow::Cow<'t, str> {
    lazy_static! {
        static ref RE: Regex = Regex::new("[\x02\x1F\x0F\x16]|\x03(\\d\\d?(,\\d\\d?)?)?").unwrap();
    }
    RE.replace_all(s, "")
}


pub fn contains_bad_words(content: &str, badwords: &[String]) -> bool {
    content.split_whitespace().any(|w| badwords.iter().any(|b| b == w))
}
