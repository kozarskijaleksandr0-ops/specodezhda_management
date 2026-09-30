use std::future::Future;
use uuid::Uuid;

use crate::models::Ceh;

pub trait CehStorage {
    fn add(&self, ceh: Ceh) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>>;

    fn get(
        &self,
        uuid: Uuid,
    ) -> impl Future<Output = Result<Option<Ceh>, Box<dyn std::error::Error>>>;

    fn remove(&self, ceh: Ceh) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>>;

    fn update(&self, ceh: &Ceh) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>>;
}
