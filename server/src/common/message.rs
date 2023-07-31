use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub uuid: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub message: String,
    pub sender: User,
    pub receiver: User,
}

impl User {
    pub fn new(uuid: u64) -> Self {
        Self { uuid }
    }
}

impl Message {
    pub fn new(message: String, sender: User, receiver: User) -> Self {
        Self { message, sender, receiver }
    }
}
