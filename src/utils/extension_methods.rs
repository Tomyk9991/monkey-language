pub trait RemoveWhiteSpacesBetween {
    fn remove_whitespaces_between(&self) -> String;
}

impl RemoveWhiteSpacesBetween for String {
    fn remove_whitespaces_between(&self) -> String {
        self.trim().split_whitespace().collect::<Vec<&str>>().join(" ")
    }
}

impl RemoveWhiteSpacesBetween for &str {
    fn remove_whitespaces_between(&self) -> String {
        self.trim().split_whitespace().collect::<Vec<&str>>().join(" ")
    }
}