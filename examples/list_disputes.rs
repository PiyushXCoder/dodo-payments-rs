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

    // List all disputes
    let response = client
        .list_disputes()
        .page_size(20)
        .page_number(0)
        .send()
        .await;

    match response {
        Ok(disputes) => {
            println!("Found {} disputes", disputes.items.len());
            for dispute in disputes.items {
                println!("- Dispute ID: {}, Payment ID: {}, Status: {:?}, Stage: {:?}", 
                    dispute.dispute_id, dispute.payment_id, dispute.dispute_status, dispute.dispute_stage);
            }
        },
        Err(e) => println!("Failed to list disputes: {:?}", e),
    }

    // List open disputes
    let response2 = client
        .list_disputes()
        .dispute_status("dispute_opened".to_string())
        .page_size(10)
        .send()
        .await;

    match response2 {
        Ok(disputes) => {
            println!("\nFound {} open disputes", disputes.items.len());
            for dispute in disputes.items {
                println!("- Dispute ID: {}, Amount: {}, Created: {}", 
                    dispute.dispute_id, dispute.amount, dispute.created_at);
            }
        },
        Err(e) => println!("Failed to list open disputes: {:?}", e),
    }

    // List disputes for a specific customer within a date range
    let customer_id = "customer_12345".to_string();
    let response3 = client
        .list_disputes()
        .customer_id(customer_id)
        .created_at_gte("2023-11-01T00:00:00Z".to_string())
        .created_at_lte("2023-11-30T23:59:59Z".to_string())
        .send()
        .await;

    match response3 {
        Ok(disputes) => {
            println!("\nFound {} disputes for customer in November", disputes.items.len());
        },
        Err(e) => println!("Failed to list disputes for customer: {:?}", e),
    }
}