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

    // First create a brand to update
    let create_response = client
        .create_brand()
        .name("Original Brand Name".to_string())
        .statement_descriptor("Original Descriptor".to_string())
        .support_email("original@example.com".to_string())
        .send()
        .await;

    match create_response {
        Ok(brand) => {
            println!("Brand created successfully!");
            println!("Brand ID: {}", brand.brand_id);
            println!("Original Name: {:?}", brand.name);
            println!("Original Statement Descriptor: {}", brand.statement_descriptor);
            println!("Original Support Email: {:?}", brand.support_email);
            
            // Now update the brand
            let update_response = client
                .update_brand(brand.brand_id.clone())
                .name("Updated Brand Name".to_string())
                .statement_descriptor("Updated Descriptor".to_string())
                .support_email("updated@example.com".to_string())
                .send()
                .await;
                
            match update_response {
                Ok(updated_brand) => {
                    println!("\nBrand updated successfully!");
                    println!("Brand ID: {}", updated_brand.brand_id);
                    println!("Updated Name: {:?}", updated_brand.name);
                    println!("Updated Statement Descriptor: {}", updated_brand.statement_descriptor);
                    println!("Updated Support Email: {:?}", updated_brand.support_email);
                    println!("Verification Status: {:?}", updated_brand.verification_status);
                    println!("Enabled: {}", updated_brand.enabled);
                },
                Err(e) => println!("Failed to update brand: {:?}", e),
            }
        },
        Err(e) => println!("Failed to create brand: {:?}", e),
    }
}