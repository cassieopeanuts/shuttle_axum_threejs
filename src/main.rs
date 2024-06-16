use axum::{
    routing::get_service,
    Router,
};
use tower_http::services::ServeDir;
use shuttle_axum::ShuttleAxum;

#[shuttle_runtime::main]
async fn main() -> ShuttleAxum {
    let service = get_service(ServeDir::new("build"));

    let router = Router::new().nest_service("/", service);

    Ok(router.into())
}
