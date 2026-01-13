use regex::Regex;

pub fn normalize_and_extract_codes(text: &str) -> Option<String> {
    // 1. Define the Regex with two capture groups:
    //    Group 1: ([A-Z]+) -> The letters
    //    Optional Separator: -?
    //    Group 2: (\d+) -> The digits
    let re = Regex::new(r"([A-Z]+)-?(\d+)").unwrap(); // Safe to ignore unwrap() -> valid regex

    // 2. Define the replacement string using backreferences:
    //    $1: Content of Group 1
    //    -: A literal hyphen
    //    $2: Content of Group 2
    let replacement_template = "$1-$2";

    // 3. Use find_iter to iterate over all matches, and then map each match
    //    to its normalized replacement using `re.replace`.

    let normalized_codes: Vec<String> = re
        .find_iter(text)
        .map(|m| {
            // Get the entire matched string (e.g., "ABC123" or "DEF-456")
            let matched_string = m.as_str();

            // Apply the replacement ONLY to the current matched string.
            // `replace` takes a `&str` and returns the replaced content as a `Cow<'_, str>`.
            // We convert it to an owned String.
            let normalized = re.replace(matched_string, replacement_template);

            // Convert Cow<str> to String for the final vector
            normalized.to_string()
        })
        .collect();

    if normalized_codes.len() == 1 {
        Some(normalized_codes[0].clone())
    } else {
        if let Some(i) = normalized_codes.clone().into_iter().next() {
            return match i.len() {
                7 => Some(i.to_string()),
                8 => Some(i.to_string()),
                11 => Some(i.to_string()),
                _ => Some(normalized_codes[1].clone()),
            };
        }
        None
    }
}
