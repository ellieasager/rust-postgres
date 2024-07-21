use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct Message {
    pub id: Uuid,
    pub content: String,
}

impl Serialize for Message {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Message", 2)?;
        s.serialize_field("id", &self.id.to_string())?;
        s.serialize_field("content", &self.content)?;
        s.end()
    }
}

#[derive(Serialize)]
pub struct ListResponse {
    pub messages: Vec<Message>,
}
