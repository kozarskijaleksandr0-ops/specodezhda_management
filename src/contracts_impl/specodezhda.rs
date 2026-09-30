use std::path::PathBuf;

use async_sqlite::{Pool, PoolBuilder};
use rusqlite::{params_from_iter, types::Value};

use crate::contracts::SpecodezhdaStorage;
use crate::models::Specodezhda;

pub struct SpecodezhdaSqliteStorage {
    connection: Pool,
}

impl SpecodezhdaSqliteStorage {
    pub async fn new(database_url: &str) -> Self {
        let path_buf: PathBuf = PathBuf::from(database_url);
        let builder: PoolBuilder = PoolBuilder::new()
            .path(path_buf)
            .journal_mode(async_sqlite::JournalMode::Wal)
            .num_conns(10);
        let connection: Pool = builder.open().await.expect("Database creation error");
        Self { connection }
    }

    pub fn from_pool(connection: Pool) -> Self {
        Self { connection }
    }
}

impl SpecodezhdaStorage for SpecodezhdaSqliteStorage {
    async fn add(&self, specodezhda: Specodezhda) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "
            INSERT INTO specodezhda (id, vid, srok_noski, stoimost, created_at, updated_at, deleted_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
        ";

        let parameters: Vec<Value> = vec![
            specodezhda.id.into(),
            specodezhda.vid.into(),
            (specodezhda.srok_noski as i64).into(),
            (specodezhda.stoimost as f64).into(),
            specodezhda.created_at.to_string().into(),
            specodezhda.updated_at.map(|dt| dt.to_string()).into(),
            specodezhda.deleted_at.map(|dt| dt.to_string()).into(),
        ];

        self.connection
            .conn(move |connection| {
                connection.execute(QUERY, params_from_iter(parameters))?;
                Ok(())
            })
            .await?;

        Ok(())
    }

    async fn get(
        &self,
        uuid: uuid::Uuid,
    ) -> Result<Option<Specodezhda>, Box<dyn std::error::Error>> {
        const QUERY: &str = "
            SELECT id, vid, srok_noski, stoimost, created_at, updated_at, deleted_at
            FROM specodezhda
            WHERE id = ?1
        ";

        let parameters: Vec<Value> = vec![uuid.into()];

        let row = self
            .connection
            .conn(move |connection| {
                let result = connection.query_row(QUERY, params_from_iter(parameters), |row| {
                    let id: uuid::Uuid = row.get(0)?;
                    let vid: String = row.get(1)?;
                    let srok_noski: i64 = row.get(2)?;
                    let stoimost: f64 = row.get(3)?;
                    let created_at: String = row.get(4)?;
                    let updated_at: Option<String> = row.get(5)?;
                    let deleted_at: Option<String> = row.get(6)?;
                    Ok((
                        id, vid, srok_noski, stoimost, created_at, updated_at, deleted_at,
                    ))
                });
                Ok(result)
            })
            .await?;

        match row {
            Ok((id, vid, srok_noski, stoimost, created_at, updated_at, deleted_at)) => {
                use time::UtcDateTime;
                use time::format_description::well_known::Rfc3339;

                let created_at = UtcDateTime::parse(&created_at, &Rfc3339)?;
                let updated_at = match updated_at {
                    Some(s) => Some(UtcDateTime::parse(&s, &Rfc3339)?),
                    None => None,
                };
                let deleted_at = match deleted_at {
                    Some(s) => Some(UtcDateTime::parse(&s, &Rfc3339)?),
                    None => None,
                };

                Ok(Some(Specodezhda {
                    id,
                    vid,
                    srok_noski,
                    stoimost,
                    created_at,
                    updated_at,
                    deleted_at,
                }))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn remove(&self, specodezhda: Specodezhda) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "DELETE FROM specodezhda WHERE id = ?1";
        let parameters: Vec<Value> = vec![specodezhda.id.into()];

        self.connection
            .conn(move |connection| {
                connection.execute(QUERY, params_from_iter(parameters))?;
                Ok(())
            })
            .await?;

        Ok(())
    }

    async fn update(&self, specodezhda: &Specodezhda) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "
            UPDATE specodezhda
            SET vid = ?1, srok_noski = ?2, stoimost = ?3, updated_at = ?4, deleted_at = ?5
            WHERE id = ?6
        ";

        let parameters: Vec<Value> = vec![
            specodezhda.vid.clone().into(),
            (specodezhda.srok_noski).into(),
            (specodezhda.stoimost).into(),
            specodezhda.updated_at.map(|dt| dt.to_string()).into(),
            specodezhda.deleted_at.map(|dt| dt.to_string()).into(),
            specodezhda.id.into(),
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
