use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Challenge {
    pub id: Uuid,
    pub description: String,
    pub questions: Vec<Question>,
    pub points: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Question {
    pub id: Uuid,
    pub question: String,
    pub options: Vec<String>,
    pub correct_option: u8,
    pub point: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attempt {
    pub challenge_id: Uuid,
    pub question_id: Uuid,
    pub user_option: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Response {
    pub point: u8,
}
