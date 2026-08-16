use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug, Clone, Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "status", rename_all = "lowercase")]
pub enum HostStatus {
    Online,
    Offline,
    Unknown,
}

impl Display for HostStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HostStatus::Online => write!(f, "online"),
            HostStatus::Offline => write!(f, "offline"),
            HostStatus::Unknown => write!(f, "unknown"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "type", rename_all = "lowercase")]
pub enum HostType {
    Local,
    Remote,
}

impl Display for HostType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HostType::Local => write!(f, "local"),
            HostType::Remote => write!(f, "remote"),
        }
    }
}

impl Into<HostType> for String {
    fn into(self) -> HostType {
        match self.as_str() {
            "local" => HostType::Local,
            "remote" => HostType::Remote,
            _ => panic!("invalid host type"),
        }
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct Host {
    pub id: uuid::Uuid,
    pub name: String,
    #[sqlx(rename = "type")]
    pub _type: HostType,
    pub docker_endpoint: String,
    pub status: HostStatus,
    pub last_seen_at: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
}
