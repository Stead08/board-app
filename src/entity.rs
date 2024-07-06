use crate::value_object::{Content, Email, Name, Password, PostId, Title, UserId};

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub name: Name,
    pub email: Email,
    pub password: Password,
}
#[derive(Debug, Clone)]
pub struct Post {
    pub id: PostId,
    pub user_id: UserId,
    pub title: Title,
    pub content: Content,
}
