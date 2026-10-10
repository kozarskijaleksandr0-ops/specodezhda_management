use serde::{Deserialize, Serialize};

//Цех 

#[derive(Deserialize)]
pub struct AddCehPayload {
    pub name: String,
    pub nachalnik: String,
}

#[derive(Deserialize)]
pub struct UpdateCehPayload {
    pub name: String,
    pub nachalnik: String,
}

#[derive(Serialize)]
pub struct CehResponse {
    pub id: String,
    pub name: String,
    pub nachalnik: String,
}

//Спецодежда

#[derive(Deserialize)]
pub struct AddSpecodezhdaPayload {
    pub vid: String,
    pub srok_noski: i64,
    pub stoimost: f64,
}

#[derive(Deserialize)]
pub struct UpdateSpecodezhdaPayload {
    pub vid: String,
    pub srok_noski: i64,
    pub stoimost: f64,
}

#[derive(Serialize)]
pub struct SpecodezhdaResponse {
    pub id: String,
    pub vid: String,
    pub srok_noski: i64,
    pub stoimost: f64,
}

//Работник

#[derive(Deserialize)]
pub struct AddRabotnikPayload {
    pub fio: String,
    pub dolzhnost: String,
    pub skidka: f64,
    pub ceh_id: String,
}

#[derive(Deserialize)]
pub struct UpdateRabotnikPayload {
    pub fio: String,
    pub dolzhnost: String,
    pub skidka: f64,
    pub ceh_id: String,
}

#[derive(Serialize)]
pub struct RabotnikResponse {
    pub id: String,
    pub fio: String,
    pub dolzhnost: String,
    pub skidka: f64,
    pub ceh_id: String,
}

//Получение

#[derive(Deserialize)]
pub struct AddPolucheniePayload {
    pub rabotnik_id: String,
    pub specodezhda_id: String,
    pub data_polucheniya: String, // ISO 8601, например "2026-09-30T12:00:00Z"
    pub podpis: String,
}

#[derive(Serialize)]
pub struct PoluchenieResponse {
    pub rabotnik_id: String,
    pub specodezhda_id: String,
    pub data_polucheniya: String,
    pub podpis: String,
}

//Общий ответ об ошибке
#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
}