use super::*;
use crate::http::host::dto::{CreateHostRequest, GetAllHostRequest, UpdateHostMetadataRequest};
use async_trait::async_trait;
use chrono::Utc;
use domain::project::Project;
use std::sync::{Arc, Mutex};

#[derive(Clone, Default)]
struct FakeHostRepository {
    hosts: Arc<Mutex<Vec<Host>>>,
    inserted: Arc<Mutex<usize>>,
    updated: Arc<Mutex<usize>>,
    deleted: Arc<Mutex<Vec<String>>>,
}

impl FakeHostRepository {
    fn with_hosts(hosts: Vec<Host>) -> Self {
        Self {
            hosts: Arc::new(Mutex::new(hosts)),
            ..Default::default()
        }
    }

    fn insert_count(&self) -> usize {
        *self.inserted.lock().unwrap()
    }

    fn update_count(&self) -> usize {
        *self.updated.lock().unwrap()
    }

    fn deleted_ids(&self) -> Vec<String> {
        self.deleted.lock().unwrap().clone()
    }
}

#[async_trait]
impl HostRepository for FakeHostRepository {
    async fn insert(&self, host: &Host) -> anyhow::Result<()> {
        self.hosts.lock().unwrap().push(host.clone());
        *self.inserted.lock().unwrap() += 1;
        Ok(())
    }

    async fn get_all(&self, query: PageRequest<HostPosition>) -> anyhow::Result<Vec<Host>> {
        let hosts = self.hosts.lock().unwrap();
        let mut result = hosts
            .iter()
            .filter(|host| {
                query
                    .q
                    .as_ref()
                    .map(|q| host.name.to_lowercase().contains(&q.to_lowercase()))
                    .unwrap_or(true)
            })
            .filter(|host| {
                query
                    .status
                    .as_ref()
                    .map(|status| host.status.to_string() == *status)
                    .unwrap_or(true)
            })
            .cloned()
            .collect::<Vec<_>>();

        result.sort_by_key(|host| host.id);
        if let Some(position) = query.after {
            result.retain(|host| host.id > position.id);
        }
        result.truncate(query.limit as usize);
        Ok(result)
    }

    async fn get_by_id(&self, id: &str) -> anyhow::Result<Option<Host>> {
        Ok(self
            .hosts
            .lock()
            .unwrap()
            .iter()
            .find(|host| host.id.to_string() == id)
            .cloned())
    }

    async fn get_by(&self, column: &str, value: &str) -> anyhow::Result<Option<Host>> {
        Ok(self
            .hosts
            .lock()
            .unwrap()
            .iter()
            .find(|host| match column {
                "name" => host.name == value,
                "status" => host.status.to_string() == value,
                "type" => host._type.to_string() == value,
                _ => false,
            })
            .cloned())
    }

    async fn update(&self, host: &Host) -> anyhow::Result<()> {
        let mut hosts = self.hosts.lock().unwrap();
        let stored = hosts
            .iter_mut()
            .find(|stored| stored.id == host.id)
            .ok_or_else(|| anyhow::anyhow!("host not found"))?;
        *stored = host.clone();
        *self.updated.lock().unwrap() += 1;
        Ok(())
    }

    async fn delete(&self, id: &str) -> anyhow::Result<()> {
        self.hosts
            .lock()
            .unwrap()
            .retain(|host| host.id.to_string() != id);
        self.deleted.lock().unwrap().push(id.to_owned());
        Ok(())
    }
}

#[derive(Clone, Default)]
struct FakeProjectRepository {
    projects: Arc<Mutex<Vec<Project>>>,
}

#[async_trait]
impl ProjectRepository for FakeProjectRepository {
    async fn get_by_host(
        &self,
        host_id: &uuid::Uuid,
        after: Option<uuid::Uuid>,
        limit: i64,
    ) -> anyhow::Result<Vec<Project>> {
        let mut projects = self
            .projects
            .lock()
            .unwrap()
            .iter()
            .filter(|project| project.host_id == *host_id)
            .cloned()
            .collect::<Vec<_>>();
        projects.sort_by_key(|project| project.id);
        if let Some(after) = after {
            projects.retain(|project| project.id > after);
        }
        projects.truncate(limit as usize);
        Ok(projects)
    }
}

