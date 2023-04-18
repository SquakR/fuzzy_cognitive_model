use crate::models::{Concept, ConceptValueType, Connection, ConnectionValueType, Project, User};
use crate::plugins::{ChangeConceptValueExtra, Plugins};
use crate::response::{ServiceResult, ToServiceResult};
use crate::schema::{concepts, connections, projects};
use crate::services::{permission_services, project_services};
use crate::types::{
    ConceptInChangeDescriptionType, ConceptInCreateType, ConceptInMoveType,
    ConceptOutChangeDescriptionType, ConceptOutChangeValueType, ConceptOutDeleteType,
    ConceptOutMoveType, ConceptOutType, ConnectionInCreateType, ConnectionOutChangeDescriptionType,
    ConnectionOutChangeValueType, ConnectionOutDeleteType, ConnectionOutType, ModelActionType,
    ModelOutType, ProjectOutType,
};
use crate::validation_error;
use crate::web_socket::WebSocketProjectService;
use chrono::DateTime;
use chrono::Utc;
use diesel::prelude::*;
use diesel::Connection as DieselConnection;
use diesel::PgConnection;
use schemars::JsonSchema;
use serde::Serialize;
use serde_json::{Map, Value};

pub fn get_model(
    conn: &mut PgConnection,
    plugins: &Plugins,
    user: &User,
    project_id: i32,
) -> ServiceResult<ModelOutType> {
    let project = project_services::find_project_by_id(conn, project_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    permission_services::can_view_project(conn, &project, user)?;
    let project = ProjectOutType::from_project(conn, project)?;
    let concepts = find_project_concepts(conn, project_id)
        .to_service_result()?
        .into_iter()
        .map(ConceptOutType::from)
        .collect();
    let connections = find_project_connections(conn, project_id)
        .to_service_result()?
        .into_iter()
        .map(ConnectionOutType::from)
        .collect();
    let mut model_out = ModelOutType {
        project,
        concepts,
        connections,
    };
    model_out = plugins
        .get_model_emitter
        .lock()
        .unwrap()
        .emit(model_out, ())?;
    Ok(model_out)
}

pub fn find_project_concepts(
    conn: &mut PgConnection,
    project_id: i32,
) -> QueryResult<Vec<Concept>> {
    projects::table
        .inner_join(concepts::table)
        .select(concepts::all_columns)
        .filter(projects::id.eq(project_id))
        .get_results::<Concept>(conn)
}

pub fn find_project_connections(
    conn: &mut PgConnection,
    project_id: i32,
) -> QueryResult<Vec<Connection>> {
    projects::table
        .inner_join(connections::table)
        .select(connections::all_columns)
        .filter(projects::id.eq(project_id))
        .get_results::<Connection>(conn)
}

pub fn find_concept_by_id(conn: &mut PgConnection, concept_id: i32) -> QueryResult<Concept> {
    concepts::table
        .filter(concepts::id.eq(concept_id))
        .get_result::<Concept>(conn)
}

pub fn find_project_by_concept_id(
    conn: &mut PgConnection,
    concept_id: i32,
) -> QueryResult<Project> {
    concepts::table
        .inner_join(projects::table)
        .select(projects::all_columns)
        .filter(concepts::id.eq(concept_id))
        .get_result::<Project>(conn)
}

pub fn find_project_by_connection_id(
    conn: &mut PgConnection,
    connection_id: i32,
) -> QueryResult<Project> {
    connections::table
        .inner_join(projects::table)
        .select(projects::all_columns)
        .filter(connections::id.eq(connection_id))
        .get_result::<Project>(conn)
}

pub async fn create_concept(
    conn: &mut PgConnection,
    plugins: &Plugins,
    project_service: WebSocketProjectService,
    user: &User,
    project_id: i32,
    concept_in: ConceptInCreateType,
) -> ServiceResult<ModelActionType<ConceptOutType>> {
    let project = project_services::find_project_by_id(conn, project_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    permission_services::can_change_model(conn, &project, user.id)?;
    check_concept_value(&project, concept_in.value.clone())?;
    let (concept, project) = conn
        .transaction(|conn| {
            let concept = diesel::insert_into(concepts::table)
                .values((
                    concepts::project_id.eq(project_id),
                    concepts::name.eq(concept_in.name),
                    concepts::description.eq(concept_in.description),
                    concepts::value.eq(concept_in.value),
                    concepts::x_position.eq(concept_in.x_position),
                    concepts::y_position.eq(concept_in.y_position),
                ))
                .get_result::<Concept>(conn)?;
            let project = project_services::update_project(conn, project_id, concept.created_at)?;
            Ok((concept, project))
        })
        .to_service_result()?;
    let concept_out = plugins
        .add_concept_emitter
        .lock()
        .unwrap()
        .emit(ConceptOutType::from(concept), project.clone())?;
    let model_action = ModelActionType::new(&project, String::from("create_concept"), concept_out);
    project_service.notify(model_action.clone()).await;
    Ok(model_action)
}

pub fn update_concept(
    conn: &mut PgConnection,
    concept_id: i32,
    project_id: i32,
    updated_at: DateTime<Utc>,
) -> QueryResult<(Concept, Project)> {
    conn.transaction(|conn| {
        let concept = diesel::update(concepts::table)
            .filter(concepts::id.eq(concept_id))
            .set(concepts::updated_at.eq(updated_at))
            .get_result::<Concept>(conn)?;
        let project = project_services::update_project(conn, project_id, updated_at)?;
        Ok((concept, project))
    })
}

pub fn update_connection(
    conn: &mut PgConnection,
    connection_id: i32,
    project_id: i32,
    updated_at: DateTime<Utc>,
) -> QueryResult<(Connection, Project)> {
    conn.transaction(|conn| {
        let connection = diesel::update(connections::table)
            .filter(connections::id.eq(connection_id))
            .set(connections::updated_at.eq(updated_at))
            .get_result::<Connection>(conn)?;
        let project = project_services::update_project(conn, project_id, updated_at)?;
        Ok((connection, project))
    })
}

pub async fn change_concept_description(
    conn: &mut PgConnection,
    project_service: WebSocketProjectService,
    user: &User,
    concept_id: i32,
    concept_in: ConceptInChangeDescriptionType,
) -> ServiceResult<ModelActionType<ConceptOutChangeDescriptionType>> {
    let project = find_project_by_concept_id(conn, concept_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    permission_services::can_change_model(conn, &project, user.id)?;
    let (concept, project) = conn
        .transaction(|conn| {
            let concept = diesel::update(concepts::table)
                .filter(concepts::id.eq(concept_id))
                .set((
                    concepts::name.eq(concept_in.name),
                    concepts::description.eq(concept_in.description),
                ))
                .get_result::<Concept>(conn)?;
            let project = project_services::update_project(conn, project.id, concept.updated_at)?;
            Ok((concept, project))
        })
        .to_service_result_find(String::from("concept_not_found_error"))?;
    let concept_out = ConceptOutChangeDescriptionType::from(concept);
    let model_action = ModelActionType::new(
        &project,
        String::from("change_concept_description"),
        concept_out,
    );
    project_service.notify(model_action.clone()).await;
    Ok(model_action)
}

pub async fn change_concept_value(
    conn: &mut PgConnection,
    plugins: &Plugins,
    project_service: WebSocketProjectService,
    user: &User,
    concept_id: i32,
    mut value: Option<f64>,
) -> ServiceResult<ModelActionType<ConceptOutChangeValueType>> {
    let project = find_project_by_concept_id(conn, concept_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    permission_services::can_change_model(conn, &project, user.id)?;
    value = plugins.change_concept_value_emitter.lock().unwrap().emit(
        value,
        ChangeConceptValueExtra::new(project.clone(), concept_id),
    )?;
    check_concept_value(&project, value.clone())?;
    let (concept, project) = conn
        .transaction(|conn| {
            let concept = diesel::update(concepts::table)
                .filter(concepts::id.eq(concept_id))
                .set((concepts::value.eq(value),))
                .get_result::<Concept>(conn)?;
            let project = project_services::update_project(conn, project.id, concept.updated_at)?;
            Ok((concept, project))
        })
        .to_service_result_find(String::from("concept_not_found_error"))?;
    let concept_out = ConceptOutChangeValueType::from(concept);
    let model_action =
        ModelActionType::new(&project, String::from("change_concept_value"), concept_out);
    project_service.notify(model_action.clone()).await;
    Ok(model_action)
}

pub async fn move_concept(
    conn: &mut PgConnection,
    project_service: WebSocketProjectService,
    user: &User,
    concept_id: i32,
    concept_in: ConceptInMoveType,
) -> ServiceResult<ModelActionType<ConceptOutMoveType>> {
    let project = find_project_by_concept_id(conn, concept_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    permission_services::can_change_model(conn, &project, user.id)?;
    let (concept, project) = conn
        .transaction(|conn| {
            let concept = diesel::update(concepts::table)
                .filter(concepts::id.eq(concept_id))
                .set((
                    concepts::x_position.eq(concept_in.x_position),
                    concepts::y_position.eq(concept_in.y_position),
                ))
                .get_result::<Concept>(conn)?;
            let project = project_services::update_project(conn, project.id, concept.updated_at)?;
            Ok((concept, project))
        })
        .to_service_result_find(String::from("concept_not_found_error"))?;
    let concept_out = ConceptOutMoveType::from(concept);
    let model_action = ModelActionType::new(&project, String::from("move_concept"), concept_out);
    project_service.notify(model_action.clone()).await;
    Ok(model_action)
}

pub async fn delete_concept(
    conn: &mut PgConnection,
    project_service: WebSocketProjectService,
    user: &User,
    concept_id: i32,
) -> ServiceResult<ModelActionType<ConceptOutDeleteType>> {
    let project = find_project_by_concept_id(conn, concept_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    permission_services::can_change_model(conn, &project, user.id)?;
    let (deleted_number, project) = conn
        .transaction(|conn| {
            let deleted_number =
                diesel::delete(concepts::table.filter(concepts::id.eq(concept_id)))
                    .execute(conn)?;
            let project = if deleted_number == 0 {
                project
            } else {
                project_services::update_project(conn, project.id, Utc::now())?
            };
            Ok((deleted_number, project))
        })
        .to_service_result()?;
    if deleted_number == 0 {
        return validation_error!("concept_not_found_error");
    }
    let model_action = ModelActionType::new(
        &project,
        String::from("delete_concept"),
        ConceptOutDeleteType {
            id: concept_id,
            updated_at: project.updated_at,
        },
    );
    project_service.notify(model_action.clone()).await;
    Ok(model_action)
}

pub async fn create_connection(
    conn: &mut PgConnection,
    plugins: &Plugins,
    project_service: WebSocketProjectService,
    user: &User,
    project_id: i32,
    connection_in: ConnectionInCreateType,
) -> ServiceResult<ModelActionType<ConnectionOutType>> {
    let project = project_services::find_project_by_id(conn, project_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    permission_services::can_change_model(conn, &project, user.id)?;
    find_concept_by_id(conn, connection_in.source_id)
        .to_service_result_find(String::from("connection_source_concept_not_found_error"))?;
    find_concept_by_id(conn, connection_in.target_id)
        .to_service_result_find(String::from("connection_target_concept_not_found_error"))?;
    check_connection_value(&project, connection_in.value)?;
    let (connection, project) = conn
        .transaction(|conn| {
            let connection = diesel::insert_into(connections::table)
                .values((
                    connections::project_id.eq(project_id),
                    connections::description.eq(connection_in.description),
                    connections::value.eq(connection_in.value),
                    connections::source_id.eq(connection_in.source_id),
                    connections::target_id.eq(connection_in.target_id),
                ))
                .get_result::<Connection>(conn)?;
            let project =
                project_services::update_project(conn, project_id, connection.created_at)?;
            Ok((connection, project))
        })
        .to_service_result_unique(String::from("connection_duplication_error"))?;
    let connection_out = plugins
        .add_connection_emitter
        .lock()
        .unwrap()
        .emit(ConnectionOutType::from(connection), project.clone())?;
    let model_action =
        ModelActionType::new(&project, String::from("create_connection"), connection_out);
    project_service.notify(model_action.clone()).await;
    Ok(model_action)
}

pub async fn change_connection_description(
    conn: &mut PgConnection,
    project_service: WebSocketProjectService,
    user: &User,
    connection_id: i32,
    description: String,
) -> ServiceResult<ModelActionType<ConnectionOutChangeDescriptionType>> {
    let project = find_project_by_connection_id(conn, connection_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    permission_services::can_change_model(conn, &project, user.id)?;
    let (connection, project) = conn
        .transaction(|conn| {
            let connection = diesel::update(connections::table)
                .filter(connections::id.eq(connection_id))
                .set(connections::description.eq(description))
                .get_result::<Connection>(conn)?;
            let project =
                project_services::update_project(conn, project.id, connection.updated_at)?;
            Ok((connection, project))
        })
        .to_service_result_find(String::from("connection_not_found_error"))?;
    let connection_out = ConnectionOutChangeDescriptionType::from(connection);
    let model_action = ModelActionType::new(
        &project,
        String::from("change_connection_description"),
        connection_out,
    );
    project_service.notify(model_action.clone()).await;
    Ok(model_action)
}

pub async fn change_connection_value(
    conn: &mut PgConnection,
    project_service: WebSocketProjectService,
    user: &User,
    connection_id: i32,
    value: f64,
) -> ServiceResult<ModelActionType<ConnectionOutChangeValueType>> {
    let project = find_project_by_connection_id(conn, connection_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    permission_services::can_change_model(conn, &project, user.id)?;
    check_connection_value(&project, value)?;
    let (connection, project) = conn
        .transaction(|conn| {
            let connection = diesel::update(connections::table)
                .filter(connections::id.eq(connection_id))
                .set((connections::value.eq(value),))
                .get_result::<Connection>(conn)?;
            let project =
                project_services::update_project(conn, project.id, connection.updated_at)?;
            Ok((connection, project))
        })
        .to_service_result_find(String::from("connection_not_found_error"))?;
    let connection_out = ConnectionOutChangeValueType::from(connection);
    let model_action = ModelActionType::new(
        &project,
        String::from("change_connection_value"),
        connection_out,
    );
    project_service.notify(model_action.clone()).await;
    Ok(model_action)
}

pub async fn delete_connection(
    conn: &mut PgConnection,
    project_service: WebSocketProjectService,
    user: &User,
    connection_id: i32,
) -> ServiceResult<ModelActionType<ConnectionOutDeleteType>> {
    let project = find_project_by_connection_id(conn, connection_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    permission_services::can_change_model(conn, &project, user.id)?;
    let (deleted_number, project) = conn
        .transaction(|conn| {
            let deleted_number =
                diesel::delete(connections::table.filter(connections::id.eq(connection_id)))
                    .execute(conn)?;
            let project = if deleted_number == 0 {
                project
            } else {
                project_services::update_project(conn, project.id, Utc::now())?
            };
            Ok((deleted_number, project))
        })
        .to_service_result()?;
    if deleted_number == 0 {
        return validation_error!("connection_not_found_error");
    }
    let model_action = ModelActionType::new(
        &project,
        String::from("delete_connection"),
        ConnectionOutDeleteType {
            id: connection_id,
            updated_at: project.updated_at,
        },
    );
    project_service.notify(model_action.clone()).await;
    Ok(model_action)
}

pub fn check_concept_value(project: &Project, value: Option<f64>) -> ServiceResult<()> {
    match value {
        Some(value) => match project.concept_value_type {
            ConceptValueType::None => {
                validation_error!(
                    "invalid_concept_value_error",
                    expected = "null",
                    got = value
                )
            }
            ConceptValueType::FromZeroToOne => {
                if value >= 0.0 && value <= 1.0 {
                    Ok(())
                } else {
                    validation_error!(
                        "invalid_concept_value_error",
                        expected = "[0.0; 1.0]",
                        got = value
                    )
                }
            }
        },
        None => match project.concept_value_type {
            ConceptValueType::None => Ok(()),
            ConceptValueType::FromZeroToOne => {
                validation_error!(
                    "invalid_concept_value_error",
                    expected = "[0.0; 1.0]",
                    got = "null"
                )
            }
        },
    }
}

fn check_connection_value(project: &Project, value: f64) -> ServiceResult<()> {
    match project.connection_value_type {
        ConnectionValueType::Symbolic => {
            if value == 0.0 || value == 1.0 {
                Ok(())
            } else {
                validation_error!("invalid_connection_value_symbolic_error", got = value)
            }
        }
        ConnectionValueType::FromMinusOneToOne => {
            if value >= -1.0 && value <= 1.0 {
                Ok(())
            } else {
                validation_error!(
                    "invalid_connection_value_error",
                    expected = "[-1.0; 1.0]",
                    got = value
                )
            }
        }
    }
}

impl<T> ModelActionType<T>
where
    T: Clone + Serialize + JsonSchema,
{
    pub fn new(project: &Project, name: String, data: T) -> Self {
        Self {
            project_id: project.id,
            project_updated_at: project.updated_at,
            name,
            data,
        }
    }
}

impl From<Concept> for ConceptOutType {
    fn from(concept: Concept) -> Self {
        Self {
            id: concept.id,
            name: concept.name,
            description: concept.description,
            value: concept.value,
            project_id: concept.project_id,
            x_position: concept.x_position,
            y_position: concept.y_position,
            plugins_data: Value::Object(Map::new()),
            created_at: concept.created_at,
            updated_at: concept.updated_at,
        }
    }
}

impl From<Concept> for ConceptOutChangeDescriptionType {
    fn from(concept: Concept) -> Self {
        Self {
            id: concept.id,
            name: concept.name,
            description: concept.description,
            updated_at: concept.updated_at,
        }
    }
}

impl From<Concept> for ConceptOutChangeValueType {
    fn from(concept: Concept) -> Self {
        Self {
            id: concept.id,
            value: concept.value,
            updated_at: concept.updated_at,
        }
    }
}

impl From<Concept> for ConceptOutMoveType {
    fn from(concept: Concept) -> Self {
        Self {
            id: concept.id,
            x_position: concept.x_position,
            y_position: concept.y_position,
            updated_at: concept.updated_at,
        }
    }
}

impl From<Connection> for ConnectionOutType {
    fn from(connection: Connection) -> Self {
        Self {
            id: connection.id,
            description: connection.description,
            value: connection.value,
            source_id: connection.source_id,
            target_id: connection.target_id,
            project_id: connection.project_id,
            plugins_data: Value::Object(Map::new()),
            created_at: connection.created_at,
            updated_at: connection.updated_at,
        }
    }
}

impl From<Connection> for ConnectionOutChangeDescriptionType {
    fn from(connection: Connection) -> Self {
        Self {
            id: connection.id,
            description: connection.description,
            updated_at: connection.updated_at,
        }
    }
}

impl From<Connection> for ConnectionOutChangeValueType {
    fn from(connection: Connection) -> Self {
        Self {
            id: connection.id,
            value: connection.value,
            updated_at: connection.updated_at,
        }
    }
}
