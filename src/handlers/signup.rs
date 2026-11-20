use axum::response::Html;

pub async fn sign_up_handler() -> Html<&'static str> {
    Html(include_str!("../../static/signup/sign_up.html"))
}