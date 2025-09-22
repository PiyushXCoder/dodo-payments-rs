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

    // List all payouts
    let response = client
        .list_payouts()
        .page_size(20)
        .page_number(0)
        .send()
        .await;

    match response {
        Ok(payouts) => {
            println!("Found {} payouts", payouts.items.len());
            for payout in payouts.items {
                println!("- Payout ID: {}, Amount: {}, Status: {:?}", 
                    payout.payout_id, payout.amount, payout.status);
            }
        },
        Err(e) => println!("Failed to list payouts: {:?}", e),
    }

    // List payouts with pagination
    let response2 = client
        .list_payouts()
        .page_size(10)
        .page_number(1)
        .send()
        .await;

    match response2 {
        Ok(payouts) => {
            println!("\nFound {} payouts on page 1", payouts.items.len());
            for payout in payouts.items {
                println!("- Payout ID: {}, Payment Method: {}, Created: {}", 
                    payout.payout_id, payout.payment_method, payout.created_at);
                
                if let Some(name) = payout.name {
                    println!("  Name: {}", name);
                }
            }
        },
        Err(e) => println!("Failed to list payouts: {:?}", e),
    }
}