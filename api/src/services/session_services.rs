use crate::errors::AppError;
use crate::models::{Session, User};
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
) -> Result<Session, AppError> {
    AppError::update_result(
        diesel::insert_into(sessions::table)
            .values((
                sessions::user_id.eq(user_id),
                sessions::ip_address.eq(ip_address),
                sessions::user_agent.eq(user_agent),
            ))
            .get_result::<Session>(connection),
    )
}

pub fn get_user_active_sessions(
    connection: &mut PgConnection,
    user_id: i32,
) -> Result<Vec<Session>, AppError> {
    AppError::update_result(
        sessions::table
            .filter(sessions::user_id.eq(user_id))
            .filter(sessions::is_active.eq(true))
            .get_results::<Session>(connection),
    )
}

pub fn find_session_by_id(
    connection: &mut PgConnection,
    session_id: i32,
) -> Result<Session, AppError> {
    AppError::update_result(
        sessions::table
            .find(session_id)
            .first::<Session>(connection),
    )
}

pub fn deactivate_all_user_sessions(
    connection: &mut PgConnection,
    user: &User,
) -> Result<Vec<Session>, AppError> {
    AppError::update_result(
        diesel::update(sessions::table)
            .filter(sessions::user_id.eq(user.id))
            .filter(sessions::is_active.eq(true))
            .set(sessions::is_active.eq(false))
            .get_results::<Session>(connection),
    )
}

pub fn deactivate_user_session(
    connection: &mut PgConnection,
    user: &User,
    session_id: i32,
) -> Result<Session, AppError> {
    let session = find_session_by_id(connection, session_id)?;
    if !session.is_active {
        deactivate_all_user_sessions(connection, user)?;
        return Err(AppError::BadRequestError);
    }
    if session.user_id != user.id {
        return Err(AppError::BadRequestError);
    }
    AppError::update_result(
        diesel::update(sessions::table.filter(sessions::id.eq(session_id)))
            .set(sessions::is_active.eq(false))
            .get_result::<Session>(connection),
    )
}

impl From<UserAgentDevice<'_>> for DeviceType {
    fn from(value: UserAgentDevice) -> Self {
        DeviceType {
            name: value.name.and_then(|name| Some(name.into_owned())),
            brand: value.brand.and_then(|brand| Some(brand.into_owned())),
            model: value.model.and_then(|model| Some(model.into_owned())),
        }
    }
}

impl From<UserAgentOS<'_>> for OSType {
    fn from(value: UserAgentOS<'_>) -> Self {
        OSType {
            name: value.name.and_then(|name| Some(name.into_owned())),
            major: value.major.and_then(|major| Some(major.into_owned())),
            minor: value.minor.and_then(|minor| Some(minor.into_owned())),
            patch: value.patch.and_then(|patch| Some(patch.into_owned())),
            patch_minor: value
                .patch_minor
                .and_then(|patch_minor| Some(patch_minor.into_owned())),
        }
    }
}

impl From<UserAgentProduct<'_>> for ProductType {
    fn from(value: UserAgentProduct<'_>) -> Self {
        ProductType {
            name: value.name.and_then(|name| Some(name.into_owned())),
            major: value.major.and_then(|major| Some(major.into_owned())),
            minor: value.minor.and_then(|minor| Some(minor.into_owned())),
            patch: value.patch.and_then(|patch| Some(patch.into_owned())),
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
