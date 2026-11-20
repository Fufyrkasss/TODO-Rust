use axum::response::Html;

pub async fn login_handler() -> Html<&'static str> {
    Html(include_str!("../../static/login/login.html"))
}