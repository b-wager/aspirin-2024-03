use colored::{Color, Colorize};
use serde_json::Value;

pub fn print_json(json: &Value, current_indent: usize, original_indent: usize) {
    match json {
        Value::Object(ref map) => {
            println!("{{");
            for (key, value) in map {
                print!("{}\"{}\": ", " ".repeat(current_indent), key);
                print_json(value, current_indent + original_indent, original_indent);
            }
            println!("{}}}", " ".repeat(current_indent - original_indent));
        }
        Value::Array(ref arr) => {
            println!("[");
            for value in arr {
                print!("{}", " ".repeat(current_indent));
                print_json(value, current_indent + original_indent, original_indent);
            }
            println!("{}]", " ".repeat(current_indent - original_indent));
        }
        Value::String(ref s) => println!("\"{}\"", s),
        Value::Number(ref n) => println!("{}", n),
        Value::Bool(ref b) => println!("{}", b),
        Value::Null => println!("null"),
    }
}

pub fn print_compact_json(json: &Value) {
    match json {
        Value::Object(ref map) => {
            print!("{{");
            let mut iter = map.iter().peekable();
            while let Some((key, value)) = iter.next() {
                print!("\"{}\":", key);
                print_compact_json(value);
                if iter.peek().is_some() {
                    print!(",");
                }
            }
            print!("}}");
        }
        Value::Array(ref arr) => {
            print!("[");
            let mut iter = arr.iter().peekable();
            while let Some(value) = iter.next() {
                print_compact_json(value);
                if iter.peek().is_some() {
                    print!(",");
                }
            }
            print!("]");
        }
        Value::String(ref s) => print!("\"{}\"", s),
        Value::Number(ref n) => print!("{}", n),
        Value::Bool(ref b) => print!("{}", b),
        Value::Null => print!("null"),
    }
}

pub fn print_json_color(json: &Value, current_indent: usize, original_indent: usize) {
    match json {
        Value::Object(ref map) => {
            println!("{{");
            for (key, value) in map {
                print!(
                    "{}{}: ",
                    " ".repeat(current_indent),
                    format!("\"{}\"", key).color(Color::Blue).bold()
                );
                print_json_color(value, current_indent + original_indent, original_indent);
            }
            println!("{}}}", " ".repeat(current_indent - original_indent));
        }
        Value::Array(ref arr) => {
            println!("[");
            for value in arr {
                print!("{}", " ".repeat(current_indent));
                print_json_color(value, current_indent + original_indent, original_indent);
            }
            println!("{}]", " ".repeat(current_indent - original_indent));
        }
        Value::String(ref s) => println!("{}", format!("\"{}\"", s).color(Color::Green)),
        Value::Number(ref n) => println!("{}", n),
        Value::Bool(ref b) => println!("{}", b),
        Value::Null => println!("{}", "null".color(Color::Black)),
    }
}

pub fn print_compact_json_color(json: &Value) {
    match json {
        Value::Object(ref map) => {
            print!("{{");
            let mut iter = map.iter().peekable();
            while let Some((key, value)) = iter.next() {
                print!("{}:", format!("\"{}\"", key).color(Color::Blue).bold());
                print_compact_json_color(value);
                if iter.peek().is_some() {
                    print!(",");
                }
            }
            print!("}}");
        }
        Value::Array(ref arr) => {
            print!("[");
            let mut iter = arr.iter().peekable();
            while let Some(value) = iter.next() {
                print_compact_json_color(value);
                if iter.peek().is_some() {
                    print!(",");
                }
            }
            print!("]");
        }
        Value::String(ref s) => print!("{}", format!("\"{}\"", s).color(Color::Green)),
        Value::Number(ref n) => print!("{}", n),
        Value::Bool(ref b) => print!("{}", b),
        Value::Null => print!("{}", "null".color(Color::Black)),
    }
}
