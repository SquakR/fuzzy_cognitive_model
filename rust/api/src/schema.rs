// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "concept_value_type"))]
    pub struct ConceptValueType;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "connection_value_type"))]
    pub struct ConnectionValueType;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "dynamic_model_type"))]
    pub struct DynamicModelType;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "project_user_status_value"))]
    pub struct ProjectUserStatusValue;
}

diesel::table! {
    adjustment_chromosomes (id) {
        id -> Int4,
        adjustment_generation_id -> Int4,
        number -> Int4,
        time -> Int4,
        error -> Float8,
    }
}

diesel::table! {
    adjustment_concept_values (id) {
        id -> Int4,
        adjustment_chromosome_id -> Int4,
        concept_id -> Int4,
        value -> Float8,
    }
}

diesel::table! {
    adjustment_connection_values (id) {
        id -> Int4,
        adjustment_chromosome_id -> Int4,
        connection_id -> Int4,
        value -> Float8,
    }
}

diesel::table! {
    adjustment_generations (id) {
        id -> Int4,
        adjustment_run_id -> Int4,
        number -> Int4,
        error -> Float8,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::DynamicModelType;

    adjustment_runs (id) {
        id -> Int4,
        project_id -> Int4,
        model_copy_id -> Int4,
        name -> Varchar,
        description -> Text,
        min_model_time -> Int4,
        max_model_time -> Int4,
        dynamic_model_type -> DynamicModelType,
        generation_size -> Int4,
        generation_save_interval -> Int4,
        max_generations -> Int4,
        max_without_improvements -> Int4,
        error -> Float8,
        created_at -> Timestamptz,
        result_chromosome_id -> Nullable<Int4>,
    }
}

diesel::table! {
    concept_constraints (concept_id) {
        concept_id -> Int4,
        has_constraint -> Bool,
        min_value -> Float8,
        include_min_value -> Bool,
        max_value -> Float8,
        include_max_value -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::DynamicModelType;

    concept_dynamic_models (concept_id) {
        concept_id -> Int4,
        dynamic_model_type -> Nullable<DynamicModelType>,
    }
}

diesel::table! {
    concepts (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
        value -> Nullable<Float8>,
        project_id -> Int4,
        x_position -> Float8,
        y_position -> Float8,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    connection_constraints (connection_id) {
        connection_id -> Int4,
        has_constraint -> Bool,
        min_value -> Float8,
        include_min_value -> Bool,
        max_value -> Float8,
        include_max_value -> Bool,
    }
}

diesel::table! {
    connections (id) {
        id -> Int4,
        description -> Text,
        value -> Float8,
        source_id -> Int4,
        target_id -> Int4,
        project_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    control_concepts (concept_id) {
        concept_id -> Int4,
        is_control -> Bool,
    }
}

diesel::table! {
    control_connections (connection_id) {
        connection_id -> Int4,
        is_control -> Bool,
    }
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
    model_copies (id) {
        id -> Int4,
        project_id -> Int4,
        model -> Jsonb,
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
    plugin_dependencies (id) {
        id -> Int4,
        dependent_plugin_name -> Varchar,
        dependency_plugin_name -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ConceptValueType;
    use super::sql_types::ConnectionValueType;

    plugins (name) {
        name -> Varchar,
        description -> Text,
        concept_value_type -> Nullable<ConceptValueType>,
        connection_value_type -> Nullable<ConnectionValueType>,
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
    project_user_permissions (id) {
        id -> Int4,
        permission_key -> Varchar,
        project_user_id -> Int4,
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
        last_status_id -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ConceptValueType;
    use super::sql_types::ConnectionValueType;

    projects (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
        is_public -> Bool,
        is_archived -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        concept_value_type -> ConceptValueType,
        connection_value_type -> ConnectionValueType,
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
    target_concepts (concept_id) {
        concept_id -> Int4,
        is_target -> Bool,
        value -> Nullable<Float8>,
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
        locale -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(adjustment_chromosomes -> adjustment_generations (adjustment_generation_id));
diesel::joinable!(adjustment_concept_values -> adjustment_chromosomes (adjustment_chromosome_id));
diesel::joinable!(adjustment_concept_values -> concepts (concept_id));
diesel::joinable!(adjustment_connection_values -> adjustment_chromosomes (adjustment_chromosome_id));
diesel::joinable!(adjustment_connection_values -> connections (connection_id));
diesel::joinable!(adjustment_generations -> adjustment_runs (adjustment_run_id));
diesel::joinable!(adjustment_runs -> adjustment_chromosomes (result_chromosome_id));
diesel::joinable!(adjustment_runs -> model_copies (model_copy_id));
diesel::joinable!(adjustment_runs -> projects (project_id));
diesel::joinable!(concept_constraints -> concepts (concept_id));
diesel::joinable!(concept_dynamic_models -> concepts (concept_id));
diesel::joinable!(concepts -> projects (project_id));
diesel::joinable!(connection_constraints -> connections (connection_id));
diesel::joinable!(connections -> projects (project_id));
diesel::joinable!(control_concepts -> concepts (concept_id));
diesel::joinable!(control_connections -> connections (connection_id));
diesel::joinable!(email_confirmations -> users (user_id));
diesel::joinable!(model_copies -> projects (project_id));
diesel::joinable!(password_resets -> users (user_id));
diesel::joinable!(project_plugins -> plugins (plugin_name));
diesel::joinable!(project_plugins -> projects (project_id));
diesel::joinable!(project_user_permissions -> permissions (permission_key));
diesel::joinable!(project_user_permissions -> project_users (project_user_id));
diesel::joinable!(project_users -> projects (project_id));
diesel::joinable!(project_users -> users (user_id));
diesel::joinable!(sessions -> users (user_id));
diesel::joinable!(target_concepts -> concepts (concept_id));

diesel::allow_tables_to_appear_in_same_query!(
    adjustment_chromosomes,
    adjustment_concept_values,
    adjustment_connection_values,
    adjustment_generations,
    adjustment_runs,
    concept_constraints,
    concept_dynamic_models,
    concepts,
    connection_constraints,
    connections,
    control_concepts,
    control_connections,
    email_confirmations,
    model_copies,
    password_resets,
    permissions,
    plugin_dependencies,
    plugins,
    project_plugins,
    project_user_permissions,
    project_user_statuses,
    project_users,
    projects,
    sessions,
    target_concepts,
    users,
);
