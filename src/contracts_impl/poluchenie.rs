use std::path::PathBuf;

use async_sqlite::{Pool, PoolBuilder};
use rusqlite::{params_from_iter, types::Value};
use time::UtcDateTime;
use uuid::Uuid;

use crate::contracts::PoluchenieStorage;
use crate::models::value_objects::{EntityId, Name, RecordLifetime};
use crate::models::Poluchenie;

pub struct PoluchenieSqliteStorage {
    connection: Pool,
}

impl PoluchenieSqliteStorage {
    pub async fn new(database_url: &str) -> Self {
        let path_buf = PathBuf::from(database_url);
        let builder = PoolBuilder::new()
            .path(path_buf)
            .journal_mode(async_sqlite::JournalMode::Wal)
            .num_conns(10);
        let pool: Pool = builder.open().await.expect("Database creation error");
        Self { connection: pool }
    }

    pub fn from_pool(connection: Pool) -> Self {
        Self { connection }
    }
}

impl PoluchenieStorage for PoluchenieSqliteStorage {
    async fn add(&self, poluchenie: Poluchenie) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "
            INSERT INTO poluchenie (rabotnik_id, specodezhda_id, data_polucheniya, podpis, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5)
        ";
        let parameters: Vec<Value> = vec![
            poluchenie.rabotnik_id.value().into(),
            poluchenie.specodezhda_id.value().into(),
            poluchenie.data_polucheniya.to_string().into(),
            poluchenie.podpis.value().to_string().into(),
            poluchenie.record_lifetime.created_at().to_string().into(),
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
        rabotnik_id: Uuid,
        specodezhda_id: Uuid,
        data_polucheniya: UtcDateTime,
    ) -> Result<Option<Poluchenie>, Box<dyn std::error::Error>> {
        const QUERY: &str = "
            SELECT rabotnik_id, specodezhda_id, data_polucheniya, podpis, created_at
            FROM poluchenie
            WHERE rabotnik_id = ?1 AND specodezhda_id = ?2 AND data_polucheniya = ?3
        ";
        let parameters: Vec<Value> = vec![
            rabotnik_id.into(),
            specodezhda_id.into(),
            data_polucheniya.to_string().into(),
        ];

        let row = self
            .connection
            .conn(move |connection| {
                let result = connection.query_row(QUERY, params_from_iter(parameters), |row| {
                    let rabotnik_id: String = row.get(0)?;
                    let specodezhda_id: String = row.get(1)?;
                    let data_polucheniya: String = row.get(2)?;
                    let podpis: String = row.get(3)?;
                    let created_at: String = row.get(4)?;
                    Ok((rabotnik_id, specodezhda_id, data_polucheniya, podpis, created_at))
                });
                Ok(result)
            })
            .await?;

        match row {
            Ok((rabotnik_id, specodezhda_id, data_polucheniya, podpis, created_at)) => {
                use time::format_description::well_known::Rfc3339;
                let rabotnik_id = EntityId::create(Uuid::parse_str(&rabotnik_id)?)?;
                let specodezhda_id = EntityId::create(Uuid::parse_str(&specodezhda_id)?)?;
                let podpis = Name::new(podpis)?;
                let record_lifetime = RecordLifetime::create(
                    UtcDateTime::parse(&created_at, &Rfc3339)?,
                    None,
                    None,
                )?;
                Ok(Some(Poluchenie {
                    rabotnik_id,
                    specodezhda_id,
                    data_polucheniya: UtcDateTime::parse(&data_polucheniya, &Rfc3339)?,
                    podpis,
                    record_lifetime,
                }))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn remove(&self, poluchenie: Poluchenie) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "
            DELETE FROM poluchenie
            WHERE rabotnik_id = ?1 AND specodezhda_id = ?2 AND data_polucheniya = ?3
        ";
        let parameters: Vec<Value> = vec![
            poluchenie.rabotnik_id.value().into(),
            poluchenie.specodezhda_id.value().into(),
            poluchenie.data_polucheniya.to_string().into(),
        ];
        self.connection
            .conn(move |connection| {
                connection.execute(QUERY, params_from_iter(parameters))?;
                Ok(())
            })
            .await?;
        Ok(())
    }

    async fn update(&self, poluchenie: &Poluchenie) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "
            UPDATE poluchenie
            SET podpis = ?1
            WHERE rabotnik_id = ?2 AND specodezhda_id = ?3 AND data_polucheniya = ?4
        ";
        let parameters: Vec<Value> = vec![
            poluchenie.podpis.value().to_string().into(),
            poluchenie.rabotnik_id.value().into(),
            poluchenie.specodezhda_id.value().into(),
            poluchenie.data_polucheniya.to_string().into(),
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