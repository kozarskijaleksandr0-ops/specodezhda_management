use ohkami::{Json, Response};
use ohkami::fang::Context;
use ohkami::claw::Path;
use uuid::Uuid;

use crate::context::RequestContext;
use crate::contracts::RabotnikStorage;
use crate::contracts_impl::RabotnikSqliteStorage;
use crate::models::{EntityId, Name, Rabotnik, Skidka};

use super::payloads::{AddRabotnikPayload, UpdateRabotnikPayload, RabotnikResponse, ErrorResponse};

pub async fn add_rabotnik(
    Json(payload): Json<AddRabotnikPayload>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let Ok(ceh_uuid) = Uuid::parse_str(&payload.ceh_id) else {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "invalid ceh_id".to_string(),
        });
    };
    let ceh_id = match EntityId::create(ceh_uuid) {
        Ok(id) => id,
        Err(e) => {
            return Response::BadRequest().with_json(ErrorResponse {
                message: format!("ceh_id error: {:?}", e),
            })
        }
    };

    let rabotnik = match Rabotnik::create_new(
        payload.fio,
        payload.dolzhnost,
        payload.skidka,
        ceh_id,
    ) {
        Ok(r) => r,
        Err(e) => {
            return Response::BadRequest().with_json(ErrorResponse {
                message: format!("validation error: {:?}", e),
            });
        }
    };

    let response_payload = RabotnikResponse {
        id: rabotnik.id.value().to_string(),
        fio: rabotnik.fio.value().to_string(),
        dolzhnost: rabotnik.dolzhnost.value().to_string(),
        skidka: rabotnik.skidka.value(),
        ceh_id: rabotnik.ceh_id.value().to_string(),
    };

    let storage = RabotnikSqliteStorage::from_pool((*context.connection_pool).clone());
    match storage.add(rabotnik).await {
        Ok(()) => Response::Created().with_json(response_payload),
        Err(error) => {
            eprintln!("Ошибка добавления работника: {error}");
            Response::InternalServerError().with_json(ErrorResponse {
                message: "database error".to_string(),
            })
        }
    }
}

pub async fn get_rabotnik(
    Path(id): Path<&str>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let Ok(id) = Uuid::parse_str(id) else {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "invalid rabotnik id".to_string(),
        });
    };

    let storage = RabotnikSqliteStorage::from_pool((*context.connection_pool).clone());
    match storage.get(id).await {
        Ok(Some(r)) => Response::OK().with_json(RabotnikResponse {
            id: r.id.value().to_string(),
            fio: r.fio.value().to_string(),
            dolzhnost: r.dolzhnost.value().to_string(),
            skidka: r.skidka.value(),
            ceh_id: r.ceh_id.value().to_string(),
        }),
        Ok(None) => Response::NotFound().with_json(ErrorResponse {
            message: "rabotnik not found".to_string(),
        }),
        Err(error) => {
            eprintln!("Ошибка чтения работника: {error}");
            Response::InternalServerError().with_json(ErrorResponse {
                message: "database error".to_string(),
            })
        }
    }
}

pub async fn update_rabotnik(
    Path(id): Path<&str>,
    Json(payload): Json<UpdateRabotnikPayload>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let Ok(id) = Uuid::parse_str(id) else {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "invalid rabotnik id".to_string(),
        });
    };
    let Ok(ceh_uuid) = Uuid::parse_str(&payload.ceh_id) else {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "invalid ceh_id".to_string(),
        });
    };
    let ceh_id = match EntityId::create(ceh_uuid) {
        Ok(id) => id,
        Err(e) => {
            return Response::BadRequest().with_json(ErrorResponse {
                message: format!("ceh_id error: {:?}", e),
            })
        }
    };

    let storage = RabotnikSqliteStorage::from_pool((*context.connection_pool).clone());
    let existing = match storage.get(id).await {
        Ok(Some(r)) => r,
        Ok(None) => {
            return Response::NotFound().with_json(ErrorResponse {
                message: "rabotnik not found".to_string(),
            })
        }
        Err(error) => {
            eprintln!("Ошибка чтения работника: {error}");
            return Response::InternalServerError().with_json(ErrorResponse {
                message: "database error".to_string(),
            });
        }
    };

    let fio = match Name::new(payload.fio) {
        Ok(n) => n,
        Err(e) => {
            return Response::BadRequest().with_json(ErrorResponse {
                message: format!("fio error: {:?}", e),
            })
        }
    };
    let dolzhnost = match Name::new(payload.dolzhnost) {
        Ok(n) => n,
        Err(e) => {
            return Response::BadRequest().with_json(ErrorResponse {
                message: format!("dolzhnost error: {:?}", e),
            })
        }
    };
    let skidka = match Skidka::new(payload.skidka) {
        Ok(s) => s,
        Err(e) => {
            return Response::BadRequest().with_json(ErrorResponse {
                message: format!("skidka error: {:?}", e),
            })
        }
    };

    let updated = Rabotnik {
        id: existing.id,
        fio,
        dolzhnost,
        skidka,
        ceh_id,
        record_lifetime: existing.record_lifetime,
    };

    match storage.update(&updated).await {
        Ok(()) => Response::OK().with_json(RabotnikResponse {
            id: updated.id.value().to_string(),
            fio: updated.fio.value().to_string(),
            dolzhnost: updated.dolzhnost.value().to_string(),
            skidka: updated.skidka.value(),
            ceh_id: updated.ceh_id.value().to_string(),
        }),
        Err(error) => {
            eprintln!("Ошибка обновления работника: {error}");
            Response::InternalServerError().with_json(ErrorResponse {
                message: "database error".to_string(),
            })
        }
    }
}

pub async fn remove_rabotnik(
    Path(id): Path<&str>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let Ok(id) = Uuid::parse_str(id) else {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "invalid rabotnik id".to_string(),
        });
    };

    let storage = RabotnikSqliteStorage::from_pool((*context.connection_pool).clone());
    let existing = match storage.get(id).await {
        Ok(Some(r)) => r,
        Ok(None) => {
            return Response::NotFound().with_json(ErrorResponse {
                message: "rabotnik not found".to_string(),
            })
        }
        Err(error) => {
            eprintln!("Ошибка чтения работника: {error}");
            return Response::InternalServerError().with_json(ErrorResponse {
                message: "database error".to_string(),
            });
        }
    };

    match storage.remove(existing).await {
        Ok(()) => Response::OK().with_json(serde_json::json!({"removed": true})),
        Err(error) => {
            eprintln!("Ошибка удаления работника: {error}");
            Response::InternalServerError().with_json(ErrorResponse {
                message: "database error".to_string(),
            })
        }
    }
}