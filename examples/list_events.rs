use dodo_payments::{DodoPayments, client::DodoPaymentsConfigBuilder};
use std::env;

#[tokio::main]
async fn main() {
    let bearer_token = env::var("DODO_PAYMENTS_BEARER_TOKEN")
        .expect("DODO_PAYMENTS_BEARER_TOKEN must be set in env variables");

    let client = DodoPayments::new(
        DodoPaymentsConfigBuilder::new()
            .bearer_token(&bearer_token)
            .environment("test_mode"),
    );

    // List all events
    let response = client
        .list_events()
        .page_size(20)
        .page_number(0)
        .send()
        .await;

    match response {
        Ok(events) => {
            println!("Found {} events", events.items.len());
            for event in events.items {
                println!("- Event ID: {}, Name: {}, Customer: {}", 
                    event.event_id, event.event_name, event.customer_id);
            }
        },
        Err(e) => println!("Failed to list events: {:?}", e),
    }

    // List events for a specific customer
    let customer_id = "customer_12345".to_string();
    let response2 = client
        .list_events()
        .customer_id(customer_id)
        .page_size(10)
        .send()
        .await;

    match response2 {
        Ok(events) => {
            println!("\nFound {} events for customer", events.items.len());
            for event in events.items {
                println!("- Event ID: {}, Name: {}, Timestamp: {}", 
                    event.event_id, event.event_name, event.timestamp);
            }
        },
        Err(e) => println!("Failed to list events for customer: {:?}", e),
    }

    // List events within a specific time range
    let response3 = client
        .list_events()
        .start("2023-11-01T00:00:00Z".to_string())
        .end("2023-11-30T23:59:59Z".to_string())
        .send()
        .await;

    match response3 {
        Ok(events) => {
            println!("\nFound {} events in November", events.items.len());
        },
        Err(e) => println!("Failed to list events in time range: {:?}", e),
    }
}