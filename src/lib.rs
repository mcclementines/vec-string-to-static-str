//! src/lib.rs

/// Converts a vector of `String`s into a vector of `&'static str`.
///
/// This function leaks memory as it uses `Box::leak` to create
/// static string slices from the input strings. Use with caution.
///
/// # Arguments
///
/// * `strings` - A vector of `String`s to be converted.
///
/// # Returns
///
/// A vector of `&'static str` references to the input strings.
///
/// # Example
///
/// ```
/// use vec_string_to_static_str::vec_string_to_static_str;
///
/// let strings = vec![String::from("hello"), String::from("world")];
/// let static_strs = vec_string_to_static_str(&strings);
/// assert_eq!(static_strs, vec!["hello", "world"]);
/// ```
pub fn vec_string_to_static_str(strings: &Vec<String>) -> Vec<&'static str> {
    let mut strs: Vec<&'static str> = Vec::new();

    for string in strings {
        strs.push(Box::leak(string.clone().into_boxed_str()));
    }

    strs
}

#[cfg(feature = "unsafe")]
/// Unsafely converts a vector of `String`s into a vector of `&'static str`.
///
/// This function uses `std::mem::transmute` to convert string slices
/// into static string slices without leaking memory. Use this function
/// only when you are certain of the lifetime safety.
///
/// # Arguments
///
/// * `strings` - A vector of `String`s to be converted.
///
/// # Returns
///
/// A vector of `&'static str` references to the input strings.
///
/// # Safety
///
/// This function is unsafe because it extends the lifetime of the string
/// slices to `'static`, which can cause undefined behavior if not used correctly.
///
/// # Example
///
/// ```
/// use vec_string_to_static_str::unsafe_vec_string_to_static_str;
///
/// let strings = vec![String::from("hello"), String::from("world")];
/// let static_strs = unsafe_vec_string_to_static_str(&strings);
/// assert_eq!(static_strs, vec!["hello", "world"]);
/// ```
pub fn unsafe_vec_string_to_static_str(strings: &Vec<String>) -> Vec<&'static str> {
    let mut strs: Vec<&'static str> = Vec::new();

    for string in strings {
        strs.push(unsafe { std::mem::transmute::<&str, &'static str>(string.as_str()) });
    }

    strs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_string_to_static_str_from_literals() {
        let strings = vec!["string_a".to_string(), "string_b".to_string()];

        let actual = vec_string_to_static_str(&strings);

        assert_eq!(vec!["string_a", "string_b"], actual);
    }

    #[test]
    fn vec_string_to_static_str_from_dynamic_allocation() {
        let strings = vec![String::from("string_a"), String::from("string_b")];

        let actual = vec_string_to_static_str(&strings);

        assert_eq!(vec!["string_a", "string_b"], actual);
    }
    #[test]
    fn vec_string_to_static_str_empty_vector() {
        let strings: Vec<String> = Vec::new();

        let actual = vec_string_to_static_str(&strings);

        assert_eq!(Vec::<&'static str>::new(), actual);
    }

    #[test]
    fn vec_string_to_static_str_mixed_content() {
        let strings = vec!["".to_string(), "a".to_string(), "longer string".to_string()];

        let actual = vec_string_to_static_str(&strings);

        assert_eq!(vec!["", "a", "longer string"], actual);
    }

    #[test]
    fn vec_string_to_static_str_special_characters() {
        let strings = vec![
            "hello, world!".to_string(),
            "你好，世界！".to_string(),
            "こんにちは、世界！".to_string(),
            "안녕하세요, 세계!".to_string(),
        ];

        let actual = vec_string_to_static_str(&strings);

        assert_eq!(
            vec![
                "hello, world!",
                "你好，世界！",
                "こんにちは、世界！",
                "안녕하세요, 세계!"
            ],
            actual
        );
    }

    #[test]
    #[cfg(feature = "unsafe")]
    fn unsafe_vec_string_to_static_str_from_literals() {
        let strings = vec!["string_a".to_string(), "string_b".to_string()];

        let actual = unsafe_vec_string_to_static_str(&strings);

        assert_eq!(vec!["string_a", "string_b"], actual);
    }

    #[test]
    #[cfg(feature = "unsafe")]
    fn unsafe_vec_string_to_static_str_from_dynamic_allocation() {
        let strings = vec![String::from("string_a"), String::from("string_b")];

        let actual = unsafe_vec_string_to_static_str(&strings);

        assert_eq!(vec!["string_a", "string_b"], actual);
    }

    #[test]
    #[cfg(feature = "unsafe")]
    fn unsafe_vec_string_to_static_str_empty_vector() {
        let strings: Vec<String> = Vec::new();

        let actual = unsafe_vec_string_to_static_str(&strings);

        assert_eq!(Vec::<&'static str>::new(), actual);
    }

    #[test]
    #[cfg(feature = "unsafe")]
    fn unsafe_vec_string_to_static_str_mixed_content() {
        let strings = vec!["".to_string(), "a".to_string(), "longer string".to_string()];

        let actual = unsafe_vec_string_to_static_str(&strings);

        assert_eq!(vec!["", "a", "longer string"], actual);
    }

    #[test]
    #[cfg(feature = "unsafe")]
    fn unsafe_vec_string_to_static_str_special_characters() {
        let strings = vec![
            "hello, world!".to_string(),
            "你好，世界！".to_string(),
            "こんにちは、世界！".to_string(),
            "안녕하세요, 세계!".to_string(),
        ];

        let actual = unsafe_vec_string_to_static_str(&strings);

        assert_eq!(
            vec![
                "hello, world!",
                "你好，世界！",
                "こんにちは、世界！",
                "안녕하세요, 세계!"
            ],
            actual
        );
    }
}
