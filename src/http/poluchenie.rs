<<<<<<< HEAD
use ohkami::{Response, Status};

pub async fn add_poluchenie() -> Response {
    let mut response = Response::new(Status::OK);
    response.set_text("added poluchenie");
    response
}

pub async fn get_poluchenie() -> Response {
    let mut response = Response::new(Status::OK);
    response.set_text("got poluchenie");
    response
}

pub async fn update_poluchenie() -> Response {
    let mut response = Response::new(Status::OK);
    response.set_text("updated poluchenie");
    response
}

pub async fn remove_poluchenie() -> Response {
    let mut response = Response::new(Status::OK);
    response.set_text("removed poluchenie");
    response
}
=======
use ohkami::{Json, Response};
use ohkami::fang::Context;
use ohkami::claw::Query;
use serde::Deserialize;
use uuid::Uuid;
use time::format_description::well_known::Rfc3339;
use time::UtcDateTime;

use crate::context::RequestContext;
use crate::contracts::PoluchenieStorage;
use crate::contracts_impl::PoluchenieSqliteStorage;
use crate::models::Poluchenie;

use super::payloads::{AddPolucheniePayload, PoluchenieResponse, ErrorResponse};

#[derive(Deserialize)]
pub struct PoluchenieQuery {
    pub rabotnik_id: String,
    pub specodezhda_id: String,
    pub data_polucheniya: String,
}

//POST
pub async fn add_poluchenie(
    Json(payload): Json<AddPolucheniePayload>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let Ok(rabotnik_id) = Uuid::parse_str(&payload.rabotnik_id) else {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "invalid rabotnik_id".to_string(),
        });
    };
    let Ok(specodezhda_id) = Uuid::parse_str(&payload.specodezhda_id) else {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "invalid specodezhda_id".to_string(),
        });
    };
    let Ok(data_polucheniya) = UtcDateTime::parse(&payload.data_polucheniya, &Rfc3339) else {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "invalid data_polucheniya (expect RFC3339)".to_string(),
        });
    };

    let rabotnik_stub = crate::models::Rabotnik {
        id: rabotnik_id,
        fio: String::new(),
        dolzhnost: String::new(),
        skidka: 0.0,
        ceh_id: Uuid::nil(),
        created_at: UtcDateTime::now(),
        updated_at: None,
        deleted_at: None,
    };
    let specodezhda_stub = crate::models::Specodezhda {
        id: specodezhda_id,
        vid: String::new(),
        srok_noski: 0,
        stoimost: 0.0,
        created_at: UtcDateTime::now(),
        updated_at: None,
        deleted_at: None,
    };

    let poluchenie = Poluchenie::create_new(
        &rabotnik_stub,
        &specodezhda_stub,
        data_polucheniya,
        payload.podpis,
    );

    let response_payload = PoluchenieResponse {
        rabotnik_id: poluchenie.rabotnik_id.to_string(),
        specodezhda_id: poluchenie.specodezhda_id.to_string(),
        data_polucheniya: poluchenie.data_polucheniya.to_string(),
        podpis: poluchenie.podpis.clone(),
    };

    let storage = PoluchenieSqliteStorage::from_pool((*context.connection_pool).clone());

    match storage.add(poluchenie).await {
        Ok(()) => Response::Created().with_json(response_payload),
        Err(error) => {
            eprintln!("Ошибка добавления получения: {error}");
            Response::InternalServerError().with_json(ErrorResponse {
                message: "database error".to_string(),
            })
        }
    }
}

//GET
pub async fn get_poluchenie(
    Query(query): Query<PoluchenieQuery>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let Ok(rabotnik_id) = Uuid::parse_str(&query.rabotnik_id) else {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "invalid rabotnik_id".to_string(),
        });
    };
    let Ok(specodezhda_id) = Uuid::parse_str(&query.specodezhda_id) else {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "invalid specodezhda_id".to_string(),
        });
    };
    let Ok(data_polucheniya) = UtcDateTime::parse(&query.data_polucheniya, &Rfc3339) else {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "invalid data_polucheniya".to_string(),
        });
    };

    let storage = PoluchenieSqliteStorage::from_pool((*context.connection_pool).clone());

    match storage.get(rabotnik_id, specodezhda_id, data_polucheniya).await {
        Ok(Some(p)) => Response::OK().with_json(PoluchenieResponse {
            rabotnik_id: p.rabotnik_id.to_string(),
            specodezhda_id: p.specodezhda_id.to_string(),
            data_polucheniya: p.data_polucheniya.to_string(),
            podpis: p.podpis,
        }),
        Ok(None) => Response::NotFound().with_json(ErrorResponse {
            message: "poluchenie not found".to_string(),
        }),
        Err(error) => {
            eprintln!("Ошибка чтения получения: {error}");
            Response::InternalServerError().with_json(ErrorResponse {
                message: "database error".to_string(),
            })
        }
    }
}

//DELETE
pub async fn remove_poluchenie(
    Query(query): Query<PoluchenieQuery>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let Ok(rabotnik_id) = Uuid::parse_str(&query.rabotnik_id) else {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "invalid rabotnik_id".to_string(),
        });
    };
    let Ok(specodezhda_id) = Uuid::parse_str(&query.specodezhda_id) else {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "invalid specodezhda_id".to_string(),
        });
    };
    let Ok(data_polucheniya) = UtcDateTime::parse(&query.data_polucheniya, &Rfc3339) else {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "invalid data_polucheniya".to_string(),
        });
    };

    let storage = PoluchenieSqliteStorage::from_pool((*context.connection_pool).clone());

    let existing = match storage.get(rabotnik_id, specodezhda_id, data_polucheniya).await {
        Ok(Some(p)) => p,
        Ok(None) => {
            return Response::NotFound().with_json(ErrorResponse {
                message: "poluchenie not found".to_string(),
            })
        }
        Err(error) => {
            eprintln!("Ошибка чтения получения: {error}");
            return Response::InternalServerError().with_json(ErrorResponse {
                message: "database error".to_string(),
            });
        }
    };

    match storage.remove(existing).await {
        Ok(()) => Response::OK().with_json(serde_json::json!({"removed": true})),
        Err(error) => {
            eprintln!("Ошибка удаления получения: {error}");
            Response::InternalServerError().with_json(ErrorResponse {
                message: "database error".to_string(),
            })
        }
    }
}
>>>>>>> dcb3657 (7)
