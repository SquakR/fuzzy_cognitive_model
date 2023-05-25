use super::model_service::WebSocketModelService;
use super::WebSocketAdjustmentRunService;
use crate::authenticate;
use crate::cookies::GetPrivate;
use crate::db;
use crate::models::{Session, User};
use crate::services::project_user_services;
use crate::utils;
use cookie::{Cookie, CookieJar, Key};
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use rocket::fairing;
use rocket::http::hyper::Uri;
use rocket::log::PaintExt;
use rocket::tokio::net::{TcpListener, TcpStream};
use rocket::tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use rocket::tokio::sync::Mutex;
use rocket::yansi::Paint;
use rocket::{Build, Data, Orbit, Request as RocketRequest, Rocket};
use std::collections::HashMap;
use std::fmt;
use std::net::SocketAddr;
use std::sync::mpsc;
use std::sync::mpsc::channel;
use std::sync::Arc;
use tokio_tungstenite::tungstenite::handshake::server::{ErrorResponse, Request, Response};
use tokio_tungstenite::tungstenite::http::StatusCode;
use tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode;
use tokio_tungstenite::tungstenite::protocol::{CloseFrame, Message};
use tokio_tungstenite::WebSocketStream;

pub struct WebSocketListener {
    host: String,
    port: i32,
    secret_key: Key,
    model_connections: ModelConnections,
    adjustment_run_connections: AdjustmentRunConnections,
    model_service: WebSocketModelService,
    adjustment_run_service: WebSocketAdjustmentRunService,
}

macro_rules! shutdown {
    ($connection:expr) => {
        for (_, senders) in $connection.lock().await.iter_mut() {
            for connection_sender in senders.iter_mut() {
                connection_sender
                    .sender
                    .send(Message::Close(Some(CloseFrame {
                        code: CloseCode::Away,
                        reason: "The server has been shut down.".into(),
                    })))
                    .unwrap();
            }
        }
    };
}

#[rocket::async_trait]
impl fairing::Fairing for WebSocketListener {
    fn info(&self) -> fairing::Info {
        fairing::Info {
            name: "WebSocket listener",
            kind: fairing::Kind::Ignite
                | fairing::Kind::Liftoff
                | fairing::Kind::Request
                | fairing::Kind::Shutdown,
        }
    }
    async fn on_ignite(&self, rocket: Rocket<Build>) -> fairing::Result {
        match self.listen() {
            Ok(_) => Ok(rocket),
            Err(err) => {
                error_!("{}", err);
                Err(rocket)
            }
        }
    }
    async fn on_liftoff(&self, _: &Rocket<Orbit>) {
        info!(
            "{}{}:",
            Paint::emoji("🖇️ "),
            Paint::magenta("WebSocket listener")
        );
        info_!("{}: {}", "Host", Paint::default(self.host.clone()));
        info_!("{}: {}", "Port", Paint::default(self.port));
    }
    async fn on_request(&self, request: &mut RocketRequest<'_>, _: &mut Data<'_>) {
        let model_service = self.model_service.clone();
        request.local_cache(move || model_service);
        let adjustment_run_service = self.adjustment_run_service.clone();
        request.local_cache(move || adjustment_run_service);
    }
    async fn on_shutdown(&self, _: &Rocket<Orbit>) {
        shutdown!(self.model_connections);
        shutdown!(self.adjustment_run_connections);
    }
}

struct ListenerError(pub String);

impl fmt::Display for ListenerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

enum ConnectionType {
    Model(ModelConnectionData),
    AdjustmentRun(AdjustmentRunConnectionData),
}

#[derive(Clone, PartialEq, Eq)]
pub struct ModelConnectionData {
    pub session_id: i32,
    pub user_id: i32,
    pub project_id: i32,
}

