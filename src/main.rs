mod db;
mod models;

#[tokio::main]
async fn main() {
    let conn = db::connect().unwrap();
    db::init::init_db(&conn).unwrap();
    
}