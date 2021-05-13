// ** src/models.rs

use crate::schema::*;

// ** ----- GET Requests ----- **

#[derive(serde::Serialize, diesel::Queryable)]
#[serde(rename_all = "camelCase")]
pub struct Board {
    pub id: i64,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(serde::Serialize, diesel::Queryable)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub id: i64,
    pub board_id: i64,
    pub description: String,
    pub status: Status,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(serde::Serialize, serde::Deserialize, diesel_derive_enum::DbEnum, std::fmt::Debug)]
#[serde(rename_all = "camelCase")]
#[DieselType = "Status_enum"]
pub enum Status {
    Todo,
    Doing,
    Done,
}

#[derive(serde::Serialize)]
pub struct BoardSummary {
    pub todo: i64,
    pub doing: i64,
    pub done: i64,
}

// ** ----- POST Requests ----- **

#[derive(serde::Deserialize)]
pub struct CreateBoard {
    pub name: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCard {
    pub board_id: i64,
    pub description: String,
}

// ** ----- PATCH Requests ----- **

#[derive(serde::Deserialize)]
pub struct UpdateCard {
    pub description: String,
    pub status: Status,
}
