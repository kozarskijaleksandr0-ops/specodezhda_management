use std::path::PathBuf;

use async_sqlite::{Pool, PoolBuilder};
use rusqlite::{params_from_iter, types::Value};

use crate::contracts::RabotnikStorage;
use crate::models::Rabotnik;

pub struct RabotnikSqliteStorage {
    connection: Pool,
}

impl RabotnikSqliteStorage {
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

impl RabotnikStorage for RabotnikSqliteStorage {
    async fn add(&self, rabotnik: Rabotnik) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "
            INSERT INTO rabotnik (id, fio, dolzhnost, skidka, ceh_id, created_at, updated_at, deleted_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
        ";
        let parameters: Vec<Value> = vec![
            rabotnik.id.into(),
            rabotnik.fio.into(),
            rabotnik.dolzhnost.into(),
            (rabotnik.skidka).into(),
            rabotnik.ceh_id.into(),
            rabotnik.created_at.to_string().into(),
            rabotnik.updated_at.map(|dt| dt.to_string()).into(),
            rabotnik.deleted_at.map(|dt| dt.to_string()).into(),
        ];
        self.connection
            .conn(move |connection| {
                connection.execute(QUERY, params_from_iter(parameters))?;
                Ok(())
            })
            .await?;
        Ok(())
    }

    async fn get(&self, uuid: uuid::Uuid) -> Result<Option<Rabotnik>, Box<dyn std::error::Error>> {
        const QUERY: &str = "
            SELECT id, fio, dolzhnost, skidka, ceh_id, created_at, updated_at, deleted_at
            FROM rabotnik WHERE id = ?1
        ";
        let parameters: Vec<Value> = vec![uuid.into()];
        let row = self
            .connection
            .conn(move |connection| {
                let result = connection.query_row(QUERY, params_from_iter(parameters), |row| {
                    let id: uuid::Uuid = row.get(0)?;
                    let fio: String = row.get(1)?;
                    let dolzhnost: String = row.get(2)?;
                    let skidka: f64 = row.get(3)?;
                    let ceh_id: uuid::Uuid = row.get(4)?;
                    let created_at: String = row.get(5)?;
                    let updated_at: Option<String> = row.get(6)?;
                    let deleted_at: Option<String> = row.get(7)?;
                    Ok((
                        id, fio, dolzhnost, skidka, ceh_id, created_at, updated_at, deleted_at,
                    ))
                });
                Ok(result)
            })
            .await?;

        match row {
            Ok((id, fio, dolzhnost, skidka, ceh_id, created_at, updated_at, deleted_at)) => {
                use time::UtcDateTime;
                use time::format_description::well_known::Rfc3339;
                Ok(Some(Rabotnik {
                    id,
                    fio,
                    dolzhnost,
                    skidka,
                    ceh_id,
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

    async fn remove(&self, rabotnik: Rabotnik) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "DELETE FROM rabotnik WHERE id = ?1";
        let parameters: Vec<Value> = vec![rabotnik.id.into()];
        self.connection
            .conn(move |connection| {
                connection.execute(QUERY, params_from_iter(parameters))?;
                Ok(())
            })
            .await?;
        Ok(())
    }

    async fn update(&self, rabotnik: &Rabotnik) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "
            UPDATE rabotnik
            SET fio = ?1, dolzhnost = ?2, skidka = ?3, ceh_id = ?4, updated_at = ?5, deleted_at = ?6
            WHERE id = ?7
        ";
        let parameters: Vec<Value> = vec![
            rabotnik.fio.clone().into(),
            rabotnik.dolzhnost.clone().into(),
            (rabotnik.skidka).into(),
            rabotnik.ceh_id.into(),
            rabotnik.updated_at.map(|dt| dt.to_string()).into(),
            rabotnik.deleted_at.map(|dt| dt.to_string()).into(),
            rabotnik.id.into(),
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
