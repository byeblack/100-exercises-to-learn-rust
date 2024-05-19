// This is our last exercise. Let's go down a more unstructured path!
// Try writing an **asynchronous REST API** to expose the functionality
// of the ticket management system we built throughout the course.
// It should expose endpoints to:
//  - Create a ticket
//  - Retrieve ticket details
//  - Patch a ticket
//
// Use Rust's package registry, crates.io, to find the dependencies you need
// (if any) to build this system.

pub mod data;
pub mod error;
pub mod server;
pub mod store;

#[cfg(test)]
mod test {
    use crate::*;
    use serde::{Deserialize, Serialize};

    const LOCAL_ADDR: &str = "127.0.0.1:5800";

    #[derive(Debug, Serialize, Deserialize)]
    struct CreateResponse {
        id: u64,
    }

    #[tokio::test]
    async fn test_serve() {
        let server_handle = tokio::spawn(server::run(LOCAL_ADDR));

        let base_url: reqwest::Url = format!("http://{LOCAL_ADDR}/api/ticket").parse().unwrap();
        let client = reqwest::Client::new();

        let src_data = r#"{ "title": "Test Title", "description": "Test Description" }"#;

        let res = client
            .post(base_url.clone())
            .body(src_data)
            .header("Content-Type", "application/json")
            .send()
            .await
            .unwrap();

        let create_result_data: CreateResponse = res.json().await.unwrap();

        let res = client
            .get(
                base_url
                    .join(&format!("ticket/{}", create_result_data.id))
                    .unwrap(),
            )
            .header("Content-Type", "application/json")
            .send()
            .await
            .unwrap();

        let data: data::Ticket = res.json().await.unwrap();
        assert_eq!(data.title, "Test Title");
        assert_eq!(data.description, "Test Description");

        let res = client
            .patch(
                base_url
                    .join(&format!("ticket/{}", create_result_data.id))
                    .unwrap(),
            )
            .body(r#"{ "status": "InProgress" }"#)
            .header("Content-Type", "application/json")
            .send()
            .await
            .unwrap();

        let data: data::Ticket = res.json().await.unwrap();
        assert_eq!(data.status, data::Status::InProgress);

        let res = client
            .patch(
                base_url
                    .join(&format!("ticket/{}", create_result_data.id))
                    .unwrap(),
            )
            .body(r#"{ "title": "I'm ready to learn about futures!I'm ready to learn about futures!" }"#)
            .header("Content-Type", "application/json")
            .send()
            .await
            .unwrap();

        let text_data = res.text().await.unwrap();
        assert_eq!(
            text_data,
            r#"{"error":"Ticket title error: The title cannot be longer than 50 characters"}"#
        );

        server_handle.abort();
        let result = server_handle.await;
        println!("{:?}", result);
        assert!(result.is_err());
    }
}
