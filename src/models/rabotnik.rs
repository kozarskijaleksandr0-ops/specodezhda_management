use time::UtcDateTime;
use uuid::Uuid;

use super::ceh::Ceh;

#[doc = "Работник. Поля:
        - идентификатор
        - Ф.И.О. работника
        - должность
        - скидка на спецодежду
        - идентификатор цеха
        - дата создания
        - дата обновления
        - дата удаления"]
pub struct Rabotnik {
    pub(crate) id: Uuid,
    pub(crate) fio: String,
    pub(crate) dolzhnost: String,
    pub(crate) skidka: f64,
    pub(crate) ceh_id: Uuid,
    pub(crate) created_at: UtcDateTime,
    pub(crate) updated_at: Option<UtcDateTime>,
    pub(crate) deleted_at: Option<UtcDateTime>,
}

impl Rabotnik {
    pub fn create(
        id: Uuid,
        fio: String,
        dolzhnost: String,
        skidka: f64,
        ceh: &Ceh,
        created_at: UtcDateTime,
        updated_at: Option<UtcDateTime>,
        deleted_at: Option<UtcDateTime>,
    ) -> Self {
        Self {
            id,
            fio,
            dolzhnost,
            skidka,
            ceh_id: ceh.id,
            created_at,
            updated_at,
            deleted_at,
        }
    }

    pub fn create_new(fio: String, dolzhnost: String, skidka: f64, ceh: &Ceh) -> Self {
        let id = Uuid::new_v4();
        let created_at = UtcDateTime::now();
        Self::create(id, fio, dolzhnost, skidka, ceh, created_at, None, None)
    }
}
