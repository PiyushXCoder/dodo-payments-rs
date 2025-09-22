use dodo_payments::{
    DodoPayments, 
    client::DodoPaymentsConfigBuilder,
    operations::common::structs::{
        MeterAggregation, 
        AggregationType,
        MeterFilter,
        MeterFilterCondition,
        FilterOperator,
        Conjunction
    }
};
use serde_json::Value;
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

    // Create a simple count aggregation meter
    let aggregation = MeterAggregation {
        aggregation_type: AggregationType::Count,
        key: None,
    };

    let response = client
        .create_meter(
            "API Requests".to_string(),
            "api_request".to_string(),
            aggregation,
            "requests".to_string(),
        )
        .description("Track API requests".to_string())
        .send()
        .await;

    println!("Response: {:#?}", response);
    
    // Create a more complex meter with sum aggregation and a filter
    let sum_aggregation = MeterAggregation {
        aggregation_type: AggregationType::Sum,
        key: Some("amount".to_string()),
    };
    
    // Note: For complex filters, you would need to construct the JSON structure manually
    // This is a simplified example showing the basic structure
    let filter_json = serde_json::json!({
        "conjunction": "and",
        "clauses": [
            {
                "key": "user_type",
                "operator": "equals",
                "value": "premium"
            }
        ]
    });
    
    let filter = MeterFilter {
        conjunction: Conjunction::And,
        clauses: vec![filter_json],
    };
    
    let response2 = client
        .create_meter(
            "Premium Revenue".to_string(),
            "payment_processed".to_string(),
            sum_aggregation,
            "usd_cents".to_string(),
        )
        .description("Track revenue from premium users".to_string())
        .filter(filter)
        .send()
        .await;

    println!("Response 2: {:#?}", response2);
}