use dodo_payments::{DodoPayments, client::DodoPaymentsConfigBuilder};
use std::env;

#[tokio::main]
async fn main() {
    let bearer_token = env::var("DODO_PAYMENTS_BEARER_TOKEN")
        .expect("DODO_PAYMENTS_BEARER_TOKEN must be set in env variables");
        
    let dispute_id = env::var("DODO_PAYMENTS_DISPUTE_ID")
        .expect("DODO_PAYMENTS_DISPUTE_ID must be set in env variables");

    let client = DodoPayments::new(
        DodoPaymentsConfigBuilder::new()
            .bearer_token(&bearer_token)
            .environment("test_mode"),
    );

    let response = client
        .get_dispute(dispute_id)
        .send()
        .await;

    match response {
        Ok(dispute) => {
            println!("Dispute details:");
            println!("Dispute ID: {}", dispute.dispute_id);
            println!("Payment ID: {}", dispute.payment_id);
            println!("Business ID: {}", dispute.business_id);
            println!("Amount: {}", dispute.amount);
            println!("Currency: {}", dispute.currency);
            println!("Status: {:?}", dispute.dispute_status);
            println!("Stage: {:?}", dispute.dispute_stage);
            println!("Created At: {}", dispute.created_at);
            println!("Customer: {} ({})", dispute.customer.name, dispute.customer.email);
            
            if let Some(reason) = dispute.reason {
                println!("Reason: {}", reason);
            }
            
            if let Some(remarks) = dispute.remarks {
                println!("Remarks: {}", remarks);
            }
        },
        Err(e) => println!("Failed to get dispute details: {:?}", e),
    }
}