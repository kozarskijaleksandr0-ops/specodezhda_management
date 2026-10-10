<<<<<<< HEAD
use ohkami::{Response, Status};

pub async fn add_rabotnik() -> Response {
    let mut response = Response::new(Status::OK);
    response.set_text("added rabotnik");
    response
}

pub async fn get_rabotnik() -> Response {
    let mut response = Response::new(Status::OK);
    response.set_text("got rabotnik");
    response
}

pub async fn update_rabotnik() -> Response {
    let mut response = Response::new(Status::OK);
    response.set_text("updated rabotnik");
    response
}

pub async fn remove_rabotnik() -> Response {
    let mut response = Response::new(Status::OK);
    response.set_text("removed rabotnik");
    response
}
=======
use ohkami::{Json, Response};
use ohkami::fang::Context;
use ohkami::claw::Path;
use uuid::Uuid;

use crate::context::RequestContext;
use crate::contracts::RabotnikStorage;
use crate::contracts_impl::RabotnikSqliteStorage;
use crate::models::Rabotnik;

use super::payloads::{
    AddRabotnikPayload, UpdateRabotnikPayload, RabotnikResponse, ErrorResponse,
};

//POST
pub async fn add_rabotnik(
    Json(payload): Json<AddRabotnikPayload>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let Ok(ceh_id) = Uuid::parse_str(&payload.ceh_id) else {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "invalid ceh_id".to_string(),
        });
    };

    let ceh_stub = crate::models::Ceh {
        id: ceh_id,
        name: String::new(),
        nachalnik: String::new(),
        created_at: time::UtcDateTime::now(),
        updated_at: None,
        deleted_at: None,
    };

    let rabotnik = Rabotnik::create_new(
        payload.fio,
        payload.dolzhnost,
        payload.skidka,
        &ceh_stub,
    );

    let response_payload = RabotnikResponse {
        id: rabotnik.id.to_string(),
        fio: rabotnik.fio.clone(),
        dolzhnost: rabotnik.dolzhnost.clone(),
        skidka: rabotnik.skidka,
        ceh_id: rabotnik.ceh_id.to_string(),
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

//GET
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
            id: r.id.to_string(),
            fio: r.fio,
            dolzhnost: r.dolzhnost,
            skidka: r.skidka,
            ceh_id: r.ceh_id.to_string(),
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

//PUT
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

    let Ok(ceh_id) = Uuid::parse_str(&payload.ceh_id) else {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "invalid ceh_id".to_string(),
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

    let updated = Rabotnik {
        id: existing.id,
        fio: payload.fio,
        dolzhnost: payload.dolzhnost,
        skidka: payload.skidka,
        ceh_id,
        created_at: existing.created_at,
        updated_at: Some(time::UtcDateTime::now()),
        deleted_at: existing.deleted_at,
    };

    match storage.update(&updated).await {
        Ok(()) => Response::OK().with_json(RabotnikResponse {
            id: updated.id.to_string(),
            fio: updated.fio,
            dolzhnost: updated.dolzhnost,
            skidka: updated.skidka,
            ceh_id: updated.ceh_id.to_string(),
        }),
        Err(error) => {
            eprintln!("Ошибка обновления работника: {error}");
            Response::InternalServerError().with_json(ErrorResponse {
                message: "database error".to_string(),
            })
        }
    }
}

//DELETE
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
>>>>>>> dcb3657 (7)
