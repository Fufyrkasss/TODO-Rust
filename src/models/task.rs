use super::user::User;

pub struct Task{
    pub id: i32,
    pub name: String,
    pub content: String,
    pub creator: User,
    pub deadline: String,
}