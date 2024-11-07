#[derive(Debug)]
pub struct EmailAddress(String);

#[derive(Debug)]
pub struct Password(String);

impl From<String> for EmailAddress {
    fn from(value: String) -> Self {
        EmailAddress(value)
    }
}

impl EmailAddress {
    pub fn to_string(self) -> String {
        self.0
    }
}

impl From<String> for Password {
    fn from(value: String) -> Self {
        Password(value)
    }
}

impl Password {
    pub fn to_string(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug)]
pub struct CreateUser {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub permission_system_setting: bool,
    pub permission_schedule: bool,
    pub permission_temporary_schedule: bool,
    pub permission_post_setting: bool,
}
