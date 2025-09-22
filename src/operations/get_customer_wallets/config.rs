use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct GetCustomerWalletsConfig {
    // customer_id is a path parameter, so no fields here
}
