use time::UtcDateTime;
use uuid::Uuid;

#[doc = "Сущность спецодежда:
        Поля сущности:
        - идентификатор
        - вид спецодежды
        - срок носки
        - стоимость единицы
        - дата создания
        - дата удаления
        - дата обновления"]
pub struct Specodezhda {
    pub(crate) id: Uuid,
    pub(crate) vid: String,
    pub(crate) srok_noski: i64,
    pub(crate) stoimost: f64,
    pub(crate) created_at: UtcDateTime,
    pub(crate) updated_at: Option<UtcDateTime>,
    pub(crate) deleted_at: Option<UtcDateTime>,
}

impl Specodezhda {
    pub fn create(
        id: Uuid,
        vid: String,
        srok_noski: i64,
        stoimost: f64,
        created_at: UtcDateTime,
        updated_at: Option<UtcDateTime>,
        deleted_at: Option<UtcDateTime>,
    ) -> Self {
        Self {
            id,
            vid,
            srok_noski,
            stoimost,
            created_at,
            updated_at,
            deleted_at,
        }
    }

    pub fn create_new(vid: String, srok_noski: i64, stoimost: f64) -> Self {
        let id = Uuid::new_v4();
        let created_at = UtcDateTime::now();
        Self::create(id, vid, srok_noski, stoimost, created_at, None, None)
    }
}
