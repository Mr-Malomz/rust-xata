use std::env;

use dotenv::dotenv;
use reqwest::{header, Client, Error};

use super::model::{Project, ProjectRequest, ProjectResponse};

pub struct XataService {}

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

    pub async fn create_project(new_project: ProjectRequest) -> Result<ProjectResponse, Error> {
        let url = format!(
            "{}:main/tables/Project/data",
            XataService::env_loader("XATA_DATABASE_URL")
        );

        let mut headers = header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert(
            header::AUTHORIZATION,
            format!("Bearer {}", XataService::env_loader("XATA_API_KEY"))
                .parse()
                .unwrap(),
        );

        let client = XataService::init()
            .post(url)
            .headers(headers)
            .json(&new_project)
            .send()
            .await;

        match client {
            Ok(response) => {
                let json = response.text().await?;
                let created_project: ProjectResponse = serde_json::from_str(json.as_str()).unwrap();

                Ok(created_project)
            }
            Err(error) => Err(error),
        }
    }

    pub async fn get_project(project_id: String) -> Result<Project, Error> {
        let url = format!(
            "{}:main/tables/Project/data/{}",
            XataService::env_loader("XATA_DATABASE_URL"),
            project_id
        );

        let mut headers = header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert(
            header::AUTHORIZATION,
            format!("Bearer {}", XataService::env_loader("XATA_API_KEY"))
                .parse()
                .unwrap(),
        );

        let client = XataService::init().get(url).headers(headers).send().await;

        match client {
            Ok(response) => {
                let json = response.text().await?;
                println!("{}", json.as_str());
                let project_details: Project = serde_json::from_str(json.as_str()).unwrap();

                Ok(project_details)
            }
            Err(error) => Err(error),
        }
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

        let mut headers = header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert(
            header::AUTHORIZATION,
            format!("Bearer {}", XataService::env_loader("XATA_API_KEY"))
                .parse()
                .unwrap(),
        );

        let client = XataService::init()
            .put(url)
            .headers(headers)
            .json(&updated_project)
            .send()
            .await;

        match client {
            Ok(response) => {
                let json = response.text().await?;
                println!("{}", json.as_str());
                let updates: ProjectResponse = serde_json::from_str(json.as_str()).unwrap();

                Ok(updates)
            }
            Err(error) => Err(error),
        }
    }

    pub async fn delete_project(project_id: String) -> Result<String, Error> {
        let url = format!(
            "{}:main/tables/Project/data/{}",
            XataService::env_loader("XATA_DATABASE_URL"),
            project_id
        );

        let mut headers = header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert(
            header::AUTHORIZATION,
            format!("Bearer {}", XataService::env_loader("XATA_API_KEY"))
                .parse()
                .unwrap(),
        );

        let client = XataService::init()
            .delete(url)
            .headers(headers)
            .send()
            .await;

        match client {
            Ok(_) => {
                let json = format!("Project with ID: ${project_id} deleted successfully!!");
                Ok(json)
            }
            Err(error) => Err(error),
        }
    }
}
