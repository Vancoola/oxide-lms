use uuid::Uuid;

pub struct ChatId(Uuid);
impl ChatId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

pub struct ChatMemberId(Uuid);
impl ChatMemberId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

pub struct MemberId(Uuid);
impl MemberId {
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }
}