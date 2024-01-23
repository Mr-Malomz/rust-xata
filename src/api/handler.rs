use actix_web::{
    delete, get, post, put,
    web::{Json, Path},
    HttpResponse,
};
use reqwest::StatusCode;

use super::{
    model::{APIErrorResponse, APIResponse, Project, ProjectRequest, ProjectResponse},
    xata_service::XataService,
};

#[post("/project")]
pub async fn create_project_handler(new_project: Json<ProjectRequest>) -> HttpResponse {
    let project_details = XataService::create_project(new_project.to_owned()).await;

    match project_details {
        Ok(data) => HttpResponse::Accepted().json(APIResponse::<ProjectResponse> {
            status: StatusCode::CREATED.as_u16(),
            message: "success".to_string(),
            data: Some(data),
        }),
        Err(error) => HttpResponse::InternalServerError().json(APIErrorResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: "failure".to_string(),
            data: Some(error.to_string()),
        }),
    }
}

#[get("/project/{id}")]
pub async fn get_project_handler(path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().json(APIErrorResponse {
            status: StatusCode::BAD_REQUEST.as_u16(),
            message: "failure".to_string(),
            data: Some("invalid ID".to_string()),
        });
    };

    let project_details = XataService::get_project(id).await;

    match project_details {
        Ok(data) => HttpResponse::Accepted().json(APIResponse::<Project> {
            status: StatusCode::OK.as_u16(),
            message: "success".to_string(),
            data: Some(data),
        }),
        Err(error) => HttpResponse::InternalServerError().json(APIErrorResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: "failure".to_string(),
            data: Some(error.to_string()),
        }),
    }
}

#[put("/project/{id}")]
pub async fn update_project_handler(
    updated_project: Json<ProjectRequest>,
    path: Path<String>,
) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().json(APIErrorResponse {
            status: StatusCode::BAD_REQUEST.as_u16(),
            message: "failure".to_string(),
            data: Some("invalid ID".to_string()),
        });
    };

    let project_details = XataService::update_project(updated_project.to_owned(), id).await;

    match project_details {
        Ok(data) => HttpResponse::Accepted().json(APIResponse::<ProjectResponse> {
            status: StatusCode::OK.as_u16(),
            message: "success".to_string(),
            data: Some(data),
        }),
        Err(error) => HttpResponse::InternalServerError().json(APIErrorResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: "failure".to_string(),
            data: Some(error.to_string()),
        }),
    }
}

#[delete("/project/{id}")]
pub async fn delete_project_handler(path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().json(APIErrorResponse {
            status: StatusCode::BAD_REQUEST.as_u16(),
            message: "failure".to_string(),
            data: Some("invalid ID".to_string()),
        });
    };
    let project_details = XataService::delete_project(id).await;

    match project_details {
        Ok(data) => HttpResponse::Accepted().json(APIResponse::<String> {
            status: StatusCode::ACCEPTED.as_u16(),
            message: "success".to_string(),
            data: Some(data),
        }),
        Err(error) => HttpResponse::InternalServerError().json(APIErrorResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: "failure".to_string(),
            data: Some(error.to_string()),
        }),
    }
}
