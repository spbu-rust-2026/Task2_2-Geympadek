fn main() {
    print!("{}", process(String::from("hello MATMEX")));
}

fn process(text: String) -> String {
    let mut text = text;
    let mut result = String::new();
    let first = match text.split_whitespace().next() {
        Some(word) => word,
        None => &text[0..0],
    };
    result.push_str(first);
    result.push('\n');
    result.push_str(&mut text);
    result.push_str(" rust\n");
    result.push_str(&text);
    result.push_str(" rust\n");
    result
}

#[cfg(test)]
#[path = "../tests/unit/process.rs"]
mod process_tests;
