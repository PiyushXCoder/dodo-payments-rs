use dodo_payments::{
    DodoPayments, 
    client::DodoPaymentsConfigBuilder,
    operations::common::structs::PartialRefundItem
};
use std::env;

#[tokio::main]
async fn main() {
    let bearer_token = env::var("DODO_PAYMENTS_BEARER_TOKEN")
        .expect("DODO_PAYMENTS_BEARER_TOKEN must be set in env variables");
        
    let payment_id = env::var("DODO_PAYMENTS_PAYMENT_ID")
        .expect("DODO_PAYMENTS_PAYMENT_ID must be set in env variables");

    let client = DodoPayments::new(
        DodoPaymentsConfigBuilder::new()
            .bearer_token(&bearer_token)
            .environment("test_mode"),
    );

    // Create a full refund
    let response = client
        .create_refund(payment_id.clone())
        .reason("Customer requested refund".to_string())
        .send()
        .await;

    match response {
        Ok(refund) => {
            println!("Refund created successfully:");
            println!("Refund ID: {}", refund.refund_id);
            println!("Payment ID: {}", refund.payment_id);
            println!("Status: {:?}", refund.status);
            println!("Is Partial: {}", refund.is_partial);
            
            if let Some(amount) = refund.amount {
                println!("Amount: {}", amount);
            }
        },
        Err(e) => println!("Failed to create refund: {:?}", e),
    }

    // Create a partial refund for specific items
    let items = vec![
        PartialRefundItem {
            item_id: "product_123".to_string(),
            amount: Some(500), // $5.00
            tax_inclusive: Some(true),
        },
        PartialRefundItem {
            item_id: "addon_456".to_string(),
            amount: Some(250), // $2.50
            tax_inclusive: Some(true),
        }
    ];

    let response2 = client
        .create_refund(payment_id)
        .items(items)
        .reason("Partial refund for specific items".to_string())
        .send()
        .await;

    match response2 {
        Ok(refund) => {
            println!("\nPartial refund created successfully:");
            println!("Refund ID: {}", refund.refund_id);
            println!("Status: {:?}", refund.status);
            println!("Is Partial: {}", refund.is_partial);
        },
        Err(e) => println!("Failed to create partial refund: {:?}", e),
    }
}