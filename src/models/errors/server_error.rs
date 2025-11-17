use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum ServerErrorKind {
    BindError,          // Error al vincular puerto
    ConnectionError,    // Error de conexión TCP/DB
    ListenerError,      // Error del listener
    ShutdownError,      // Error al cerrar
    DatabaseError,      // Error de base de datos (queries)
}

#[derive(Debug)]
pub struct ServerError {
    code: i32,
    message: String,
    kind: ServerErrorKind,
    location: String,
}

impl ServerError {
    pub fn new(code: i32, message: &str, kind: ServerErrorKind, location: &str) -> Self {
        ServerError {
            code,
            message: message.to_string(),
            kind,
            location: location.to_string(),
        }
    }

    pub fn kind(&self) -> &ServerErrorKind {
        &self.kind
    }
    pub fn message(&self) -> &str {
        &self.message
    }
    pub fn code(&self) -> i32 {
        self.code
    }
    pub fn location(&self) -> &str {
        &self.location
    }
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let kind_str = match self.kind {
            ServerErrorKind::BindError => "vinculación",
            ServerErrorKind::ConnectionError => "conexión",
            ServerErrorKind::ListenerError => "listener",
            ServerErrorKind::ShutdownError => "cierre",
            ServerErrorKind::DatabaseError => "base de datos",
        };
        write!(
            f,
            "[{}] Error de servidor ({}): {} (en {})",
            self.code,
            kind_str,
            self.message,
            self.location
        )
    }
}

impl Error for ServerError {}