use regex::Regex;

pub fn str_i32(str: &str) -> Option<(&str, Option<i32>)> {
    let s = get_first_match(str, r"([a-z]|[A-Z])+")?;
    let i32 = get_first_match(str, r"\d+|-\d+")?.parse::<i32>().ok();
    Some((s, i32))
}

pub fn get_first_match<'a>(str: &'a str, reg: &str) -> Option<&'a str> {
    Some(Regex::new(reg).ok()?.captures(str)?.get(0)?.as_str())
}
