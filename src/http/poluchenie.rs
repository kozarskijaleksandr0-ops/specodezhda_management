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
use crate::models::{EntityId, Poluchenie};

use super::payloads::{AddPolucheniePayload, PoluchenieResponse, ErrorResponse};

#[derive(Deserialize)]
pub struct PoluchenieQuery {
    pub rabotnik_id: String,
    pub specodezhda_id: String,
    pub data_polucheniya: String,
}

pub async fn add_poluchenie(
    Json(payload): Json<AddPolucheniePayload>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let Ok(rabotnik_uuid) = Uuid::parse_str(&payload.rabotnik_id) else {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "invalid rabotnik_id".to_string(),
        });
    };
    let Ok(specodezhda_uuid) = Uuid::parse_str(&payload.specodezhda_id) else {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "invalid specodezhda_id".to_string(),
        });
    };
    let Ok(data_polucheniya) = UtcDateTime::parse(&payload.data_polucheniya, &Rfc3339) else {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "invalid data_polucheniya (expect RFC3339)".to_string(),
        });
    };

    let rabotnik_id = match EntityId::create(rabotnik_uuid) {
        Ok(id) => id,
        Err(e) => {
            return Response::BadRequest().with_json(ErrorResponse {
                message: format!("rabotnik_id error: {:?}", e),
            })
        }
    };
    let specodezhda_id = match EntityId::create(specodezhda_uuid) {
        Ok(id) => id,
        Err(e) => {
            return Response::BadRequest().with_json(ErrorResponse {
                message: format!("specodezhda_id error: {:?}", e),
            })
        }
    };

    let poluchenie = match Poluchenie::create_new(
        rabotnik_id,
        specodezhda_id,
        data_polucheniya,
        payload.podpis,
    ) {
        Ok(p) => p,
        Err(e) => {
            return Response::BadRequest().with_json(ErrorResponse {
                message: format!("validation error: {:?}", e),
            });
        }
    };

    let response_payload = PoluchenieResponse {
        rabotnik_id: poluchenie.rabotnik_id.value().to_string(),
        specodezhda_id: poluchenie.specodezhda_id.value().to_string(),
        data_polucheniya: poluchenie.data_polucheniya.to_string(),
        podpis: poluchenie.podpis.value().to_string(),
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
            rabotnik_id: p.rabotnik_id.value().to_string(),
            specodezhda_id: p.specodezhda_id.value().to_string(),
            data_polucheniya: p.data_polucheniya.to_string(),
            podpis: p.podpis.value().to_string(),
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