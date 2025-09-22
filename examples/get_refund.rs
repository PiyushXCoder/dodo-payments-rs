use dodo_payments::{DodoPayments, client::DodoPaymentsConfigBuilder};
use std::env;

#[tokio::main]
async fn main() {
    let bearer_token = env::var("DODO_PAYMENTS_BEARER_TOKEN")
        .expect("DODO_PAYMENTS_BEARER_TOKEN must be set in env variables");
        
    let refund_id = env::var("DODO_PAYMENTS_REFUND_ID")
        .expect("DODO_PAYMENTS_REFUND_ID must be set in env variables");

    let client = DodoPayments::new(
        DodoPaymentsConfigBuilder::new()
            .bearer_token(&bearer_token)
            .environment("test_mode"),
    );

    let response = client
        .get_refund(refund_id)
        .send()
        .await;

    match response {
        Ok(refund) => {
            println!("Refund details:");
            println!("Refund ID: {}", refund.refund_id);
            println!("Payment ID: {}", refund.payment_id);
            println!("Business ID: {}", refund.business_id);
            println!("Status: {:?}", refund.status);
            println!("Created At: {}", refund.created_at);
            println!("Is Partial: {}", refund.is_partial);
            
            if let Some(amount) = refund.amount {
                println!("Amount: {}", amount);
            }
            
            if let Some(reason) = refund.reason {
                println!("Reason: {}", reason);
            }
            
            println!("Customer: {} ({})", refund.customer.name, refund.customer.email);
        },
        Err(e) => println!("Failed to get refund details: {:?}", e),
    }
}