//use serde::{Deserialize, Serialize};


//#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub uuid: String,
}

//#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub message: String,
    pub sender: User,
}

impl User {
    pub fn new(uuid: String) -> Self {
        Self { uuid }
    }
}

impl Message {
    pub fn new(message: String, sender: User) -> Self {
        Self { message, sender}
    }
}
