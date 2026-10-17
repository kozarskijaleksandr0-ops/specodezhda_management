use ohkami::{Json, Response};
use ohkami::fang::Context;
use ohkami::claw::Path;
use uuid::Uuid;

use crate::context::RequestContext;
use crate::contracts::CehStorage;
use crate::contracts_impl::CehSqliteStorage;
use crate::models::{Ceh, Name};

use super::payloads::{AddCehPayload, UpdateCehPayload, CehResponse, ErrorResponse};

pub async fn add_ceh(
    Json(payload): Json<AddCehPayload>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let ceh = match Ceh::create_new(payload.name, payload.nachalnik) {
        Ok(c) => c,
        Err(e) => {
            return Response::BadRequest().with_json(ErrorResponse {
                message: format!("validation error: {:?}", e),
            });
        }
    };

    let response_payload = CehResponse {
        id: ceh.id.value().to_string(),
        name: ceh.name.value().to_string(),
        nachalnik: ceh.nachalnik.value().to_string(),
    };

    let storage = CehSqliteStorage::from_pool((*context.connection_pool).clone());
    match storage.add(ceh).await {
        Ok(()) => Response::Created().with_json(response_payload),
        Err(error) => {
            eprintln!("Ошибка добавления цеха: {error}");
            Response::InternalServerError().with_json(ErrorResponse {
                message: "database error".to_string(),
            })
        }
    }
}

pub async fn get_ceh(
    Path(id): Path<&str>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let Ok(id) = Uuid::parse_str(id) else {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "invalid ceh id".to_string(),
        });
    };

    let storage = CehSqliteStorage::from_pool((*context.connection_pool).clone());
    match storage.get(id).await {
        Ok(Some(ceh)) => Response::OK().with_json(CehResponse {
            id: ceh.id.value().to_string(),
            name: ceh.name.value().to_string(),
            nachalnik: ceh.nachalnik.value().to_string(),
        }),
        Ok(None) => Response::NotFound().with_json(ErrorResponse {
            message: "ceh not found".to_string(),
        }),
        Err(error) => {
            eprintln!("Ошибка чтения цеха: {error}");
            Response::InternalServerError().with_json(ErrorResponse {
                message: "database error".to_string(),
            })
        }
    }
}

pub async fn update_ceh(
    Path(id): Path<&str>,
    Json(payload): Json<UpdateCehPayload>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let Ok(id) = Uuid::parse_str(id) else {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "invalid ceh id".to_string(),
        });
    };

    let storage = CehSqliteStorage::from_pool((*context.connection_pool).clone());
    let existing = match storage.get(id).await {
        Ok(Some(ceh)) => ceh,
        Ok(None) => {
            return Response::NotFound().with_json(ErrorResponse {
                message: "ceh not found".to_string(),
            })
        }
        Err(error) => {
            eprintln!("Ошибка чтения цеха: {error}");
            return Response::InternalServerError().with_json(ErrorResponse {
                message: "database error".to_string(),
            });
        }
    };

    let name = match Name::new(payload.name) {
        Ok(n) => n,
        Err(e) => {
            return Response::BadRequest().with_json(ErrorResponse {
                message: format!("name error: {:?}", e),
            })
        }
    };
    let nachalnik = match Name::new(payload.nachalnik) {
        Ok(n) => n,
        Err(e) => {
            return Response::BadRequest().with_json(ErrorResponse {
                message: format!("nachalnik error: {:?}", e),
            })
        }
    };

    let updated = Ceh {
        id: existing.id,
        name,
        nachalnik,
        record_lifetime: existing.record_lifetime,
    };

    match storage.update(&updated).await {
        Ok(()) => Response::OK().with_json(CehResponse {
            id: updated.id.value().to_string(),
            name: updated.name.value().to_string(),
            nachalnik: updated.nachalnik.value().to_string(),
        }),
        Err(error) => {
            eprintln!("Ошибка обновления цеха: {error}");
            Response::InternalServerError().with_json(ErrorResponse {
                message: "database error".to_string(),
            })
        }
    }
}

pub async fn remove_ceh(
    Path(id): Path<&str>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let Ok(id) = Uuid::parse_str(id) else {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "invalid ceh id".to_string(),
        });
    };

    let storage = CehSqliteStorage::from_pool((*context.connection_pool).clone());
    let existing = match storage.get(id).await {
        Ok(Some(ceh)) => ceh,
        Ok(None) => {
            return Response::NotFound().with_json(ErrorResponse {
                message: "ceh not found".to_string(),
            })
        }
        Err(error) => {
            eprintln!("Ошибка чтения цеха: {error}");
            return Response::InternalServerError().with_json(ErrorResponse {
                message: "database error".to_string(),
            });
        }
    };

    match storage.remove(existing).await {
        Ok(()) => Response::OK().with_json(serde_json::json!({"removed": true})),
        Err(error) => {
            eprintln!("Ошибка удаления цеха: {error}");
            Response::InternalServerError().with_json(ErrorResponse {
                message: "database error".to_string(),
            })
        }
    }
}