use std::{iter::FromIterator, mem};

pub fn remove_last_if_empty<'a>(items: &mut Vec<&'a str>) -> Vec<&'a str> {
    if items.last() == Some(&"") {
        items.remove(items.len() - 1);
    }
    items.to_vec()
}

pub fn remove_ending_cr<'a>(items: &Vec<&'a str>) -> Vec<&'a str> {
    items
        .iter()
        .map(|item| {
            if item.ends_with("\r") {
                item.trim_end_matches("\r")
            } else {
                item.clone()
            }
        })
        .collect()
}
pub fn create_switch_statement() {}
pub fn create_case_statement() {}

pub fn await_user_enter() {
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
