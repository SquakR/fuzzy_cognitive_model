// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "project_user_status_value"))]
    pub struct ProjectUserStatusValue;
}

diesel::table! {
    email_confirmations (id) {
        id -> Int4,
        user_id -> Int4,
        email -> Varchar,
        is_confirmed -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    password_resets (id) {
        id -> Int4,
        user_id -> Int4,
        is_reset -> Bool,
        is_valid -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    permissions (key) {
        key -> Varchar,
        description -> Text,
    }
}

diesel::table! {
    plugins (name) {
        name -> Varchar,
        description -> Text,
    }
}

diesel::table! {
    project_plugins (id) {
        id -> Int4,
        project_id -> Int4,
        plugin_name -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ProjectUserStatusValue;

    project_user_statuses (id) {
        id -> Int4,
        project_user_id -> Int4,
        status -> ProjectUserStatusValue,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    project_users (id) {
        id -> Int4,
        project_id -> Int4,
        user_id -> Int4,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    projects (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
        is_public -> Bool,
        is_archived -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    sessions (id) {
        id -> Int4,
        is_active -> Bool,
        user_id -> Int4,
        ip_address -> Cidr,
        user_agent -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    user_permissions (id) {
        id -> Int4,
        permission_key -> Varchar,
        project_user_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
        is_email_confirmed -> Bool,
        first_name -> Varchar,
        second_name -> Nullable<Varchar>,
        last_name -> Varchar,
        avatar -> Nullable<Varchar>,
        language -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(email_confirmations -> users (user_id));
diesel::joinable!(password_resets -> users (user_id));
diesel::joinable!(project_plugins -> plugins (plugin_name));
diesel::joinable!(project_plugins -> projects (project_id));
diesel::joinable!(project_user_statuses -> project_users (project_user_id));
diesel::joinable!(project_users -> projects (project_id));
diesel::joinable!(project_users -> users (user_id));
diesel::joinable!(sessions -> users (user_id));
diesel::joinable!(user_permissions -> permissions (permission_key));
diesel::joinable!(user_permissions -> project_users (project_user_id));

diesel::allow_tables_to_appear_in_same_query!(
    email_confirmations,
    password_resets,
    permissions,
    plugins,
    project_plugins,
    project_user_statuses,
    project_users,
    projects,
    sessions,
    user_permissions,
    users,
);
