use time::UtcDateTime;
use uuid::Uuid;

use super::rabotnik::Rabotnik;
use super::specodezhda::Specodezhda;

#[doc = "Получение. Поля:
        - идентификатор работника
        - идентификатор спецодежды
        - дата получения
        - подпись
        - дата создания"]
pub struct Poluchenie {
    pub(crate) rabotnik_id: Uuid,
    pub(crate) specodezhda_id: Uuid,
    pub(crate) data_polucheniya: UtcDateTime,
    pub(crate) podpis: String,
    pub(crate) created_at: UtcDateTime,
}

impl Poluchenie {
    pub fn create(
        rabotnik: &Rabotnik,
        specodezhda: &Specodezhda,
        data_polucheniya: UtcDateTime,
        podpis: String,
        created_at: UtcDateTime,
    ) -> Self {
        Self {
            rabotnik_id: rabotnik.id,
            specodezhda_id: specodezhda.id,
            data_polucheniya,
            podpis,
            created_at,
        }
    }

    pub fn create_new(
        rabotnik: &Rabotnik,
        specodezhda: &Specodezhda,
        data_polucheniya: UtcDateTime,
        podpis: String,
    ) -> Self {
        let created_at = UtcDateTime::now();
        Self::create(rabotnik, specodezhda, data_polucheniya, podpis, created_at)
    }
}
