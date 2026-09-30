use time::UtcDateTime;
use uuid::Uuid;

#[doc = "Цех. Поля:
        - идентификатор цеха
        - наименование цеха
        - Ф.И.О. начальника цеха
        - дата создания
        - дата обновления
        - дата удаления"]
pub struct Ceh {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) nachalnik: String,
    pub(crate) created_at: UtcDateTime,
    pub(crate) updated_at: Option<UtcDateTime>,
    pub(crate) deleted_at: Option<UtcDateTime>,
}

impl Ceh {
    pub fn create(
        id: Uuid,
        name: String,
        nachalnik: String,
        created_at: UtcDateTime,
        updated_at: Option<UtcDateTime>,
        deleted_at: Option<UtcDateTime>,
    ) -> Self {
        Self {
            id,
            name,
            nachalnik,
            created_at,
            updated_at,
            deleted_at,
        }
    }

    pub fn create_new(name: String, nachalnik: String) -> Self {
        let id = Uuid::new_v4();
        let created_at = UtcDateTime::now();
        Self::create(id, name, nachalnik, created_at, None, None)
    }
}
