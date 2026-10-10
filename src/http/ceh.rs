<<<<<<< HEAD
use ohkami::{Response, Status};

pub async fn add_ceh() -> Response {
    let mut response = Response::new(Status::OK);
    response.set_text("added ceh");
    response
}

pub async fn get_ceh() -> Response {
    let mut response = Response::new(Status::OK);
    response.set_text("got ceh");
    response
}

pub async fn update_ceh() -> Response {
    let mut response = Response::new(Status::OK);
    response.set_text("updated ceh");
    response
}

pub async fn remove_ceh() -> Response {
    let mut response = Response::new(Status::OK);
    response.set_text("removed ceh");
    response
}
=======
use ohkami::{Json, Response, Status};
use ohkami::fang::Context;
use ohkami::claw::Path;
use uuid::Uuid;

use crate::context::RequestContext;
use crate::contracts::CehStorage;
use crate::contracts_impl::CehSqliteStorage;
use crate::models::Ceh;

use super::payloads::{AddCehPayload, UpdateCehPayload, CehResponse, ErrorResponse};

//POST
pub async fn add_ceh(
    Json(payload): Json<AddCehPayload>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let ceh = Ceh::create_new(payload.name, payload.nachalnik);
    let storage = CehSqliteStorage::from_pool((*context.connection_pool).clone());

    match storage.add(ceh).await {
        Ok(()) => Response::Created().with_json(CehResponse {
            id: String::new(), // заполним ниже, см. примечание
            name: String::new(),
            nachalnik: String::new(),
        }),
        Err(error) => {
            eprintln!("Ошибка добавления цеха: {error}");
            Response::InternalServerError().with_json(ErrorResponse {
                message: "database error".to_string(),
            })
        }
    }
}

//GET
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
            id: ceh.id.to_string(),
            name: ceh.name,
            nachalnik: ceh.nachalnik,
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

//PUT
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

    let updated = Ceh {
        id: existing.id,
        name: payload.name,
        nachalnik: payload.nachalnik,
        created_at: existing.created_at,
        updated_at: Some(time::UtcDateTime::now()),
        deleted_at: existing.deleted_at,
    };

    match storage.update(&updated).await {
        Ok(()) => Response::OK().with_json(CehResponse {
            id: updated.id.to_string(),
            name: updated.name,
            nachalnik: updated.nachalnik,
        }),
        Err(error) => {
            eprintln!("Ошибка обновления цеха: {error}");
            Response::InternalServerError().with_json(ErrorResponse {
                message: "database error".to_string(),
            })
        }
    }
}

//DELETE
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
>>>>>>> dcb3657 (7)
