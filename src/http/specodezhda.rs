<<<<<<< HEAD
use ohkami::{Response, Status};

pub async fn add_specodezhda() -> Response {
    let mut response = Response::new(Status::OK);
    response.set_text("added specodezhda");
    response
}

pub async fn get_specodezhda() -> Response {
    let mut response = Response::new(Status::OK);
    response.set_text("got specodezhda");
    response
}

pub async fn update_specodezhda() -> Response {
    let mut response = Response::new(Status::OK);
    response.set_text("updated specodezhda");
    response
}

pub async fn remove_specodezhda() -> Response {
    let mut response = Response::new(Status::OK);
    response.set_text("removed specodezhda");
    response
}
=======
use ohkami::{Json, Response};
use ohkami::fang::Context;
use ohkami::claw::Path;
use uuid::Uuid;

use crate::context::RequestContext;
use crate::contracts::SpecodezhdaStorage;
use crate::contracts_impl::SpecodezhdaSqliteStorage;
use crate::models::Specodezhda;

use super::payloads::{
    AddSpecodezhdaPayload, UpdateSpecodezhdaPayload, SpecodezhdaResponse, ErrorResponse,
};

//POST
pub async fn add_specodezhda(
    Json(payload): Json<AddSpecodezhdaPayload>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let specodezhda =
        Specodezhda::create_new(payload.vid, payload.srok_noski, payload.stoimost);

    let response_payload = SpecodezhdaResponse {
        id: specodezhda.id.to_string(),
        vid: specodezhda.vid.clone(),
        srok_noski: specodezhda.srok_noski,
        stoimost: specodezhda.stoimost,
    };

    let storage = SpecodezhdaSqliteStorage::from_pool((*context.connection_pool).clone());

    match storage.add(specodezhda).await {
        Ok(()) => Response::Created().with_json(response_payload),
        Err(error) => {
            eprintln!("Ошибка добавления спецодежды: {error}");
            Response::InternalServerError().with_json(ErrorResponse {
                message: "database error".to_string(),
            })
        }
    }
}

//GET
pub async fn get_specodezhda(
    Path(id): Path<&str>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let Ok(id) = Uuid::parse_str(id) else {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "invalid specodezhda id".to_string(),
        });
    };

    let storage = SpecodezhdaSqliteStorage::from_pool((*context.connection_pool).clone());

    match storage.get(id).await {
        Ok(Some(s)) => Response::OK().with_json(SpecodezhdaResponse {
            id: s.id.to_string(),
            vid: s.vid,
            srok_noski: s.srok_noski,
            stoimost: s.stoimost,
        }),
        Ok(None) => Response::NotFound().with_json(ErrorResponse {
            message: "specodezhda not found".to_string(),
        }),
        Err(error) => {
            eprintln!("Ошибка чтения спецодежды: {error}");
            Response::InternalServerError().with_json(ErrorResponse {
                message: "database error".to_string(),
            })
        }
    }
}

//PUT
pub async fn update_specodezhda(
    Path(id): Path<&str>,
    Json(payload): Json<UpdateSpecodezhdaPayload>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let Ok(id) = Uuid::parse_str(id) else {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "invalid specodezhda id".to_string(),
        });
    };

    let storage = SpecodezhdaSqliteStorage::from_pool((*context.connection_pool).clone());

    let existing = match storage.get(id).await {
        Ok(Some(s)) => s,
        Ok(None) => {
            return Response::NotFound().with_json(ErrorResponse {
                message: "specodezhda not found".to_string(),
            })
        }
        Err(error) => {
            eprintln!("Ошибка чтения спецодежды: {error}");
            return Response::InternalServerError().with_json(ErrorResponse {
                message: "database error".to_string(),
            });
        }
    };

    let updated = Specodezhda {
        id: existing.id,
        vid: payload.vid,
        srok_noski: payload.srok_noski,
        stoimost: payload.stoimost,
        created_at: existing.created_at,
        updated_at: Some(time::UtcDateTime::now()),
        deleted_at: existing.deleted_at,
    };

    match storage.update(&updated).await {
        Ok(()) => Response::OK().with_json(SpecodezhdaResponse {
            id: updated.id.to_string(),
            vid: updated.vid,
            srok_noski: updated.srok_noski,
            stoimost: updated.stoimost,
        }),
        Err(error) => {
            eprintln!("Ошибка обновления спецодежды: {error}");
            Response::InternalServerError().with_json(ErrorResponse {
                message: "database error".to_string(),
            })
        }
    }
}

//DELETE
pub async fn remove_specodezhda(
    Path(id): Path<&str>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let Ok(id) = Uuid::parse_str(id) else {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "invalid specodezhda id".to_string(),
        });
    };

    let storage = SpecodezhdaSqliteStorage::from_pool((*context.connection_pool).clone());

    let existing = match storage.get(id).await {
        Ok(Some(s)) => s,
        Ok(None) => {
            return Response::NotFound().with_json(ErrorResponse {
                message: "specodezhda not found".to_string(),
            })
        }
        Err(error) => {
            eprintln!("Ошибка чтения спецодежды: {error}");
            return Response::InternalServerError().with_json(ErrorResponse {
                message: "database error".to_string(),
            });
        }
    };

    match storage.remove(existing).await {
        Ok(()) => Response::OK().with_json(serde_json::json!({"removed": true})),
        Err(error) => {
            eprintln!("Ошибка удаления спецодежды: {error}");
            Response::InternalServerError().with_json(ErrorResponse {
                message: "database error".to_string(),
            })
        }
    }
}
>>>>>>> dcb3657 (7)
