use std::sync::Arc;

use axum::Router;
use axum::routing::{delete, get, post};

use crate::controller::api;
use crate::middleware;
use crate::model::global::AppState;

pub fn api(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/ping", get(api::ping))
        .route("/add_user", post(api::add_user))
        .route("/update-user",post(api::update_user))
        .route("/get_user/:specify_user", get(api::get_user))
        .route("/del_user/:users", delete(api::del_user))
        .route("/users/:page_num/:page_size/:sort", get(api::get_users))
        .route("/search_user/:con/:page_num/:page_size", get(api::search_user))
        .route("/add-menu",post(api::add_menu))
        .route("/get-menu",get(api::get_menu))
        .layer(axum::middleware::from_fn_with_state(state, middleware::auth)) // 鉴权中间件
}
