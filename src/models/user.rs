pub enum Role {
    Admin,
    User,
    None,
}

pub struct User{
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: Role,
}