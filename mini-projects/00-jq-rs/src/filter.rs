use std::str::Chars;

use serde_json::Value;

pub fn filter(json: &Value, filter: &str) -> Value {
    let mut values = json.clone();
    let filter_parts = filter.split('|');
    for next_filter in filter_parts {
        values = apply_filter(&values, next_filter.trim());
    }
    values
}

fn apply_filter(json: &Value, filter: &str) -> Value {
    let mut values = json.clone();
    let mut filter_chars = filter.chars();
    if filter_chars.next() == Some('.') {
        if filter_chars.clone().count() == 0 {
            // Return the entire JSON if the filter is empty
        } else if filter_chars.next() == Some('[') {
            values = index_array(json, filter_chars);
        } else {
            let filter = &filter.chars().skip(1).collect::<String>();
            if values.is_object() {
                values = values.get(filter).unwrap_or(&Value::Null).clone();
            } else {
                panic!("Cannot index into a non-object");
            }
        }
    } else {
        panic!("Filter must start with a '.'");
    }

    values
}

fn index_array(json: &Value, filter_chars: Chars) -> Value {
    let mut indexer = String::new();
    let mut reached_end = true;
    // Parse the index
    for c in filter_chars {
        if c == ']' {
            reached_end = false;
            break;
        }
        indexer.push(c);
    }

    if reached_end {
        panic!("Unclosed bracket in filter");
    }

    if json.is_array() {
        let array = json.as_array().unwrap();
        if indexer.is_empty() {
            return serde_json::Value::Array(json.as_array().unwrap().clone());
        }
        // Check if the index is a range
        else if indexer.contains(':') {
            let parts: Vec<&str> = indexer.split(':').collect();
            let start = parts[0].parse::<usize>().unwrap_or(0);
            let end = parts
                .get(1)
                .and_then(|&s| s.parse::<usize>().ok())
                .unwrap_or(array.len());
            // Return the slice of the array
            if start < end && end <= array.len() {
                let slice = &array[start..end];
                return Value::Array(slice.to_vec());
            } else {
                panic!("Index range out of bounds");
            }
        // Parse the index as a single value
        } else {
            let index = indexer.parse::<usize>().unwrap();
            return array
                .get(index)
                .unwrap_or_else(|| panic!("Index out of range"))
                .clone();
        }
    } else {
        panic!("Cannot index into a non-array");
    }
}
