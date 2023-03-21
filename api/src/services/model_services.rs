use crate::models::{Node, NodeValueType, Project, User};
use crate::response::{AppError, ServiceResult, ToServiceResult};
use crate::schema::nodes;
use crate::services::{permission_services, project_services};
use crate::types::{NodeInCreateType, NodeOutType};
use crate::web_socket::{WebSocketProjectService, WebSocketService};
use diesel::prelude::*;
use diesel::PgConnection;

pub async fn create_node(
    conn: &mut PgConnection,
    project_service: WebSocketProjectService,
    user: &User,
    project_id: i32,
    node_in: NodeInCreateType,
) -> ServiceResult<NodeOutType> {
    let project = project_services::find_project_by_id(conn, project_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    permission_services::can_change_model(conn, &project, user.id)?;
    check_node_value(&project, node_in.value.clone())?;
    let node = diesel::insert_into(nodes::table)
        .values((
            nodes::project_id.eq(project_id),
            nodes::name.eq(node_in.name),
            nodes::description.eq(node_in.description),
            nodes::value.eq(node_in.value),
            nodes::x_position.eq(node_in.x_position),
            nodes::y_position.eq(node_in.y_position),
        ))
        .get_result::<Node>(conn)
        .to_service_result()?;
    let node_out = NodeOutType::from(node);
    project_service
        .notify(project_id, String::from("create_node"), node_out.clone())
        .await;
    Ok(node_out)
}

fn check_node_value(project: &Project, value: Option<f64>) -> ServiceResult<()> {
    match value {
        Some(value) => match project.node_value_type {
            NodeValueType::None => Err(AppError::ValidationError(Box::new(move |locale| {
                t!(
                    "invalid_node_value_error",
                    locale = locale,
                    expected = "null",
                    got = value
                )
            }))),
            NodeValueType::FromZeroToOne => {
                if value >= 0.0 && value <= 1.0 {
                    Ok(())
                } else {
                    Err(AppError::ValidationError(Box::new(move |locale| {
                        t!(
                            "invalid_node_value_error",
                            locale = locale,
                            expected = "[0.0; 1.0]",
                            got = value
                        )
                    })))
                }
            }
        },
        None => match project.node_value_type {
            NodeValueType::None => Ok(()),
            NodeValueType::FromZeroToOne => {
                Err(AppError::ValidationError(Box::new(move |locale| {
                    t!(
                        "invalid_node_value_error",
                        locale = locale,
                        expected = "[0.0; 1.0]",
                        got = "null"
                    )
                })))
            }
        },
    }
}

impl From<Node> for NodeOutType {
    fn from(node: Node) -> Self {
        NodeOutType {
            id: node.id,
            name: node.name,
            description: node.description,
            value: node.value,
            project_id: node.project_id,
            x_position: node.x_position,
            y_position: node.y_position,
            created_at: node.created_at,
            updated_at: node.updated_at,
        }
    }
}
