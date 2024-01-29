use std::env;

use dotenv::dotenv;
use reqwest::{header, Client, Error, RequestBuilder};

use super::model::{Project, ProjectRequest, ProjectResponse};

pub struct XataService;

impl XataService {
    fn env_loader(key: &str) -> String {
        dotenv().ok();
        match env::var(key) {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        }
    }

    fn init() -> Client {
        Client::new()
    }

    fn create_headers() -> header::HeaderMap {
        let mut headers = header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert(
            header::AUTHORIZATION,
            format!("Bearer {}", XataService::env_loader("XATA_API_KEY"))
                .parse()
                .unwrap(),
        );
        headers
    }

    async fn send_request(builder: RequestBuilder) -> Result<reqwest::Response, reqwest::Error> {
        builder.send().await
    }

    async fn handle_response<T>(response: reqwest::Response) -> Result<T, reqwest::Error>
    where
        T: serde::de::DeserializeOwned,
    {
        let json = response.text().await?;
        Ok(serde_json::from_str(json.as_str()).unwrap())
    }

    pub async fn create_project(new_project: ProjectRequest) -> Result<ProjectResponse, Error> {
        let url = format!(
            "{}:main/tables/Project/data",
            XataService::env_loader("XATA_DATABASE_URL")
        );

        let client = XataService::init()
            .post(url)
            .headers(XataService::create_headers())
            .json(&new_project);

        let response = XataService::send_request(client).await?;
        XataService::handle_response(response).await
    }

    pub async fn get_project(project_id: String) -> Result<Project, Error> {
        let url = format!(
            "{}:main/tables/Project/data/{}",
            XataService::env_loader("XATA_DATABASE_URL"),
            project_id
        );

        let client = XataService::init()
            .get(url)
            .headers(XataService::create_headers());

        let response = XataService::send_request(client).await?;
        XataService::handle_response(response).await
    }

    pub async fn update_project(
        updated_project: ProjectRequest,
        project_id: String,
    ) -> Result<ProjectResponse, Error> {
        let url = format!(
            "{}:main/tables/Project/data/{}",
            XataService::env_loader("XATA_DATABASE_URL"),
            project_id
        );

        let client = XataService::init()
            .put(url)
            .headers(XataService::create_headers())
            .json(&updated_project);

        let response = XataService::send_request(client).await?;
        XataService::handle_response(response).await
    }

    pub async fn delete_project(project_id: String) -> Result<String, Error> {
        let url = format!(
            "{}:main/tables/Project/data/{}",
            XataService::env_loader("XATA_DATABASE_URL"),
            project_id
        );

        let client = XataService::init()
            .delete(url)
            .headers(XataService::create_headers());

        let _response = XataService::send_request(client).await?;
        Ok(format!(
            "Project with ID: {} deleted successfully!!",
            project_id
        ))
    }
}