impl ModelConnectionData {
    pub fn new(session_id: i32, user_id: i32, project_id: i32) -> Self {
        Self {
            session_id,
            user_id,
            project_id,
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct AdjustmentRunConnectionData {
    pub session_id: i32,
    pub user_id: i32,
    pub project_id: i32,
}

impl AdjustmentRunConnectionData {
    pub fn new(session_id: i32, user_id: i32, project_id: i32) -> Self {
        Self {
            session_id,
            user_id,
            project_id,
        }
    }
}

pub struct ConnectionSender<T> {
    pub data: T,
    pub sender: UnboundedSender<Message>,
}

impl<T> ConnectionSender<T> {
    pub fn new(data: T, sender: UnboundedSender<Message>) -> Self {
        Self { data, sender }
    }
}

impl<T: PartialEq> PartialEq<ConnectionSender<T>> for ConnectionSender<T> {
    fn eq(&self, other: &ConnectionSender<T>) -> bool {
        self.data == other.data
    }
}
impl<T: PartialEq> Eq for ConnectionSender<T> {}

pub type Connections<K, V> = Arc<Mutex<HashMap<K, V>>>;

pub type ModelConnections = Connections<i32, Vec<ConnectionSender<ModelConnectionData>>>;
pub type AdjustmentRunConnections =
    Connections<i32, Vec<ConnectionSender<AdjustmentRunConnectionData>>>;

impl WebSocketListener {
    pub fn new(host: String, port: i32) -> Self {
        let mut buf = [0u8; 88];
        let model_connections = Arc::new(Mutex::new(HashMap::new()));
        let adjustment_run_connections = Arc::new(Mutex::new(HashMap::new()));
        Self {
            host,
            port,
            secret_key: Key::from(
                binascii::b64decode(utils::get_env("ROCKET_SECRET_KEY").as_bytes(), &mut buf)
                    .unwrap(),
            ),
            model_connections: Arc::clone(&model_connections),
            adjustment_run_connections: Arc::clone(&adjustment_run_connections),
            model_service: WebSocketModelService::new(model_connections),
            adjustment_run_service: WebSocketAdjustmentRunService::new(adjustment_run_connections),
        }
    }
    fn listen(&self) -> Result<(), ListenerError> {
        let host = self.host.clone();
        let port = self.port;
        let secret_key = self.secret_key.clone();
        let model_connections = Arc::clone(&self.model_connections);
        let adjustment_run_connections = Arc::clone(&self.adjustment_run_connections);
        let (tx, rx) = mpsc::channel();
        rocket::tokio::spawn(async move {
            let addr = format!("{}:{}", host, port);
            let listener = match TcpListener::bind(&addr).await {
                Ok(listener) => {
                    tx.send(Ok(())).unwrap();
                    listener
                }
                Err(_) => {
                    tx.send(Err(ListenerError(String::from(
                        "Failed to create WebSocket listener.",
                    ))))
                    .unwrap();
                    return;
                }
            };
            let mut connection_id = 1;
            while let Ok((stream, addr)) = listener.accept().await {
                rocket::tokio::spawn(WebSocketListener::handle_connection(
                    connection_id,
                    secret_key.clone(),
                    Arc::clone(&model_connections),
                    Arc::clone(&adjustment_run_connections),
                    stream,
                    addr,
                ));
                connection_id += 1;
            }
        });
        rx.recv().unwrap()
    }
    async fn handle_connection(
        connection_id: u32,
        secret_key: Key,
        model_connections: ModelConnections,
        adjustment_run_connections: AdjustmentRunConnections,
        raw_stream: TcpStream,
        addr: SocketAddr,
    ) {
        info_!(
            "Incoming WebSocket connection with identifier {} from {}",
            Paint::default(connection_id),
            Paint::default(addr)
        );
        let (ws_stream, connection_type) =
            match WebSocketListener::handle_request(connection_id, secret_key, raw_stream, &addr)
                .await
            {
                Ok(res) => res,
                Err(_) => {
                    return;
                }
            };
        let (tx, rx) = unbounded_channel();
        let (outgoing, incoming) = ws_stream.split();
        match connection_type {
            ConnectionType::Model(model_connection_data) => {
                model_connections
                    .lock()
                    .await
                    .entry(model_connection_data.project_id)
                    .or_insert(vec![])
                    .push(ConnectionSender::new(model_connection_data.clone(), tx));
                WebSocketListener::base_connection_loop(outgoing, incoming, rx).await;
                model_connections
                    .lock()
                    .await
                    .get_mut(&model_connection_data.project_id)
                    .unwrap()
                    .retain(|sender| sender.data != model_connection_data);
            }
            ConnectionType::AdjustmentRun(adjustment_run_connection_data) => {
                adjustment_run_connections
                    .lock()
                    .await
                    .entry(adjustment_run_connection_data.project_id)
                    .or_insert(vec![])
                    .push(ConnectionSender::new(
                        adjustment_run_connection_data.clone(),
                        tx,
                    ));
                WebSocketListener::base_connection_loop(outgoing, incoming, rx).await;
                adjustment_run_connections
                    .lock()
                    .await
                    .get_mut(&adjustment_run_connection_data.project_id)
                    .unwrap()
                    .retain(|sender| sender.data != adjustment_run_connection_data);
            }
        };
        info_!(
            "WebSocket connection with identifier {} from {} disconnected",
            Paint::default(connection_id),
            Paint::default(&addr)
        );
    }
    async fn base_connection_loop(
        mut outgoing: SplitSink<WebSocketStream<TcpStream>, Message>,
        mut incoming: SplitStream<WebSocketStream<TcpStream>>,
        mut rx: UnboundedReceiver<Message>,
    ) -> () {
        loop {
            rocket::tokio::select! {
                in_msg = incoming.next() => {
                    match in_msg {
                        Some(in_msg) => match in_msg {
                            Ok(msg) => match msg {
                                Message::Ping(payload) => match outgoing.send(Message::Pong(payload)).await {
                                    Ok(_) => {},
                                    Err(_) => return
                                },
                                Message::Text(text) => {
                                    if text == "ping" {
                                        match outgoing.send(Message::Text(String::from("pong"))).await {
                                            Ok(_) => {},
                                            Err(_) => return
                                        }
                                    }
                                }
                                Message::Close(_) => return,
                                _ => {}
                            },
                            Err(_) => return
                        },
                        None => return
                    }
                }
                out_msg = rx.recv() => {
                    match out_msg {
                        Some(out_msg) => {
                            match outgoing.send(out_msg).await {
                                Ok(_) => {},
                                Err(_) => return
                            }
                        }
                        None => return
                    }
                }
            }
        }
    }
    async fn handle_request(
        connection_id: u32,
        secret_key: Key,
        raw_stream: TcpStream,
        addr: &SocketAddr,
    ) -> Result<(WebSocketStream<TcpStream>, ConnectionType), ()> {
        let (tx, rx) = channel();
        let ws_stream = match tokio_tungstenite::accept_hdr_async(
            raw_stream,
            move |request: &Request, response: Response| {
                let (connection_type, response) =
                    match WebSocketListener::on_request(secret_key, request, response) {
                        Ok(connection_type) => connection_type,
                        Err((uri, err)) => {
                            WebSocketListener::log_error(err.status(), uri, connection_id, addr);
                            return Err(err);
                        }
                    };
                tx.send(connection_type).unwrap();
                Ok(response)
            },
        )
        .await
        {
            Ok(ws_stream) => ws_stream,
            Err(_) => {
                return Err(());
            }
        };
        let connection_type = rx.recv().unwrap();
        Ok((ws_stream, connection_type))
    }
    fn on_request(
        secret_key: Key,
        request: &Request,
        response: Response,
    ) -> Result<(ConnectionType, Response), (Uri, ErrorResponse)> {
        let (user, session) = WebSocketListener::parse_cookie(secret_key, request)?;
        let uri_string = request.uri().to_string();
        let model_uri = "/api/v1/model/";
        let adjustment_run_uri = "/api/v1/adjustment_run/";
        if uri_string.starts_with(model_uri) || uri_string.starts_with(adjustment_run_uri) {
            let project_id_str = if uri_string.starts_with(model_uri) {
                uri_string.trim_start_matches(model_uri)
            } else {
                uri_string.trim_start_matches(adjustment_run_uri)
            };
            if let Ok(project_id) = project_id_str.parse::<i32>() {
                let conn = &mut db::establish_connection();
                let is_project_member =
                    match project_user_services::is_project_member(conn, &user, project_id) {
                        Ok(is_project_member) => is_project_member,
                        Err(_) => false,
                    };
                if !is_project_member {
                    return Err(WebSocketListener::create_or_request_error(
                        request,
                        StatusCode::FORBIDDEN,
                        "Forbidden error",
                    ));
                }
                return if uri_string.starts_with(model_uri) {
                    Ok((
                        ConnectionType::Model(ModelConnectionData::new(
                            session.id, user.id, project_id,
                        )),
                        response,
                    ))
                } else {
                    Ok((
                        ConnectionType::AdjustmentRun(AdjustmentRunConnectionData::new(
                            session.id, user.id, project_id,
                        )),
                        response,
                    ))
                };
            }
        }
        Err(WebSocketListener::create_or_request_error(
            request,
            StatusCode::NOT_FOUND,
            "Not found",
        ))
    }
    fn parse_cookie(
        secret_key: Key,
        request: &Request,
    ) -> Result<(User, Session), (Uri, ErrorResponse)> {
        let cookie_value = match request.headers().get("cookie") {
            Some(cookie) => match cookie.to_str() {
                Ok(value) => value.to_owned(),
                Err(_) => {
                    return Err(WebSocketListener::create_or_request_error(
                        request,
                        StatusCode::UNAUTHORIZED,
                        "Unauthorized",
                    ));
                }
            },
            None => {
                return Err(WebSocketListener::create_or_request_error(
                    request,
                    StatusCode::UNAUTHORIZED,
                    "Unauthorized",
                ));
            }
        };
        let mut cookies_jar = CookieJar::new();
        for cookie in Cookie::split_parse_encoded(cookie_value) {
            if let Ok(cookie) = cookie {
                cookies_jar.add_original(cookie)
            }
        }
        let conn = &mut db::establish_connection();
        match authenticate!(conn, &cookies_jar.private(&secret_key)) {
            Ok(authentication_data) => Ok(authentication_data),
            Err(status) => {
                let status_code = StatusCode::from_u16(status.code).unwrap();
                let message = match status_code {
                    StatusCode::UNAUTHORIZED => "Unauthorized",
                    StatusCode::BAD_REQUEST => "Bad request",
                    _ => unreachable!(),
                };
                Err(WebSocketListener::create_or_request_error(
                    request,
                    status_code,
                    message,
                ))
            }
        }
    }
    fn create_or_request_error(
        request: &Request,
        status_code: StatusCode,
        message: &str,
    ) -> (Uri, ErrorResponse) {
        let mut error_response = ErrorResponse::new(Some(message.to_owned()));
        let status = error_response.status_mut();
        *status = status_code;
        (request.uri().clone(), error_response)
    }
    fn log_error(status_code: StatusCode, uri: Uri, connection_id: u32, addr: &SocketAddr) -> () {
        match status_code {
            StatusCode::BAD_REQUEST => error_!(
                "Bad request on WebSocket connection with identifier {} from {}",
                Paint::default(uri),
                Paint::default(connection_id),
            ),
            StatusCode::UNAUTHORIZED => error_!(
                "No authorization on WebSocket connection with identifier {} from {}",
                Paint::default(uri),
                Paint::default(connection_id)
            ),
            StatusCode::FORBIDDEN => error_!(
                "Insufficient permissions on WebSocket connection with identifier {} from {}",
                Paint::default(uri),
                Paint::default(connection_id),
            ),
            StatusCode::NOT_FOUND => error_!(
                "Uri {} not found on WebSocket connection with identifier {} from {}",
                Paint::default(uri),
                Paint::default(connection_id),
                Paint::default(&addr)
            ),
            _ => unreachable!(),
        }
    }
}
