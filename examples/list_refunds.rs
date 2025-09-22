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

    // List all refunds
    let response = client
        .list_refunds()
        .page_size(20)
        .page_number(0)
        .send()
        .await;

    match response {
        Ok(refunds) => {
            println!("Found {} refunds", refunds.items.len());
            for refund in refunds.items {
                println!("- Refund ID: {}, Payment ID: {}, Status: {:?}", 
                    refund.refund_id, refund.payment_id, refund.status);
            }
        },
        Err(e) => println!("Failed to list refunds: {:?}", e),
    }

    // List succeeded refunds
    let response2 = client
        .list_refunds()
        .status("succeeded".to_string())
        .page_size(10)
        .send()
        .await;

    match response2 {
        Ok(refunds) => {
            println!("\nFound {} succeeded refunds", refunds.items.len());
            for refund in refunds.items {
                if let Some(amount) = refund.amount {
                    println!("- Refund ID: {}, Amount: {}, Created: {}", 
                        refund.refund_id, amount, refund.created_at);
                }
            }
        },
        Err(e) => println!("Failed to list succeeded refunds: {:?}", e),
    }

    // List refunds for a specific customer within a date range
    let customer_id = "customer_12345".to_string();
    let response3 = client
        .list_refunds()
        .customer_id(customer_id)
        .created_at_gte("2023-11-01T00:00:00Z".to_string())
        .created_at_lte("2023-11-30T23:59:59Z".to_string())
        .send()
        .await;

    match response3 {
        Ok(refunds) => {
            println!("\nFound {} refunds for customer in November", refunds.items.len());
        },
        Err(e) => println!("Failed to list refunds for customer: {:?}", e),
    }
}