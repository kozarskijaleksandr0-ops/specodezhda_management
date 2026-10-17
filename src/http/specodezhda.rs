use ohkami::{Json, Response};
use ohkami::fang::Context;
use ohkami::claw::Path;
use uuid::Uuid;

use crate::context::RequestContext;
use crate::contracts::SpecodezhdaStorage;
use crate::contracts_impl::SpecodezhdaSqliteStorage;
use crate::models::{Name, Price, Specodezhda, SrokNoski};

use super::payloads::{
    AddSpecodezhdaPayload, UpdateSpecodezhdaPayload, SpecodezhdaResponse, ErrorResponse,
};

pub async fn add_specodezhda(
    Json(payload): Json<AddSpecodezhdaPayload>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let specodezhda = match Specodezhda::create_new(
        payload.vid,
        payload.srok_noski,
        payload.stoimost,
    ) {
        Ok(s) => s,
        Err(e) => {
            return Response::BadRequest().with_json(ErrorResponse {
                message: format!("validation error: {:?}", e),
            });
        }
    };

    let response_payload = SpecodezhdaResponse {
        id: specodezhda.id.value().to_string(),
        vid: specodezhda.vid.value().to_string(),
        srok_noski: specodezhda.srok_noski.value(),
        stoimost: specodezhda.stoimost.value(),
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
            id: s.id.value().to_string(),
            vid: s.vid.value().to_string(),
            srok_noski: s.srok_noski.value(),
            stoimost: s.stoimost.value(),
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

    let vid = match Name::new(payload.vid) {
        Ok(n) => n,
        Err(e) => {
            return Response::BadRequest().with_json(ErrorResponse {
                message: format!("vid error: {:?}", e),
            })
        }
    };
    let srok_noski = match SrokNoski::new(payload.srok_noski) {
        Ok(s) => s,
        Err(e) => {
            return Response::BadRequest().with_json(ErrorResponse {
                message: format!("srok_noski error: {:?}", e),
            })
        }
    };
    let stoimost = match Price::new(payload.stoimost) {
        Ok(p) => p,
        Err(e) => {
            return Response::BadRequest().with_json(ErrorResponse {
                message: format!("stoimost error: {:?}", e),
            })
        }
    };

    let updated = Specodezhda {
        id: existing.id,
        vid,
        srok_noski,
        stoimost,
        record_lifetime: existing.record_lifetime,
    };

    match storage.update(&updated).await {
        Ok(()) => Response::OK().with_json(SpecodezhdaResponse {
            id: updated.id.value().to_string(),
            vid: updated.vid.value().to_string(),
            srok_noski: updated.srok_noski.value(),
            stoimost: updated.stoimost.value(),
        }),
        Err(error) => {
            eprintln!("Ошибка обновления спецодежды: {error}");
            Response::InternalServerError().with_json(ErrorResponse {
                message: "database error".to_string(),
            })
        }
    }
}

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