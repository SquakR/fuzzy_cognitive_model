use crate::db;
use crate::models::User;
use crate::request::UserLocale;
use crate::response::PathResult;
use crate::services::project_services;
use crate::types::{
    CancelInvitationType, InvitationResponseType, InvitationType, ProjectInChangeType,
    ProjectInCreateType, ProjectOutType,
};
use rocket::serde::json::Json;
use rocket_okapi::openapi;

/// Create new project
#[openapi(tag = "projects")]
#[post("/project", format = "json", data = "<project_in>")]
pub fn create_project(
    project_in: Json<ProjectInCreateType>,
    user: User,
    locale: UserLocale,
) -> PathResult<Json<ProjectOutType>, UserLocale> {
    let connection = &mut db::establish_connection();
    let project = match project_services::create_project(connection, &user, project_in.into_inner())
    {
        Ok(project) => project,
        Err(app_error) => return PathResult::new(Err(app_error), locale),
    };
    PathResult::new(
        Ok(Json(ProjectOutType::from((project, connection)))),
        locale,
    )
}

/// Change project
#[openapi(tag = "projects")]
#[put("/project", format = "json", data = "<project_in>")]
pub fn change_project(
    project_in: Json<ProjectInChangeType>,
    user: User,
    locale: UserLocale,
) -> PathResult<Json<ProjectOutType>, UserLocale> {
    let connection = &mut db::establish_connection();
    let project = match project_services::change_project(connection, &user, project_in.into_inner())
    {
        Ok(project) => project,
        Err(app_error) => return PathResult::new(Err(app_error), locale),
    };
    PathResult::new(
        Ok(Json(ProjectOutType::from((project, connection)))),
        locale,
    )
}

/// Invite user to project
#[openapi(tag = "projects")]
#[post("/invite_user", format = "json", data = "<invitation>")]
pub fn invite_user(
    invitation: Json<InvitationType>,
    user: User,
    locale: UserLocale,
) -> PathResult<(), UserLocale> {
    let connection = &mut db::establish_connection();
    if let Err(app_error) =
        project_services::invite_user(connection, &user, invitation.into_inner())
    {
        return PathResult::new(Err(app_error), locale);
    }
    PathResult::new(Ok(()), locale)
}

/// Cancel user invitation to project
#[openapi(tag = "projects")]
#[post("/cancel_invitation", format = "json", data = "<cancel_invitation>")]
pub fn cancel_invitation(
    cancel_invitation: Json<CancelInvitationType>,
    user: User,
    locale: UserLocale,
) -> PathResult<(), UserLocale> {
    let connection = &mut db::establish_connection();
    if let Err(app_error) =
        project_services::cancel_invitation(connection, &user, cancel_invitation.into_inner())
    {
        return PathResult::new(Err(app_error), locale);
    }
    PathResult::new(Ok(()), locale)
}

/// Respond to invitation to project
#[openapi(tag = "projects")]
#[post(
    "/respond_to_invitation",
    format = "json",
    data = "<invitation_response>"
)]
pub fn respond_to_invitation(
    invitation_response: Json<InvitationResponseType>,
    user: User,
    locale: UserLocale,
) -> PathResult<(), UserLocale> {
    let connection = &mut db::establish_connection();
    if let Err(app_error) =
        project_services::respond_to_invitation(connection, &user, invitation_response.into_inner())
    {
        return PathResult::new(Err(app_error), locale);
    }
    PathResult::new(Ok(()), locale)
}

/// Leave project
#[openapi(tag = "projects")]
#[post("/leave_project/<project_id>")]
pub fn leave_project(
    project_id: i32,
    user: User,
    locale: UserLocale,
) -> PathResult<(), UserLocale> {
    let connection = &mut db::establish_connection();
    if let Err(app_error) = project_services::leave_project(connection, &user, project_id) {
        return PathResult::new(Err(app_error), locale);
    }
    PathResult::new(Ok(()), locale)
}

/// Delete project
#[openapi(tag = "projects")]
#[delete("/project/<project_id>")]
pub fn delete_project(
    project_id: i32,
    user: User,
    locale: UserLocale,
) -> PathResult<(), UserLocale> {
    let connection = &mut db::establish_connection();
    if let Err(app_error) = project_services::delete_project(connection, &user, project_id) {
        return PathResult::new(Err(app_error), locale);
    }
    PathResult::new(Ok(()), locale)
}
