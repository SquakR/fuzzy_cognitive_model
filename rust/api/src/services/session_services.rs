use crate::models::{Session, User};
use crate::response::{ServiceResult, ToServiceResult};
use crate::schema::sessions;
use crate::services::{password_services, user_services};
use crate::types::{CredentialsType, DeviceType, OSType, ProductType, SessionType};
use crate::web_socket::{WebSocketAdjustmentRunService, WebSocketModelService};
use crate::{forbidden_error, validation_error};
use diesel::prelude::*;
use diesel::PgConnection;
use ipnetwork::IpNetwork;
use std::env;
use user_agent_parser::{
    Device as UserAgentDevice, Product as UserAgentProduct, UserAgentParser, OS as UserAgentOS,
};

#[macro_export]
macro_rules! authenticate {
    ($conn:expr, $cookies_jar:expr) => {{
        let mut result = Err(rocket::http::Status::Unauthorized);
        let session_id = crate::get_session_id!($cookies_jar);
        if session_id.is_some() {
            let session_id = session_id.unwrap();
            let session = crate::services::session_services::find_session_by_id($conn, session_id);
            if session.is_ok() {
                let session = session.unwrap();
                let user = crate::services::user_services::find_user_by_session($conn, &session);
                if session.is_active {
                    result = Ok((user, session))
                } else {
                    let _sessions = crate::services::session_services::deactivate_all_user_sessions(
                        $conn, user.id,
                    );
                    result = Err(rocket::http::Status::BadRequest);
                }
            }
        }
        result
    }};
}

pub fn sign_in(
    conn: &mut PgConnection,
    credentials: CredentialsType,
    ip_address: &IpNetwork,
    user_agent: &str,
) -> ServiceResult<Session> {
    let user_result = user_services::find_user_by_username(conn, &credentials.username)
        .to_service_result_find(String::from("user_not_found_error"));
    let user = match user_result {
        Ok(user) => user,
        Err(_) => return validation_error!("sign_in_credentials_error"),
    };
    if !password_services::verify_password(&credentials.password, &user.password) {
        return validation_error!("sign_in_credentials_error");
    }
    create_session(conn, user.id, ip_address, user_agent).to_service_result()
}

pub async fn sign_out(
    conn: &mut PgConnection,
    model_service: WebSocketModelService,
    adjustment_run_service: WebSocketAdjustmentRunService,
    session_ids: &[i32],
) -> QueryResult<Vec<Session>> {
    let sessions = deactivate_user_sessions(conn, session_ids)?;
    model_service.disconnect_sessions(session_ids).await;
    adjustment_run_service
        .disconnect_sessions(&session_ids)
        .await;
    Ok(sessions)
}

pub fn create_session(
    conn: &mut PgConnection,
    user_id: i32,
    ip_address: &IpNetwork,
    user_agent: &str,
) -> QueryResult<Session> {
    diesel::insert_into(sessions::table)
        .values((
            sessions::user_id.eq(user_id),
            sessions::ip_address.eq(ip_address),
            sessions::user_agent.eq(user_agent),
        ))
        .get_result::<Session>(conn)
}

pub fn get_user_active_sessions(
    conn: &mut PgConnection,
    user_id: i32,
) -> QueryResult<Vec<Session>> {
    sessions::table
        .filter(sessions::user_id.eq(user_id))
        .filter(sessions::is_active.eq(true))
        .order(sessions::created_at.asc())
        .get_results::<Session>(conn)
}

pub fn find_session_by_id(conn: &mut PgConnection, session_id: i32) -> QueryResult<Session> {
    sessions::table.find(session_id).first::<Session>(conn)
}

pub fn deactivate_all_user_sessions(
    conn: &mut PgConnection,
    user_id: i32,
) -> QueryResult<Vec<Session>> {
    diesel::update(sessions::table)
        .filter(sessions::user_id.eq(user_id))
        .filter(sessions::is_active.eq(true))
        .set(sessions::is_active.eq(false))
        .get_results::<Session>(conn)
}

pub fn check_user_sessions(
    conn: &mut PgConnection,
    user: &User,
    session_ids: &[i32],
) -> ServiceResult<()> {
    for session_id in session_ids {
        let session = find_session_by_id(conn, *session_id)
            .to_service_result_find(String::from("session_not_found_error"))?;
        if !session.is_active {
            deactivate_all_user_sessions(conn, user.id).to_service_result()?;
            return validation_error!("session_is_not_active_error");
        }
        if session.user_id != user.id {
            return forbidden_error!("other_user_session_forbidden_error");
        }
    }
    Ok(())
}

pub fn deactivate_user_sessions(
    conn: &mut PgConnection,
    session_ids: &[i32],
) -> QueryResult<Vec<Session>> {
    diesel::update(sessions::table.filter(sessions::id.eq_any(session_ids)))
        .set(sessions::is_active.eq(false))
        .get_results::<Session>(conn)
}

impl From<UserAgentDevice<'_>> for DeviceType {
    fn from(user_agent_device: UserAgentDevice) -> Self {
        Self {
            name: user_agent_device
                .name
                .and_then(|name| Some(name.into_owned())),
            brand: user_agent_device
                .brand
                .and_then(|brand| Some(brand.into_owned())),
            model: user_agent_device
                .model
                .and_then(|model| Some(model.into_owned())),
        }
    }
}

impl From<UserAgentOS<'_>> for OSType {
    fn from(user_agent_os: UserAgentOS<'_>) -> Self {
        Self {
            name: user_agent_os.name.and_then(|name| Some(name.into_owned())),
            major: user_agent_os
                .major
                .and_then(|major| Some(major.into_owned())),
            minor: user_agent_os
                .minor
                .and_then(|minor| Some(minor.into_owned())),
            patch: user_agent_os
                .patch
                .and_then(|patch| Some(patch.into_owned())),
            patch_minor: user_agent_os
                .patch_minor
                .and_then(|patch_minor| Some(patch_minor.into_owned())),
        }
    }
}

impl From<UserAgentProduct<'_>> for ProductType {
    fn from(user_agent_product: UserAgentProduct<'_>) -> Self {
        Self {
            name: user_agent_product
                .name
                .and_then(|name| Some(name.into_owned())),
            major: user_agent_product
                .major
                .and_then(|major| Some(major.into_owned())),
            minor: user_agent_product
                .minor
                .and_then(|minor| Some(minor.into_owned())),
            patch: user_agent_product
                .patch
                .and_then(|patch| Some(patch.into_owned())),
        }
    }
}

pub fn session_to_session_type(session: &Session, active_session_id: i32) -> SessionType {
    let ua_parser =
        UserAgentParser::from_path(env::current_dir().unwrap().join("user_agent_regexes.yaml"))
            .unwrap();
    SessionType {
        id: session.id,
        is_current: session.id == active_session_id,
        created_at: session.created_at,
        ip_address: format!("{}", session.ip_address.ip()),
        device: DeviceType::from(ua_parser.parse_device(&session.user_agent)),
        os: OSType::from(ua_parser.parse_os(&session.user_agent)),
        product: ProductType::from(ua_parser.parse_product(&session.user_agent)),
    }
}
