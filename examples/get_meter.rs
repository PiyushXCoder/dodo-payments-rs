use dodo_payments::{DodoPayments, client::DodoPaymentsConfigBuilder};
use std::env;

#[tokio::main]
async fn main() {
    let bearer_token = env::var("DODO_PAYMENTS_BEARER_TOKEN")
        .expect("DODO_PAYMENTS_BEARER_TOKEN must be set in env variables");
        
    let meter_id = env::var("DODO_PAYMENTS_METER_ID")
        .expect("DODO_PAYMENTS_METER_ID must be set in env variables");

    let client = DodoPayments::new(
        DodoPaymentsConfigBuilder::new()
            .bearer_token(&bearer_token)
            .environment("test_mode"),
    );

    let response = client
        .get_meter(meter_id)
        .send()
        .await;

    println!("Response: {:#?}", response);
    
    if let Ok(resp) = response {
        println!("Meter: {} ({})", resp.name, resp.id);
        println!("Event: {}", resp.event_name);
        println!("Measurement Unit: {}", resp.measurement_unit);
        println!("Aggregation Type: {:?}", resp.aggregation.aggregation_type);
    }
}