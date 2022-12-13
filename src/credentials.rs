// Jackson Coxson

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Credentials {
    pub username: String,
    pub token: String,
}

impl Credentials {
    pub fn new(username: impl AsRef<str>, token: impl AsRef<str>) -> Self {
        Self {
            username: username.as_ref().to_string(),
            token: token.as_ref().to_string(),
        }
    }
}
