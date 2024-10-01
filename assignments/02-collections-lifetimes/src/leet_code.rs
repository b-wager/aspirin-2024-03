use std::collections::HashMap;

fn longest_equal_sequence_prescriptive<T: PartialEq>(sequence: &[T]) -> i32 {
    let mut current_sequence = 0;
    let mut longest_sequence = 0;
    for i in 0..sequence.len() {
        if i == 0 || sequence[i] == sequence[i - 1] {
            current_sequence += 1;
        } else {
            current_sequence = 1;
        }
        if current_sequence > longest_sequence {
            longest_sequence = current_sequence;
        }
    }
    longest_sequence
}

fn longest_equal_sequence_functional<T: PartialEq>(sequence: &[T]) -> i32 {
    sequence
        .iter()
        .zip(sequence.iter().skip(1))
        .fold((1, 1), |(longest, current), (a, b)| {
            if a == b {
                (longest.max(current + 1), current + 1)
            } else {
                (longest.max(current), 1)
            }
        })
        .0
        .min(sequence.len() as i32) // only needed if the sequence is empty
}

fn is_valid_paranthesis(parenthesis: &str) -> bool {
    let parenthesis_chars = parenthesis.chars();
    let mut stack = Vec::new();
    for c in parenthesis_chars {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            _ => {}
        }
    }
    stack.is_empty()
}

fn longest_common_substring<'a>(first_str: &'a str, second_str: &str) -> &'a str {
    let mut substrings = HashMap::new();
    let first_len = first_str.len();
    let second_len = second_str.len();
    let mut longest_length = 0;
    let mut longest_substring = "";
    // Store all substrings of second_str in the HashMap
    for i in 0..second_len {
        for j in i + 1..=second_len {
            let substring = &second_str[i..j];
            substrings.insert(substring, substring.len());
        }
    }
    // Check for the longest common substring between first_str and the HashMap
    for i in 0..first_len {
        for j in i + 1..=first_len {
            let substring = &first_str[i..j];
            if let Some(&length) = substrings.get(substring) {
                if length > longest_length {
                    longest_length = length;
                    longest_substring = substring;
                }
            }
        }
    }
    longest_substring
}

