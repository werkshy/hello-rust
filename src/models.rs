use std::time::SystemTime;

#[derive(Serialize, Queryable)]
pub struct Thing {
    pub id: i64,
    pub name: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

