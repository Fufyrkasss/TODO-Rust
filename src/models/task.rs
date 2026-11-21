use super::user::User;

pub enum Status {
    Done,
    UnDone,
    InProgress,
    None,
}

pub struct Task{
    pub id: i32,
    pub name: String,
    pub content: String,
    pub creator: User,
    pub status: Status,
    pub deadline: String,
}