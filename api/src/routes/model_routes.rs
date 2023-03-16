use crate::db;
use crate::models::User;
use crate::request::UserLocale;
use crate::response::PathResult;
use crate::types::UserOutType;
use crate::web_socket::WebSocketProjectService;
use rocket::serde::json::Json;
use rocket_okapi::openapi;

/// Get active users
#[openapi(tag = "model")]
#[get("/project/<project_id>/active_users")]
pub fn get_active_users(
    project_id: i32,
    user: User,
    locale: UserLocale,
    web_socket_project_service: WebSocketProjectService,
) -> PathResult<Json<Vec<UserOutType>>, UserLocale> {
    let conn = &mut db::establish_connection();
    let users = match web_socket_project_service.get_active_users(conn, &user, project_id) {
        Ok(users) => users
            .into_iter()
            .map(UserOutType::from)
            .collect::<Vec<UserOutType>>(),
        Err(app_error) => return PathResult::new(Err(app_error), locale),
    };
    PathResult::new(Ok(Json(users)), locale)
}
