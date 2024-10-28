pub trait StrToOwned {
    fn owned_string(self) -> String;
}

impl StrToOwned for String {
    fn owned_string(self) -> String {
        self
    }
}

impl StrToOwned for &str {
    fn owned_string(self) -> String {
        self.to_owned()
    }
}

impl StrToOwned for &String {
    fn owned_string(self) -> String {
        self.to_owned()
    }
}