use uuid::Uuid;

pub enum UserEvent {
    Created {
        user_id: Uuid,
        email: String,
    }
}