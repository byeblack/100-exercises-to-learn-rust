use crate::{
    error::{
        AppError, TicketDescriptionError, TicketPatchError, TicketStatusError, TicketTitleError,
    },
    store::TicketId,
};

use serde::{Deserialize, Serialize};

const TICKET_TITLE_MAX_LENGTH: usize = 50;
const TICKET_DESCRIPTION_MAX_LENGTH: usize = 300;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TryFrom<&str> for Status {
    type Error = TicketStatusError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "todo" => Ok(Self::ToDo),
            "inprogress" => Ok(Self::InProgress),
            "done" => Ok(Self::Done),
            _ => Err(TicketStatusError::Invalid(value.to_string())),
        }
    }
}

impl TryFrom<String> for Status {
    type Error = TicketStatusError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct TicketTitle(String);

impl TicketTitle {
    pub fn new(title: String) -> Result<Self, TicketTitleError> {
        if title.is_empty() {
            return Err(TicketTitleError::Empty);
        } else if title.len() > TICKET_TITLE_MAX_LENGTH {
            return Err(TicketTitleError::TooLong(TICKET_TITLE_MAX_LENGTH));
        }
        Ok(Self(title))
    }
}

impl TryFrom<String> for TicketTitle {
    type Error = TicketTitleError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for TicketTitle {
    type Error = TicketTitleError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_string())
    }
}

impl PartialEq<&str> for TicketTitle {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct TicketDescription(pub String);

impl TicketDescription {
    pub fn new(description: String) -> Result<Self, TicketDescriptionError> {
        if description.is_empty() {
            return Err(TicketDescriptionError::Empty);
        } else if description.len() > TICKET_DESCRIPTION_MAX_LENGTH {
            return Err(TicketDescriptionError::TooLong(
                TICKET_DESCRIPTION_MAX_LENGTH,
            ));
        }
        Ok(Self(description))
    }
}

impl TryFrom<String> for TicketDescription {
    type Error = TicketDescriptionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for TicketDescription {
    type Error = TicketDescriptionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_string())
    }
}

impl PartialEq<&str> for TicketDescription {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TicketPatch {
    pub title: Option<TicketTitle>,
    pub description: Option<TicketDescription>,
    pub status: Option<Status>,
}

impl TicketPatch {
    pub fn new(
        title: Option<TicketTitle>,
        description: Option<TicketDescription>,
        status: Option<Status>,
    ) -> Result<Self, TicketPatchError> {
        if title.is_none() && description.is_none() && status.is_none() {
            return Err(TicketPatchError::MustContainOneField);
        }

        Ok(Self {
            title,
            description,
            status,
        })
    }
}

pub fn validate_ticket(ticket: &Ticket) -> Result<(), AppError> {
    if ticket.title.0.is_empty() {
        return Err(TicketTitleError::Empty.into());
    } else if ticket.title.0.len() > TICKET_TITLE_MAX_LENGTH {
        return Err(TicketTitleError::TooLong(TICKET_TITLE_MAX_LENGTH).into());
    }

    if ticket.description.0.is_empty() {
        return Err(TicketDescriptionError::Empty.into());
    } else if ticket.description.0.len() > TICKET_DESCRIPTION_MAX_LENGTH {
        return Err(TicketDescriptionError::TooLong(TICKET_DESCRIPTION_MAX_LENGTH).into());
    }

    Ok(())
}

pub fn validate_ticket_draft(ticket_draft: &TicketDraft) -> Result<(), AppError> {
    if ticket_draft.title.0.is_empty() {
        return Err(TicketTitleError::Empty.into());
    } else if ticket_draft.title.0.len() > TICKET_TITLE_MAX_LENGTH {
        return Err(TicketTitleError::TooLong(TICKET_TITLE_MAX_LENGTH).into());
    }

    if ticket_draft.description.0.is_empty() {
        return Err(TicketDescriptionError::Empty.into());
    } else if ticket_draft.description.0.len() > TICKET_DESCRIPTION_MAX_LENGTH {
        return Err(TicketDescriptionError::TooLong(TICKET_DESCRIPTION_MAX_LENGTH).into());
    }

    Ok(())
}

pub fn validate_ticket_patch(ticket_patch: &TicketPatch) -> Result<(), AppError> {
    if ticket_patch.title.is_none()
        && ticket_patch.description.is_none()
        && ticket_patch.status.is_none()
    {
        return Err(TicketPatchError::MustContainOneField.into());
    }

    if let Some(title) = &ticket_patch.title {
        if title.0.is_empty() {
            return Err(TicketTitleError::Empty.into());
        } else if title.0.len() > TICKET_TITLE_MAX_LENGTH {
            return Err(TicketTitleError::TooLong(TICKET_TITLE_MAX_LENGTH).into());
        }
    }

    if let Some(description) = &ticket_patch.description {
        if description.0.is_empty() {
            return Err(TicketDescriptionError::Empty.into());
        } else if description.0.len() > TICKET_DESCRIPTION_MAX_LENGTH {
            return Err(TicketDescriptionError::TooLong(TICKET_DESCRIPTION_MAX_LENGTH).into());
        }
    }

    Ok(())
}
