use regex::Regex;
use std::collections::HashMap;

pub fn string_map_replacement(string: &str, replacements: &HashMap<String, String>) -> String {
    if replacements.is_empty() {
        return string.to_string();
    }

    let pattern = Regex::new(
        &replacements
            .keys()
            .map(|k| regex::escape(k))
            .collect::<Vec<String>>()
            .join("|"),
    )
    .unwrap();

    pattern
        .replace_all(string, |caps: &regex::Captures| {
            replacements.get(&caps[0]).unwrap()
        })
        .to_string()
}
