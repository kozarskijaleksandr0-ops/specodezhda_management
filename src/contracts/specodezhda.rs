use std::future::Future;
use uuid::Uuid;

use crate::models::Specodezhda;

pub trait SpecodezhdaStorage {
    fn add(
        &self,
        specodezhda: Specodezhda,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>>;

    fn get(
        &self,
        uuid: Uuid,
    ) -> impl Future<Output = Result<Option<Specodezhda>, Box<dyn std::error::Error>>>;

    fn remove(
        &self,
        specodezhda: Specodezhda,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>>;

    fn update(
        &self,
        specodezhda: &Specodezhda,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>>;
}
