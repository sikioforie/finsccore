use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Config {}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsResponse {
    pub status: String,  // "00",
    pub message: String, // "The process was completed successully",
    pub data: TransactionsData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsData {
    pub summary: Summary,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,                  // "1234567890",
    pub amount: f64,                 // 1000.24,
    pub channel: String,             // "ATM",
    pub authorization_token: String, // "CARD",
    pub transaction_type: String,    // "WITHDRAWAL",
    pub debit_credit: String,        // "DEBIT",
    pub narration: String,           // "ATM Withdrawal/Karaole LANG",
    pub reference: String,           // "WDS12345678909987",
    pub transaction_time: String,    // "2019-01-02T19:58:47.1234567",
    pub value_date: String,          // "2019-01-02",
    pub balance_after: f64,          // 1200,
    pub status: String,              // "SUCESSFUL | FAILED"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Summary {
    account_number: String,    // "0123456789",
    currency_code: String,     // "NGN",
    from: String,              // "2022-01-01",
    to: String,                // "2022-07-31",
    first_transaction: String, // "2022-01-13",
    last_transaction: String,  // "2022-06-27",
    opening_balance: f64,      // 10000,
    closing_balance: f64,      // 54444,
    total_debit_count: u32,    // 6,
    total_credit_count: u32,   // 56789,
    total_debit_value: f64,    // 27000.87,
    total_credit_value: f64,   // 5000.5,
    pages: u32,                // 15,
    records_per_page: u32,     // 100
}

#[derive(Debug, Serialize, Deserialize)]
struct CustomerProperty {
    id: String,          // "Some random ID",
    description: String, // "Some random text",
    r#type: String,      // "Some random type",
    value: String,       // "Some value"
}
