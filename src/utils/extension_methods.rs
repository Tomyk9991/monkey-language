pub trait RemoveWhiteSpacesBetween {
    fn remove_whitespaces_between(&self) -> String;
}

impl<T: ToString> RemoveWhiteSpacesBetween for T {
    /// Replaces all whitespaces between words with a single whitespace.s
    fn remove_whitespaces_between(&self) -> String {
        self.to_string().split_whitespace().collect::<Vec<&str>>().join(" ")
    }
}