fn service(host_repository: FakeHostRepository) -> HostService {
    HostService::new(
        Arc::new(host_repository),
        Arc::new(FakeProjectRepository::default()),
    )
}

fn host(name: &str) -> Host {
    Host {
        id: uuid::Uuid::new_v4(),
        name: name.to_owned(),
        _type: domain::host::HostType::Local,
        docker_endpoint: "unix:///var/run/docker.sock".to_owned(),
        status: domain::host::HostStatus::Unknown,
        last_seen_at: None,
        created_at: Utc::now().naive_utc(),
    }
}

fn create_request(name: &str) -> CreateHostRequest {
    CreateHostRequest {
        name: name.to_owned(),
        _type: "local".to_owned(),
        docker_endpoint: "unix:///var/run/docker.sock".to_owned(),
    }
}

#[tokio::test]
async fn save_creates_valid_host() {
    let repository = FakeHostRepository::default();
    let service = service(repository.clone());

    let response = service.save(create_request("local-docker")).await.unwrap();

    assert_eq!(response.name, "local-docker");
    assert_eq!(response.project_count, 0);
    assert_eq!(repository.insert_count(), 1);
}

#[tokio::test]
async fn save_rejects_duplicate_name_before_insert() {
    let repository = FakeHostRepository::with_hosts(vec![host("local-docker")]);
    let service = service(repository.clone());

    let error = service
        .save(create_request("local-docker"))
        .await
        .unwrap_err();

    assert!(matches!(error, HttpError::BadRequest(message) if message == "duplicate name"));
    assert_eq!(repository.insert_count(), 0);
}

#[tokio::test]
async fn save_rejects_invalid_request_without_repository_call() {
    let repository = FakeHostRepository::default();
    let service = service(repository.clone());
    let request = CreateHostRequest {
        name: "x".to_owned(),
        _type: "invalid".to_owned(),
        docker_endpoint: "endpoint".to_owned(),
    };

    assert!(matches!(
        service.save(request).await,
        Err(HttpError::Validation(_))
    ));
    assert_eq!(repository.insert_count(), 0);
}

#[tokio::test]
async fn get_by_id_returns_not_found_for_unknown_host() {
    let service = service(FakeHostRepository::default());

    let error = service.get_by_id(uuid::Uuid::new_v4()).await.unwrap_err();

    assert!(matches!(error, HttpError::NotFound(message) if message == "host not found"));
}

#[tokio::test]
async fn update_metadata_updates_only_supplied_fields() {
    let existing = host("old-name");
    let host_id = existing.id;
    let repository = FakeHostRepository::with_hosts(vec![existing]);
    let service = service(repository.clone());

    let response = service
        .update_metadata(
            &host_id,
            UpdateHostMetadataRequest {
                name: Some("new-name".to_owned()),
                _type: None,
                docker_endpoint: None,
            },
        )
        .await
        .unwrap();

    assert_eq!(response.data.name, "new-name");
    assert_eq!(response.data.docker_endpoint, "unix:///var/run/docker.sock");
    assert_eq!(repository.update_count(), 1);
}

#[tokio::test]
async fn delete_removes_existing_host() {
    let existing = host("local-docker");
    let host_id = existing.id;
    let repository = FakeHostRepository::with_hosts(vec![existing]);
    let service = service(repository.clone());

    service.delete(&host_id).await.unwrap();

    assert_eq!(repository.deleted_ids(), vec![host_id.to_string()]);
    assert!(service.get_by_id(host_id).await.is_err());
}

#[tokio::test]
async fn get_all_applies_limit_and_reports_next_cursor() {
    let repository = FakeHostRepository::with_hosts(vec![
        host("host-one"),
        host("host-two"),
        host("host-three"),
    ]);
    let service = service(repository);

    let (hosts, metadata) = service
        .get_all(GetAllHostRequest {
            cursor: None,
            limit: Some(2),
            q: None,
            status: None,
        })
        .await
        .unwrap();

    assert_eq!(hosts.len(), 2);
    assert!(metadata.has_more);
    assert!(metadata.next_cursor.is_some());
}
