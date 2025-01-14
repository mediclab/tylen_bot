use sea_orm::Set;
use teloxide::types::{Message, User};

impl From<User> for super::entity::users::ActiveModel {
    fn from(value: User) -> Self {
        super::entity::users::ActiveModel {
            user_id: Set(value.id.0 as i64),
            username: Set(value.username),
            firstname: Set(value.first_name),
            lastname: Set(value.last_name),
            ..Default::default()
        }
    }
}

impl From<Message> for super::entity::memes::ActiveModel {
    fn from(value: Message) -> Self {
        let photo = value.photo().unwrap();

        super::entity::memes::ActiveModel {
            user_id: Set(value.from.as_ref().unwrap().id.0 as i64),
            photo_id: Set(photo[0].file.id.clone()),
            ..Default::default()
        }
    }
}
