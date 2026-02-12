mod object;

use uuid::Uuid;
use crate::chat::object::{ChatId, ChatMemberId, MemberId};

pub struct Chat {
    id: ChatId,
}

pub struct ChatMember {
    id: ChatMemberId,
    chat_id: ChatId,
    user_id: MemberId
}