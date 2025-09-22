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

    // First create a brand to get its ID
    let create_response = client
        .create_brand()
        .name("Test Brand for Get Operation".to_string())
        .description("This is a test brand for demonstrating the get_brand operation".to_string())
        .statement_descriptor("Test Brand Store".to_string())
        .support_email("support@testbrand.com".to_string())
        .url("https://www.testbrand.com".to_string())
        .send()
        .await;

    match create_response {
        Ok(brand) => {
            println!("Brand created successfully!");
            println!("Brand ID: {}", brand.brand_id);
            
            // Now get the brand we just created
            let get_response = client
                .get_brand(brand.brand_id.clone())
                .send()
                .await;
                
            match get_response {
                Ok(retrieved_brand) => {
                    println!("\nBrand retrieved successfully!");
                    println!("Brand ID: {}", retrieved_brand.brand_id);
                    println!("Business ID: {}", retrieved_brand.business_id);
                    println!("Name: {:?}", retrieved_brand.name);
                    println!("Statement Descriptor: {}", retrieved_brand.statement_descriptor);
                    println!("Verification Status: {:?}", retrieved_brand.verification_status);
                    println!("Enabled: {}", retrieved_brand.enabled);
                    
                    if let Some(description) = retrieved_brand.description {
                        println!("Description: {}", description);
                    }
                    
                    if let Some(support_email) = retrieved_brand.support_email {
                        println!("Support Email: {}", support_email);
                    }
                    
                    if let Some(url) = retrieved_brand.url {
                        println!("URL: {}", url);
                    }
                },
                Err(e) => println!("Failed to retrieve brand: {:?}", e),
            }
        },
        Err(e) => println!("Failed to create brand: {:?}", e),
    }
}