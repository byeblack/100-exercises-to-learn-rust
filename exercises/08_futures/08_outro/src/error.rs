use salvo::prelude::*;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Server error: {0}")]
    ServerError(#[from] salvo::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AppError {
    #[error("Server error: {0}")]
    ServerError(#[from] salvo::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Ticket title error: {0}")]
    TicketTitleError(#[from] TicketTitleError),
    #[error("Ticket description error: {0}")]
    TicketDescriptionError(#[from] TicketDescriptionError),
    #[error("{0}")]
    InvalidTicketStatus(String),
    #[error("Failed to serialize response")]
    SerializationError(#[from] serde_json::Error),
    #[error("Failed to parse JSON: {0}")]
    JsonParseError(#[from] salvo::http::ParseError),
    #[error("{0}")]
    TicketPatchError(#[from] TicketPatchError),

    #[error("Lock is poisoned")]
    PoisonError,
    #[error("Ticket store not initialized")]
    TicketStoreNotInitialized,
    #[error("Ticket not found")]
    NotTicket,
    #[error("Invalid ticket ID")]
    InvalidTicketId,
}

pub type AppResult<T> = Result<T, AppError>;

#[derive(Serialize)]
pub struct AppErrorWriter {
    error: String,
}

#[async_trait]
impl Writer for AppError {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        let output = AppErrorWriter {
            error: self.to_string(),
        };
        let err = serde_json::to_string(&output).unwrap_or_else(|_| {
            serde_json::json!({"error": "Failed to serialize error"}).to_string()
        });

        res.render(Text::Json(err));
    }
}

#[derive(Debug, Error)]
pub enum TicketTitleError {
    #[error("The title cannot be empty")]
    Empty,
    #[error("The title cannot be longer than {0} characters")]
    TooLong(usize),
}

#[derive(Debug, Error)]
pub enum TicketDescriptionError {
    #[error("The description cannot be empty")]
    Empty,
    #[error("The description cannot be longer than {0} characters")]
    TooLong(usize),
}

#[derive(Debug, Error)]
#[error("Invalid ticket status: {0}")]
pub enum TicketStatusError {
    Invalid(String),
}

#[derive(Debug, Error)]
pub enum TicketPatchError {
    #[error("At least one field must be present")]
    MustContainOneField,
}
