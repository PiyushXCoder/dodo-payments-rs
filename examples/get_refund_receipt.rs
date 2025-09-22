use dodo_payments::{DodoPayments, client::DodoPaymentsConfigBuilder};
use std::{env, fs::File};
use std::io::Write;

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
        .get_refund_receipt(refund_id)
        .send()
        .await;

    match response {
        Ok(pdf_data) => {
            // Save the PDF to a file
            let filename = format!("refund_receipt_{}.pdf", 
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs());
            
            let mut file = File::create(&filename).expect("Failed to create file");
            file.write_all(&pdf_data).expect("Failed to write PDF data to file");
            
            println!("Refund receipt saved to: {}", filename);
            println!("File size: {} bytes", pdf_data.len());
        },
        Err(e) => println!("Failed to get refund receipt: {:?}", e),
    }
}