use std::future::Future;
use uuid::Uuid;

use crate::models::Rabotnik;

pub trait RabotnikStorage {
    fn add(
        &self,
        rabotnik: Rabotnik,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>>;

    fn get(
        &self,
        uuid: Uuid,
    ) -> impl Future<Output = Result<Option<Rabotnik>, Box<dyn std::error::Error>>>;

    fn remove(
        &self,
        rabotnik: Rabotnik,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>>;

    fn update(
        &self,
        rabotnik: &Rabotnik,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>>;
}
