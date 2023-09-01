pub trait RemoveWhiteSpacesBetween {
    fn remove_whitespaces_between(&self) -> String;
}

impl<T: ToString> RemoveWhiteSpacesBetween for T {
    /// Replaces all whitespaces between words with a single whitespace.s
    /// # Examples:
    /// ```
    /// use monkey_language::utils::extension_methods::RemoveWhiteSpacesBetween;
    /// let text = "Hello   World";
    /// assert_eq!(text.remove_whitespaces_between(), "Hello World");
    /// ```
    /// ```
    /// use monkey_language::utils::extension_methods::RemoveWhiteSpacesBetween;
    /// let text = "Hello   World   ";
    /// assert_eq!(text.remove_whitespaces_between(), "Hello World");
    /// ```
    fn remove_whitespaces_between(&self) -> String {
        self.to_string().split_whitespace().collect::<Vec<&str>>().join(" ")
    }
}