fn longest_common_substring_multiple<'a>(strings: &'a [&str]) -> &'a str {
    let mut common_substring = strings[0];
    for string in strings.iter().skip(1) {
        common_substring = longest_common_substring(common_substring, string);
    }
    common_substring
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_equal_sequence_prescriptive() {
        assert_eq!(longest_equal_sequence_prescriptive(&vec![1, 1, 1, 1, 1]), 5);
        assert_eq!(
            longest_equal_sequence_prescriptive(&vec![1.0, 2.0, 2.0, 2.0, 3.0, 4.0, 4.0]),
            3
        );
        assert_eq!(longest_equal_sequence_prescriptive(&vec![-100]), 1);
        let empty_vec: Vec<char> = Vec::new();
        assert_eq!(longest_equal_sequence_prescriptive(&empty_vec), 0);
        assert_eq!(
            longest_equal_sequence_prescriptive(&vec![
                1000, 1000, 2000, 2000, 2000, 3000, 3000, 3000, 3000
            ]),
            4
        );
        assert_eq!(
            longest_equal_sequence_prescriptive(&vec!['a', 'b', 'a', 'b', 'a', 'b']),
            1
        );
        let vec: Vec<u8> = vec![5, 5, 5, 1, 2, 3];
        assert_eq!(longest_equal_sequence_prescriptive(&vec), 3);
        assert_eq!(
            longest_equal_sequence_prescriptive(&vec![1, 2, 3, 4, 4, 4]),
            3
        );
        assert_eq!(longest_equal_sequence_prescriptive(&vec![1, 2, 3, 4, 5]), 1);
        assert_eq!(
            longest_equal_sequence_prescriptive(&vec![1, 1, 2, 2, 2, 3, 1, 1, 1, 1, 1]),
            5
        );
    }
    #[test]
    fn test_longest_equal_sequence_functional() {
        assert_eq!(longest_equal_sequence_functional(&vec![1, 1, 1, 1, 1]), 5);
        assert_eq!(
            longest_equal_sequence_functional(&vec![1.0, 2.0, 2.0, 2.0, 3.0, 4.0, 4.0]),
            3
        );
        assert_eq!(longest_equal_sequence_functional(&vec![-100]), 1);
        let empty_vec: Vec<char> = Vec::new();
        assert_eq!(longest_equal_sequence_functional(&empty_vec), 0);
        assert_eq!(
            longest_equal_sequence_functional(&vec![
                1000, 1000, 2000, 2000, 2000, 3000, 3000, 3000, 3000
            ]),
            4
        );
        assert_eq!(
            longest_equal_sequence_functional(&vec!['a', 'b', 'a', 'b', 'a', 'b']),
            1
        );
        let vec: Vec<u8> = vec![5, 5, 5, 1, 2, 3];
        assert_eq!(longest_equal_sequence_functional(&vec), 3);
        assert_eq!(
            longest_equal_sequence_functional(&vec![1, 2, 3, 4, 4, 4]),
            3
        );
        assert_eq!(longest_equal_sequence_functional(&vec![1, 2, 3, 4, 5]), 1);
        assert_eq!(
            longest_equal_sequence_functional(&vec![1, 1, 2, 2, 2, 3, 1, 1, 1, 1, 1]),
            5
        );
    }

    #[test]
    fn test_is_valid_paranthesis() {
        assert_eq!(is_valid_paranthesis(&String::from("{}")), true);
        assert_eq!(is_valid_paranthesis(&String::from("()")), true);
        assert_eq!(is_valid_paranthesis(&String::from("()[]{}")), true);
        assert_eq!(is_valid_paranthesis(&String::from("({[]})")), true);
        assert_eq!(is_valid_paranthesis(&String::from("([]){}{}([]){}")), true);
        assert_eq!(is_valid_paranthesis(&String::from("()(")), false);
        assert_eq!(is_valid_paranthesis(&String::from("(()")), false);
        assert_eq!(is_valid_paranthesis(&String::from("([)]{[})")), false);
        assert_eq!(
            is_valid_paranthesis(&String::from("({[()]}){[([)]}")),
            false
        );
        assert_eq!(
            is_valid_paranthesis(&String::from("()[]{}(([])){[()]}(")),
            false
        );
    }

    #[test]
    fn test_common_substring() {
        assert_eq!(longest_common_substring(&"abcdefg", &"bcdef"), "bcdef");
        assert_eq!(longest_common_substring(&"apple", &"pineapple"), "apple");
        assert_eq!(longest_common_substring(&"dog", &"cat"), "");
        assert_eq!(longest_common_substring(&"racecar", &"racecar"), "racecar");
        assert_eq!(longest_common_substring(&"ababc", &"babca"), "babc");
        assert_eq!(longest_common_substring(&"xyzabcxyz", &"abc"), "abc");
        assert_eq!(longest_common_substring(&"", &"abc"), "");
        assert_eq!(longest_common_substring(&"abcdefgh", &"defghijk"), "defgh");
        assert_eq!(longest_common_substring(&"xyabcz", &"abcxy"), "abc");
        assert_eq!(longest_common_substring(&"ABCDEFG", &"abcdefg"), "");
        // assert_eq!(
        //     longest_common_substring(
        //         &"thisisaverylongstringwithacommonsubstring",
        //         &"anotherlongstringwithacommonsubstring"
        //     ),
        //     "longstringwithacommonsubstring"
        // );
        assert_eq!(longest_common_substring("a", "a"), "a");
    }

    #[test]
    fn test_common_substring_multiple() {
        assert_eq!(
            longest_common_substring_multiple(&vec!["abcdefg", "cdef"]),
            "cdef"
        );
        assert_eq!(
            longest_common_substring_multiple(&vec!["apple", "pineapple", "maple", "snapple"]),
            "ple"
        );
        assert_eq!(
            longest_common_substring_multiple(&vec!["dog", "cat", "fish"]),
            ""
        );
        assert_eq!(
            longest_common_substring_multiple(&vec!["racecar", "car", "scar"]),
            "car"
        );
        assert_eq!(
            longest_common_substring_multiple(&vec!["ababc", "babca", "abcab"]),
            "abc"
        );
        assert_eq!(
            longest_common_substring_multiple(&vec!["xyzabcxyz", "abc", "zabcy", "abc"]),
            "abc"
        );
        assert_eq!(
            longest_common_substring_multiple(&vec!["", "abc", "def"]),
            ""
        );
        assert_eq!(
            longest_common_substring_multiple(&vec![
                "abcdefgh",
                "bcd",
                "bcdtravels",
                "abcs",
                "webcam"
            ]),
            "bc"
        );
        assert_eq!(
            longest_common_substring_multiple(&vec!["identical", "identical", "identical"]),
            "identical"
        );
        assert_eq!(
            longest_common_substring_multiple(&vec!["xyabcz", "abcxy", "zabc"]),
            "abc"
        );
        assert_eq!(longest_common_substring_multiple(&vec!["a", "a", "a"]), "a");
        assert_eq!(
            longest_common_substring_multiple(&vec![
                "thisisaverylongstringwiththecommonsubstring",
                "anotherlongstringwithacommonsubstring",
                "yetanotherstringthatcontainsacommonsubstring",
            ]),
            "commonsubstring",
        );
    }
}
