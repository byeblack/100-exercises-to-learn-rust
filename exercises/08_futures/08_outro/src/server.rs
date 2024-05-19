use std::sync::OnceLock;

use crate::{
    data::{validate_ticket_draft, validate_ticket_patch, TicketPatch},
    error::{AppError, AppResult, ServerError},
    store,
};

use salvo::prelude::*;
use serde_json::json;
use tokio::sync::RwLock;

pub static TICKET_STORE: OnceLock<RwLock<store::TicketStore>> = OnceLock::new();

#[handler]
pub async fn get(res: &mut Response, req: &mut Request) -> AppResult<()> {
    let id = req
        .param::<u64>("id")
        .ok_or_else(|| AppError::InvalidTicketId)?;

    let store = TICKET_STORE
        .get()
        .ok_or_else(|| AppError::TicketStoreNotInitialized)?;

    let ticket = store
        .read()
        .await
        .get(store::TicketId(id))
        .ok_or_else(|| AppError::NotTicket)?;

    let data = ticket.read().await.to_owned();

    res.render(Json(&data));

    Ok(())
}

#[handler]
pub async fn patch(res: &mut Response, req: &mut Request) -> AppResult<()> {
    let id = req
        .param::<u64>("id")
        .ok_or_else(|| AppError::InvalidTicketId)?;

    let req_data: TicketPatch = req.parse_json().await?;

    validate_ticket_patch(&req_data)?;

    let store = TICKET_STORE
        .get()
        .ok_or_else(|| AppError::TicketStoreNotInitialized)?;

    let ticket = store
        .read()
        .await
        .get(store::TicketId(id))
        .ok_or_else(|| AppError::NotTicket)?;

    let data = {
        let mut data = ticket.write().await;

        if let Some(title) = req_data.title {
            data.title = title;
        }
        if let Some(description) = req_data.description {
            data.description = description;
        }
        if let Some(status) = req_data.status {
            data.status = status;
        }

        data.to_owned()
    };
    res.render(Json(&data));

    Ok(())
}

#[handler]
pub async fn create(res: &mut Response, req: &mut Request) -> AppResult<()> {
    let req_data = req.parse_json().await?;

    validate_ticket_draft(&req_data)?;

    let store = TICKET_STORE
        .get()
        .ok_or_else(|| AppError::TicketStoreNotInitialized)?;

    let id = { store.write().await.add_ticket(req_data) };

    res.render(Json(json!({
        "id": id.0,
    })));

    Ok(())
}

pub async fn run(local_addr: &str) -> Result<(), ServerError> {
    let store = store::TicketStore::new();
    TICKET_STORE.get_or_init(|| RwLock::new(store));

    let router = Router::with_path("/api/ticket")
        .post(create)
        .push(Router::new().path("/<id>").get(get).patch(patch));

    let acceptor = TcpListener::new(local_addr).try_bind().await?;
    Server::new(acceptor).try_serve(router).await?;

    Ok(())
}
