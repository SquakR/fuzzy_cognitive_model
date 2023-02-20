use crate::models::{Session, User};
use crate::response::{AppError, ServiceResult, ToServiceResult};
use crate::schema::sessions;
use crate::types::{DeviceType, OSType, ProductType, SessionType};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use ipnetwork::IpNetwork;
use std::env;
use user_agent_parser::{
    Device as UserAgentDevice, Product as UserAgentProduct, UserAgentParser, OS as UserAgentOS,
};

pub fn create_session(
    connection: &mut PgConnection,
    user_id: i32,
    ip_address: &IpNetwork,
    user_agent: &str,
) -> ServiceResult<Session> {
    diesel::insert_into(sessions::table)
        .values((
            sessions::user_id.eq(user_id),
            sessions::ip_address.eq(ip_address),
            sessions::user_agent.eq(user_agent),
        ))
        .get_result::<Session>(connection)
        .to_service_result()
}

pub fn get_user_active_sessions(
    connection: &mut PgConnection,
    user_id: i32,
) -> ServiceResult<Vec<Session>> {
    sessions::table
        .filter(sessions::user_id.eq(user_id))
        .filter(sessions::is_active.eq(true))
        .order(sessions::created_at.asc())
        .get_results::<Session>(connection)
        .to_service_result()
}

pub fn find_session_by_id(
    connection: &mut PgConnection,
    session_id: i32,
) -> ServiceResult<Session> {
    sessions::table
        .find(session_id)
        .first::<Session>(connection)
        .to_service_result_find(String::from("session_not_found_error"))
}

pub fn deactivate_all_user_sessions(
    connection: &mut PgConnection,
    user_id: i32,
) -> ServiceResult<Vec<Session>> {
    diesel::update(sessions::table)
        .filter(sessions::user_id.eq(user_id))
        .filter(sessions::is_active.eq(true))
        .set(sessions::is_active.eq(false))
        .get_results::<Session>(connection)
        .to_service_result()
}

pub fn deactivate_user_session(
    connection: &mut PgConnection,
    user: &User,
    session_id: i32,
) -> ServiceResult<Session> {
    let session = find_session_by_id(connection, session_id)?;
    if !session.is_active {
        deactivate_all_user_sessions(connection, user.id)?;
        return Err(AppError::ValidationError(Box::new(|locale| {
            t!("session_is_not_active_error", locale = locale)
        })));
    }
    if session.user_id != user.id {
        return Err(AppError::ForbiddenError(String::from(
            "other_user_session_forbidden_error",
        )));
    }
    diesel::update(sessions::table.filter(sessions::id.eq(session_id)))
        .set(sessions::is_active.eq(false))
        .get_result::<Session>(connection)
        .to_service_result()
}

impl From<UserAgentDevice<'_>> for DeviceType {
    fn from(user_agent_device: UserAgentDevice) -> Self {
        DeviceType {
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
        OSType {
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
        ProductType {
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
