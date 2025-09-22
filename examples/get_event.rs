use dodo_payments::{DodoPayments, client::DodoPaymentsConfigBuilder};
use std::env;

#[tokio::main]
async fn main() {
    let bearer_token = env::var("DODO_PAYMENTS_BEARER_TOKEN")
        .expect("DODO_PAYMENTS_BEARER_TOKEN must be set in env variables");
        
    let event_id = env::var("DODO_PAYMENTS_EVENT_ID")
        .expect("DODO_PAYMENTS_EVENT_ID must be set in env variables");

    let client = DodoPayments::new(
        DodoPaymentsConfigBuilder::new()
            .bearer_token(&bearer_token)
            .environment("test_mode"),
    );

    let response = client
        .get_event(event_id)
        .send()
        .await;

    match response {
        Ok(event) => {
            println!("Event retrieved successfully:");
            println!("Event ID: {}", event.event_id);
            println!("Customer ID: {}", event.customer_id);
            println!("Event Name: {}", event.event_name);
            println!("Timestamp: {}", event.timestamp);
            
            if let Some(metadata) = event.metadata {
                println!("Metadata: {:?}", metadata.0);
            } else {
                println!("No metadata");
            }
        },
        Err(e) => println!("Failed to retrieve event: {:?}", e),
    }
}