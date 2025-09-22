use dodo_payments::{
    DodoPayments, 
    client::DodoPaymentsConfigBuilder,
    operations::common::structs::{EventInput, EventMetadata}
};
use serde_json::Value;
use std::{env, collections::HashMap};

#[tokio::main]
async fn main() {
    let bearer_token = env::var("DODO_PAYMENTS_BEARER_TOKEN")
        .expect("DODO_PAYMENTS_BEARER_TOKEN must be set in env variables");

    let client = DodoPayments::new(
        DodoPaymentsConfigBuilder::new()
            .bearer_token(&bearer_token)
            .environment("test_mode"),
    );

    // Create a simple event
    let event1 = EventInput {
        event_id: "event_12345".to_string(),
        customer_id: "customer_67890".to_string(),
        event_name: "api_request".to_string(),
        metadata: None,
        timestamp: None,
    };

    // Create an event with metadata
    let mut metadata_map = HashMap::new();
    metadata_map.insert("user_agent".to_string(), Value::String("Mozilla/5.0".to_string()));
    metadata_map.insert("response_time_ms".to_string(), Value::Number(serde_json::Number::from(150)));
    metadata_map.insert("success".to_string(), Value::Bool(true));
    
    let event2 = EventInput {
        event_id: "event_67890".to_string(),
        customer_id: "customer_12345".to_string(),
        event_name: "payment_processed".to_string(),
        metadata: Some(EventMetadata(metadata_map)),
        timestamp: Some("2023-11-07T05:31:56Z".to_string()),
    };

    let events = vec![event1, event2];

    let response = client
        .ingest_events(events)
        .send()
        .await;

    match response {
        Ok(resp) => println!("Successfully ingested {} events", resp.ingested_count),
        Err(e) => println!("Failed to ingest events: {:?}", e),
    }
}