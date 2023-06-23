use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct User {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
}
