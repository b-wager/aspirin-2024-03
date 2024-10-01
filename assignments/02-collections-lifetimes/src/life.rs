fn split_string<'a>(string: &'a str, delimiter: &str) -> Vec<&'a str> {
    let mut split_string: Vec<&str> = Vec::new();
    let mut start = 0;

    while start < string.len() {
        if let Some(pos) = string[start..].find(delimiter) {
            split_string.push(&string[start..start + pos]);
            start += pos + delimiter.len();
        } else {
            split_string.push(&string[start..]);
            break;
        }
    }

    split_string
}

#[derive(PartialEq, Debug)]
struct Differences<'a> {
    only_in_first: Vec<&'a str>,
    only_in_second: Vec<&'a str>,
}

fn find_differences<'a>(first_string: &'a str, second_string: &'a str) -> Differences<'a> {
    let mut only_in_first: Vec<&str> = Vec::new();
    let mut only_in_second: Vec<&str> = Vec::new();

    for word in first_string.split_whitespace() {
        if !second_string.contains(word) {
            only_in_first.push(word);
        }
    }

    for word in second_string.split_whitespace() {
        if !first_string.contains(word) {
            only_in_second.push(word);
        }
    }

    Differences {
        only_in_first,
        only_in_second,
    }
}

fn merge_names(first_name: &str, second_name: &str) -> String {
    let mut merged_name = String::new();
    let first_name_chars: Vec<char> = first_name.chars().collect();
    let second_name_chars: Vec<char> = second_name.chars().collect();
    let mut first_index = 0;
    let mut second_index = 0;

    loop {
        let mut added_char = false;

        // Add the next character of the first name
        if first_index < first_name_chars.len() {
            merged_name.push(first_name_chars[first_index]);
            first_index += 1;
            added_char = true;
        }
        // Add characters from the first name until a vowel is found
        while first_index < first_name_chars.len() {
            let first_char = first_name_chars[first_index];
            if !matches!(first_char, 'a' | 'e' | 'i' | 'o' | 'u') {
                merged_name.push(first_char);
                first_index += 1;
                added_char = true;
            } else {
                break;
            }
        }

        // Add the next character of the second name
        if second_index < second_name_chars.len() {
            merged_name.push(second_name_chars[second_index]);
            second_index += 1;
            added_char = true;
        }
        // Add characters from the second name until a vowel is found
        while second_index < second_name_chars.len() {
            let second_char = second_name_chars[second_index];
            if !matches!(second_char, 'a' | 'e' | 'i' | 'o' | 'u') {
                merged_name.push(second_char);
                second_index += 1;
                added_char = true;
            } else {
                break;
            }
        }

        if !added_char {
            break;
        }
    }

    merged_name
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_string() {
        // First, make sure the lifetimes were correctly marked
        let matches;
        let string_to_split = String::from("Hello, World!");

        {
            let delimeter = String::from(", ");
            matches = split_string(&string_to_split, &delimeter);
        }
        println!("Matches can be printed! See: {:?}", matches);

        // Now check the split logic
        assert_eq!(split_string(&"", &""), Vec::<&str>::new());
        assert_eq!(
            split_string(&"Hello, World!", &", "),
            vec!["Hello", "World!"]
        );
        assert_eq!(
            split_string(
                &"I this think this that this sentence this is this very this confusing this ",
                &" this "
            ),
            vec!["I", "think", "that", "sentence", "is", "very", "confusing"]
        );
        assert_eq!(
            split_string(&"apple🍎banana🍎orange", &"🍎"),
            vec!["apple", "banana", "orange"]
        );
        assert_eq!(
            split_string(
                &"Ayush;put|a,lot~of`random;delimeters|in|this,sentence",
                &";"
            ),
            vec![
                "Ayush",
                "put|a,lot~of`random",
                "delimeters|in|this,sentence"
            ]
        );
    }

    #[test]
    fn test_find_differences() {
        assert_eq!(
            find_differences(&"", &""),
            Differences {
                only_in_first: Vec::new(),
                only_in_second: Vec::new()
            }
        );
        assert_eq!(
            find_differences(&"pineapple pen", &"apple"),
            Differences {
                only_in_first: vec!["pineapple", "pen"],
                only_in_second: Vec::new()
            }
        );
        assert_eq!(
            find_differences(
                &"Sally sold seashells at the seashore",
                &"Seashells seashells at the seashore"
            ),
            Differences {
                only_in_first: vec!["Sally", "sold"],
                only_in_second: vec!["Seashells"]
            }
        );
        assert_eq!(
            find_differences(
                "How much wood could a wood chuck chuck",
                "If a wood chuck could chuck wood"
            ),
            Differences {
                only_in_first: vec!["How", "much"],
                only_in_second: vec!["If"]
            }
        );
        assert_eq!(
            find_differences(
                &"How much ground would a groundhog hog",
                &"If a groundhog could hog ground"
            ),
            Differences {
                only_in_first: vec!["How", "much", "would"],
                only_in_second: vec!["If", "could"]
            }
        );
    }

    #[test]
    fn test_merge_names() {
        assert_eq!(merge_names(&"alex", &"jake"), "aljexake");
        assert_eq!(merge_names(&"steven", &"stephen"), "ststevephenen");
        assert_eq!(merge_names(&"gym", &"rhythm"), "gymrhythm");
        assert_eq!(merge_names(&"walter", &"gibraltor"), "wgaltibreraltor");
        assert_eq!(merge_names(&"baker", &"quaker"), "bqakueraker");
        assert_eq!(merge_names(&"", &""), "");
        assert_eq!(merge_names(&"samesies", &"samesies"), "ssamamesesiieses");
        assert_eq!(merge_names(&"heather", &"meagan"), "hmeeathageran");
        assert_eq!(merge_names(&"panda", &"turtle"), "ptandurtlae");
        assert_eq!(merge_names(&"hot", &"sauce"), "hsotauce");
        assert_eq!(merge_names(&"", &"second"), "second");
        assert_eq!(merge_names(&"first", &""), "first");
    }
}
