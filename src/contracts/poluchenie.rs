use std::future::Future;
use time::UtcDateTime;
use uuid::Uuid;

use crate::models::Poluchenie;

pub trait PoluchenieStorage {
    fn add(
        &self,
        poluchenie: Poluchenie,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>>;

    fn get(
        &self,
        rabotnik_id: Uuid,
        specodezhda_id: Uuid,
        data_polucheniya: UtcDateTime,
    ) -> impl Future<Output = Result<Option<Poluchenie>, Box<dyn std::error::Error>>>;

    fn remove(
        &self,
        poluchenie: Poluchenie,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>>;

    fn update(
        &self,
        poluchenie: &Poluchenie,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>>;
}
