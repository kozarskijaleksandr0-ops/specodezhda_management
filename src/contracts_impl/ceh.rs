use std::path::PathBuf;

use async_sqlite::{Pool, PoolBuilder};
use rusqlite::{params_from_iter, types::Value};

use crate::contracts::CehStorage;
use crate::models::Ceh;

pub struct CehSqliteStorage {
    connection: Pool,
}

impl CehSqliteStorage {
    pub async fn new(database_url: &str) -> Self {
        let path_buf = PathBuf::from(database_url);
        let builder = PoolBuilder::new()
            .path(path_buf)
            .journal_mode(async_sqlite::JournalMode::Wal)
            .num_conns(10);
        let connection = builder.open().await.expect("Database creation error");
        Self { connection }
    }

    pub fn from_pool(connection: Pool) -> Self {
        Self { connection }
    }
}

impl CehStorage for CehSqliteStorage {
    async fn add(&self, ceh: Ceh) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "
            INSERT INTO ceh (id, name, nachalnik, created_at, updated_at, deleted_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        ";

        let parameters: Vec<Value> = vec![
            ceh.id.into(),
            ceh.name.into(),
            ceh.nachalnik.into(),
            ceh.created_at.to_string().into(),
            ceh.updated_at.map(|dt| dt.to_string()).into(),
            ceh.deleted_at.map(|dt| dt.to_string()).into(),
        ];

        self.connection
            .conn(move |connection| {
                connection.execute(QUERY, params_from_iter(parameters))?;
                Ok(())
            })
            .await?;
        Ok(())
    }

    async fn get(&self, uuid: uuid::Uuid) -> Result<Option<Ceh>, Box<dyn std::error::Error>> {
        const QUERY: &str = "
            SELECT id, name, nachalnik, created_at, updated_at, deleted_at
            FROM ceh WHERE id = ?1
        ";
        let parameters: Vec<Value> = vec![uuid.into()];

        let row = self
            .connection
            .conn(move |connection| {
                let result = connection.query_row(QUERY, params_from_iter(parameters), |row| {
                    let id: uuid::Uuid = row.get(0)?;
                    let name: String = row.get(1)?;
                    let nachalnik: String = row.get(2)?;
                    let created_at: String = row.get(3)?;
                    let updated_at: Option<String> = row.get(4)?;
                    let deleted_at: Option<String> = row.get(5)?;
                    Ok((id, name, nachalnik, created_at, updated_at, deleted_at))
                });
                Ok(result)
            })
            .await?;

        match row {
            Ok((id, name, nachalnik, created_at, updated_at, deleted_at)) => {
                use time::UtcDateTime;
                use time::format_description::well_known::Rfc3339;
                Ok(Some(Ceh {
                    id,
                    name,
                    nachalnik,
                    created_at: UtcDateTime::parse(&created_at, &Rfc3339)?,
                    updated_at: match updated_at {
                        Some(s) => Some(UtcDateTime::parse(&s, &Rfc3339)?),
                        None => None,
                    },
                    deleted_at: match deleted_at {
                        Some(s) => Some(UtcDateTime::parse(&s, &Rfc3339)?),
                        None => None,
                    },
                }))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn remove(&self, ceh: Ceh) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "DELETE FROM ceh WHERE id = ?1";
        let parameters: Vec<Value> = vec![ceh.id.into()];
        self.connection
            .conn(move |connection| {
                connection.execute(QUERY, params_from_iter(parameters))?;
                Ok(())
            })
            .await?;
        Ok(())
    }

    async fn update(&self, ceh: &Ceh) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "
            UPDATE ceh
            SET name = ?1, nachalnik = ?2, updated_at = ?3, deleted_at = ?4
            WHERE id = ?5
        ";
        let parameters: Vec<Value> = vec![
            ceh.name.clone().into(),
            ceh.nachalnik.clone().into(),
            ceh.updated_at.map(|dt| dt.to_string()).into(),
            ceh.deleted_at.map(|dt| dt.to_string()).into(),
            ceh.id.into(),
        ];
        self.connection
            .conn(move |connection| {
                connection.execute(QUERY, params_from_iter(parameters))?;
                Ok(())
            })
            .await?;
        Ok(())
    }
}
