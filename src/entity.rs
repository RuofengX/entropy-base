use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub password: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct PlayerInfo {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub struct Guest {
    pub id: i32,
    pub energy: i64,
    pub pos: (i16, i16),
    pub temperature: i8,
    pub master_id: i32,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub struct GuestInfo {
    pub id: i32,
    pub temperature: i16,
    pub pos: (i16, i16),
    pub master_id: i32,
}
