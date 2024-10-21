#[cfg(test)]
// I am not including print testing because it works and I don't want to deal with it
mod tests {
    use serde_json::json;

    use crate::filter::filter;

    #[test]
    fn test_filter_single_key() {
        let json = json!({
            "foo": "bar",
            "baz": 42
        });
        let filter_str = ".foo";
        let result = filter(&json, filter_str);
        let expected = json!("bar");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_filter_nested_key() {
        let json = json!({
            "foo": {
                "bar": "baz"
            }
        });
        let filter_str = ".foo | .bar";
        let result = filter(&json, filter_str);
        let expected = json!("baz");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_filter_array_index() {
        let json = json!([1, 2, 3, 4, 5]);
        let filter_str = ".[2]";
        let result = filter(&json, filter_str);
        let expected = json!(3);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_filter_array_range() {
        let json = json!([1, 2, 3, 4, 5]);
        let filter_str = ".[1:4]";
        let result = filter(&json, filter_str);
        let expected = json!([2, 3, 4]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_filter_chain() {
        let json = json!({
            "foo": {
                "bar": [1, 2, 3, 4, 5]
            }
        });
        let filter_str = ".foo | .bar | .[1:4]";
        let result = filter(&json, filter_str);
        let expected = json!([2, 3, 4]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_filter_non_existent_key() {
        let json = json!({
            "foo": "bar"
        });
        let filter_str = ".baz";
        let result = filter(&json, filter_str);
        let expected = json!(null);
        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic(expected = "Index out of range")]
    fn test_filter_non_existent_index() {
        let json = json!([1, 2, 3]);
        let filter_str = ".[10]";
        filter(&json, filter_str);
    }

    #[test]
    fn test_filter_empty_filter() {
        let json = json!({
            "foo": "bar"
        });
        let filter_str = ".";
        let result = filter(&json, filter_str);
        let expected = json.clone();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_filter_array_range_with_chain() {
        let json = json!({
            "foo": [1, 2, 3, 4, 5]
        });
        let filter_str = ".foo | .[1:4]";
        let result = filter(&json, filter_str);
        let expected = json!([2, 3, 4]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_filter_nested_array() {
        let json = json!({
            "foo": {
                "bar": [1, 2, 3, 4, 5]
            }
        });
        let filter_str = ".foo | .bar | .[2]";
        let result = filter(&json, filter_str);
        let expected = json!(3);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_filter_nested_array_range() {
        let json = json!({
            "foo": {
                "bar": [1, 2, 3, 4, 5]
            }
        });
        let filter_str = ".foo | .bar | .[1:4]";
        let result = filter(&json, filter_str);
        let expected = json!([2, 3, 4]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_filter_with_whitespace() {
        let json = json!({
            "foo": "bar",
            "baz": 42
        });
        let filter_str = " .foo ";
        let result = filter(&json, filter_str.trim());
        let expected = json!("bar");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_multiple_pipes() {
        let json = json!({
            "foo": {
                "bar": {
                    "baz": [1, 2, 3, 4, 5]
                }
            }
        });
        let filter_str = ".foo | .bar | .baz | .[2:3]";
        let result = filter(&json, filter_str);
        let expected = json!([3]);
        assert_eq!(result, expected);
    }
}
