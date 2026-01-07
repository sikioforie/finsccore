use crate::openbank::Transaction;
use serde::{Deserialize, Serialize};
use std::marker::Copy;

// Configuration for our scoring model (Public Input)
#[derive(Clone, Copy, Default, Serialize, Deserialize)]
pub struct ScoringConfig {
    pub model: ScoringModel,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum ScoringModel {
    HeuristicWeighted {
        target_balance: f64, // e.g., 500.0
                             // min_active_days: u32, // e.g., 20
    },
}

impl Default for ScoringModel {
    fn default() -> Self {
        Self::HeuristicWeighted {
            target_balance: 10000.,
            // min_active_days: 20,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CreditScore {
    pub total_score: u8,      // 0-100
    pub risk_level: String,   // "Low", "Medium", "High"
    pub factors: Vec<String>, // ["Consistent Income", "High Gambling Activity"]
}

/// Helper struct to hold our aggregated data
struct AccountSummary {
    total_credit: f64,
    total_debit: f64,
    avg_balance: f64,
    success_rate: f64,
    failed_count: u32,
}

pub fn calculate_heuristic_score(transactions: &[Transaction]) -> f64 {
    if transactions.is_empty() {
        return 0.0;
    }

    // 1. AGGREGATION STEP
    let mut total_credit = 0.0;
    let mut total_debit = 0.0;
    let mut balance_sum = 0.0;
    let mut failed_count = 0;
    let mut success_count = 0;

    for tx in transactions {
        // Track balance history for average calculation
        balance_sum += tx.balance_after;

        if tx.status == "FAILED" {
            failed_count += 1;
            continue; // Skip financial sums for failed transactions
        }

        success_count += 1;

        match tx.debit_credit.as_str() {
            "CREDIT" => total_credit += tx.amount,
            "DEBIT" => total_debit += tx.amount,
            _ => {}
        }
    }

    let count = transactions.len() as f64;
    let avg_balance = balance_sum / count;
    let success_rate = if count > 0.0 {
        success_count as f64 / count
    } else {
        0.0
    };

    // 2. NORMALIZATION STEP (Scoring buckets)
    // We convert raw numbers into a 0-100 score based on "Business Logic"

    // Logic: Income > 10,000 gets max points (Adjust threshold as needed)
    let income_score = (total_credit / 10_000.0 * 100.0).min(100.0);

    // Logic: Avg Balance > 5,000 gets max points
    let liquidity_score = (avg_balance / 5_000.0 * 100.0).min(100.0);

    // Logic: If (Credit - Debit) is positive, 100 points, else 0
    let cashflow_score = if total_credit > total_debit {
        100.0
    } else {
        0.0
    };

    // Logic: Pure percentage (0.95 -> 95 points)
    let reliability_score = success_rate * 100.0;

    // 3. WEIGHTED CALCULATION STEP
    // Weights: Income(0.4), Liquidity(0.3), CashFlow(0.2), Reliability(0.1)
    let weighted_score = (income_score * 0.4)
        + (liquidity_score * 0.3)
        + (cashflow_score * 0.2)
        + (reliability_score * 0.1);

    // 4. PENALTY STEP
    // Deduct 20 points for every failed transaction (severe penalty)
    let penalty = failed_count as f64 * 20.0;

    let final_score = weighted_score - penalty;

    // Ensure score doesn't go below 0
    final_score.max(0.0)
}

// pub fn calculate_recency_score(transactions: &[Transaction]) -> f64 {
//     if transactions.is_empty() {
//         return 0.0;
//     }

//     // 1. DETERMINE "NOW"
//     // In a real app, use Utc::now(). For this example, we find the
//     // latest transaction date to act as "Now" so the math works
//     // regardless of when you run this code.
//     let latest_date = transactions
//         .iter()
//         .filter_map(|t| t.transaction_time.parse::<DateTime<Utc>>().ok())
//         .max()
//         .unwrap_or_else(Utc::now);

//     // Variables for Weighted Aggregation
//     let mut weighted_credit_sum = 0.0;
//     let mut weighted_debit_sum = 0.0;
//     let mut weighted_balance_sum = 0.0;
//     let mut weighted_penalty_score = 0.0;
//     let mut total_weight_accumulated = 0.0;

//     for tx in transactions {
//         // Skip invalid dates or parse errors
//         let tx_date = match tx.transaction_time.parse::<DateTime<Utc>>() {
//             Ok(d) => d,
//             Err(_) => continue,
//         };

//         // 2. CALCULATE DAYS ELAPSED
//         let duration = latest_date.signed_duration_since(tx_date);
//         let days_old = duration.num_days().max(0); // prevent negative days

//         // 3. DEFINE RECENCY WEIGHT (Step Decay Logic)
//         let recency_weight = if days_old <= 30 {
//             1.0 // Fresh data (100% impact)
//         } else if days_old <= 90 {
//             0.5 // Recent history (50% impact)
//         } else {
//             0.2 // Old history (20% impact)
//         };

//         // 4. APPLY WEIGHTS TO AGGREGATES

//         // We sum the weights to calculate the weighted average balance later
//         total_weight_accumulated += recency_weight;

//         // Weighted Balance (Older balances matter less for current liquidity)
//         weighted_balance_sum += tx.balance_after * recency_weight;

//         if tx.status == "FAILED" {
//             // Recent failures hurt A LOT (20 * 1.0 = 20 pts),
//             // Old failures hurt a little (20 * 0.2 = 4 pts)
//             weighted_penalty_score += 20.0 * recency_weight;
//             continue;
//         }

//         match tx.debit_credit.as_str() {
//             // We use the weighted amount to prioritize recent income
//             "CREDIT" => weighted_credit_sum += tx.amount * recency_weight,
//             "DEBIT" => weighted_debit_sum += tx.amount * recency_weight,
//             _ => {}
//         }
//     }

//     // 5. NORMALIZE SCORES (Using Weighted Totals)

//     // Normalize Weighted Average Balance
//     let avg_weighted_balance = if total_weight_accumulated > 0.0 {
//         weighted_balance_sum / total_weight_accumulated
//     } else {
//         0.0
//     };

//     // Logic: Adjusted Income Score (Target: 5000 weighted income units)
//     // Recent income fills this bucket faster than old income.
//     let income_score = (weighted_credit_sum / 5_000.0 * 100.0).min(100.0);

//     // Logic: Liquidity Score based on weighted average
//     let liquidity_score = (avg_weighted_balance / 3_000.0 * 100.0).min(100.0);

//     // Logic: Cashflow (Positive recent flow is better)
//     let cashflow_score = if weighted_credit_sum > weighted_debit_sum {
//         100.0
//     } else {
//         0.0
//     };

//     // 6. FINAL WEIGHTED CALCULATION
//     let final_heuristic = (income_score * 0.4) + (liquidity_score * 0.4) + (cashflow_score * 0.2);

//     // 7. SUBTRACT WEIGHTED PENALTIES
//     let final_score = final_heuristic - weighted_penalty_score;

//     final_score.max(0.0)
// }

#[cfg(test)]
mod test_credit_scoring {
    use super::*;

    #[test]
    fn testing_calculate_heuristic_score() {
        // Example Usage
        let history = vec![
            Transaction {
                id: "1".into(),
                amount: 50000000.0,
                channel: "ATM".into(),
                authorization_token: "x".into(),
                transaction_type: "TRF".into(),
                debit_credit: "CREDIT".into(),
                narration: "Salary".into(),
                reference: "ref1".into(),
                transaction_time: "2023-01-01".into(),
                value_date: "2023-01-01".into(),
                balance_after: 5000.0,
                status: "SUCCESSFUL".into(),
            },
            Transaction {
                id: "8".into(),
                amount: 50000000.0,
                channel: "ATM".into(),
                authorization_token: "x".into(),
                transaction_type: "TRF".into(),
                debit_credit: "CREDIT".into(),
                narration: "Salary".into(),
                reference: "ref1".into(),
                transaction_time: "2023-01-01".into(),
                value_date: "2023-01-01".into(),
                balance_after: 5000.0,
                status: "SUCCESSFUL".into(),
            },
            Transaction {
                id: "8".into(),
                amount: 50000000.0,
                channel: "ATM".into(),
                authorization_token: "x".into(),
                transaction_type: "TRF".into(),
                debit_credit: "CREDIT".into(),
                narration: "Salary".into(),
                reference: "ref1".into(),
                transaction_time: "2023-01-01".into(),
                value_date: "2023-01-01".into(),
                balance_after: 5000.0,
                status: "SUCCESSFUL".into(),
            },
            Transaction {
                id: "8".into(),
                amount: 50000000.0,
                channel: "ATM".into(),
                authorization_token: "x".into(),
                transaction_type: "TRF".into(),
                debit_credit: "CREDIT".into(),
                narration: "Salary".into(),
                reference: "ref1".into(),
                transaction_time: "2023-01-01".into(),
                value_date: "2023-01-01".into(),
                balance_after: 5000.0,
                status: "SUCCESSFUL".into(),
            },
            Transaction {
                id: "8".into(),
                amount: 50000000.0,
                channel: "ATM".into(),
                authorization_token: "x".into(),
                transaction_type: "TRF".into(),
                debit_credit: "CREDIT".into(),
                narration: "Salary".into(),
                reference: "ref1".into(),
                transaction_time: "2023-01-01".into(),
                value_date: "2023-01-01".into(),
                balance_after: 5000.0,
                status: "SUCCESSFUL".into(),
            },
            Transaction {
                id: "8".into(),
                amount: 50000000.0,
                channel: "ATM".into(),
                authorization_token: "x".into(),
                transaction_type: "TRF".into(),
                debit_credit: "CREDIT".into(),
                narration: "Salary".into(),
                reference: "ref1".into(),
                transaction_time: "2023-01-01".into(),
                value_date: "2023-01-01".into(),
                balance_after: 5000.0,
                status: "SUCCESSFUL".into(),
            },
            Transaction {
                id: "8".into(),
                amount: 50000000.0,
                channel: "ATM".into(),
                authorization_token: "x".into(),
                transaction_type: "TRF".into(),
                debit_credit: "CREDIT".into(),
                narration: "Salary".into(),
                reference: "ref1".into(),
                transaction_time: "2023-01-01".into(),
                value_date: "2023-01-01".into(),
                balance_after: 5000.0,
                status: "SUCCESSFUL".into(),
            },
            Transaction {
                id: "8".into(),
                amount: 50000000.0,
                channel: "ATM".into(),
                authorization_token: "x".into(),
                transaction_type: "TRF".into(),
                debit_credit: "CREDIT".into(),
                narration: "Salary".into(),
                reference: "ref1".into(),
                transaction_time: "2023-01-01".into(),
                value_date: "2023-01-01".into(),
                balance_after: 5000.0,
                status: "SUCCESSFUL".into(),
            },
            Transaction {
                id: "8".into(),
                amount: 50000000.0,
                channel: "ATM".into(),
                authorization_token: "x".into(),
                transaction_type: "TRF".into(),
                debit_credit: "CREDIT".into(),
                narration: "Salary".into(),
                reference: "ref1".into(),
                transaction_time: "2023-01-01".into(),
                value_date: "2023-01-01".into(),
                balance_after: 5000.0,
                status: "SUCCESSFUL".into(),
            },
            Transaction {
                id: "1".into(),
                amount: 500000.0,
                channel: "ATM".into(),
                authorization_token: "x".into(),
                transaction_type: "TRF".into(),
                debit_credit: "CREDIT".into(),
                narration: "Salary".into(),
                reference: "ref1".into(),
                transaction_time: "2023-01-01".into(),
                value_date: "2023-01-01".into(),
                balance_after: 5000.0,
                status: "SUCCESSFUL".into(),
            },
            Transaction {
                id: "1".into(),
                amount: 500000.0,
                channel: "ATM".into(),
                authorization_token: "x".into(),
                transaction_type: "TRF".into(),
                debit_credit: "CREDIT".into(),
                narration: "Salary".into(),
                reference: "ref1".into(),
                transaction_time: "2023-01-01".into(),
                value_date: "2023-01-01".into(),
                balance_after: 5000.0,
                status: "SUCCESSFUL".into(),
            },
            Transaction {
                id: "1".into(),
                amount: 500000.0,
                channel: "ATM".into(),
                authorization_token: "x".into(),
                transaction_type: "TRF".into(),
                debit_credit: "CREDIT".into(),
                narration: "Salary".into(),
                reference: "ref1".into(),
                transaction_time: "2023-01-01".into(),
                value_date: "2023-01-01".into(),
                balance_after: 5000.0,
                status: "SUCCESSFUL".into(),
            },
            Transaction {
                id: "1".into(),
                amount: 500000.0,
                channel: "ATM".into(),
                authorization_token: "x".into(),
                transaction_type: "TRF".into(),
                debit_credit: "CREDIT".into(),
                narration: "Salary".into(),
                reference: "ref1".into(),
                transaction_time: "2023-01-01".into(),
                value_date: "2023-01-01".into(),
                balance_after: 5000.0,
                status: "SUCCESSFUL".into(),
            },
            Transaction {
                id: "1".into(),
                amount: 500000.0,
                channel: "ATM".into(),
                authorization_token: "x".into(),
                transaction_type: "TRF".into(),
                debit_credit: "CREDIT".into(),
                narration: "Salary".into(),
                reference: "ref1".into(),
                transaction_time: "2023-01-01".into(),
                value_date: "2023-01-01".into(),
                balance_after: 5000.0,
                status: "SUCCESSFUL".into(),
            },
            Transaction {
                id: "1".into(),
                amount: 500000.0,
                channel: "ATM".into(),
                authorization_token: "x".into(),
                transaction_type: "TRF".into(),
                debit_credit: "CREDIT".into(),
                narration: "Salary".into(),
                reference: "ref1".into(),
                transaction_time: "2023-01-01".into(),
                value_date: "2023-01-01".into(),
                balance_after: 5000.0,
                status: "SUCCESSFUL".into(),
            },
            Transaction {
                id: "1".into(),
                amount: 500000.0,
                channel: "ATM".into(),
                authorization_token: "x".into(),
                transaction_type: "TRF".into(),
                debit_credit: "CREDIT".into(),
                narration: "Salary".into(),
                reference: "ref1".into(),
                transaction_time: "2023-01-01".into(),
                value_date: "2023-01-01".into(),
                balance_after: 5000.0,
                status: "SUCCESSFUL".into(),
            },
            Transaction {
                id: "1".into(),
                amount: 500000.0,
                channel: "ATM".into(),
                authorization_token: "x".into(),
                transaction_type: "TRF".into(),
                debit_credit: "CREDIT".into(),
                narration: "Salary".into(),
                reference: "ref1".into(),
                transaction_time: "2023-01-01".into(),
                value_date: "2023-01-01".into(),
                balance_after: 5000.0,
                status: "SUCCESSFUL".into(),
            },
            Transaction {
                id: "1".into(),
                amount: 500000.0,
                channel: "ATM".into(),
                authorization_token: "x".into(),
                transaction_type: "TRF".into(),
                debit_credit: "CREDIT".into(),
                narration: "Salary".into(),
                reference: "ref1".into(),
                transaction_time: "2023-01-01".into(),
                value_date: "2023-01-01".into(),
                balance_after: 5000.0,
                status: "SUCCESSFUL".into(),
            },
            Transaction {
                id: "1".into(),
                amount: 500000.0,
                channel: "ATM".into(),
                authorization_token: "x".into(),
                transaction_type: "TRF".into(),
                debit_credit: "CREDIT".into(),
                narration: "Salary".into(),
                reference: "ref1".into(),
                transaction_time: "2023-01-01".into(),
                value_date: "2023-01-01".into(),
                balance_after: 5000.0,
                status: "SUCCESSFUL".into(),
            },
            Transaction {
                id: "2".into(),
                amount: 2000.0,
                channel: "ATM".into(),
                authorization_token: "x".into(),
                transaction_type: "WDL".into(),
                debit_credit: "DEBIT".into(),
                narration: "Rent".into(),
                reference: "ref2".into(),
                transaction_time: "2023-01-02".into(),
                value_date: "2023-01-02".into(),
                balance_after: 3000.0,
                status: "SUCCESSFUL".into(),
            },
            Transaction {
                id: "3".into(),
                amount: 2000.0,
                channel: "ATM".into(),
                authorization_token: "x".into(),
                transaction_type: "WDL".into(),
                debit_credit: "DEBIT".into(),
                narration: "Shopping".into(),
                reference: "ref2".into(),
                transaction_time: "2023-01-02".into(),
                value_date: "2023-01-02".into(),
                balance_after: 3000.0,
                status: "SUCCESSFUL".into(),
            },
            Transaction {
                id: "4".into(),
                amount: 100.0,
                channel: "ATM".into(),
                authorization_token: "x".into(),
                transaction_type: "WDL".into(),
                debit_credit: "DEBIT".into(),
                narration: "Fail Test".into(),
                reference: "ref3".into(),
                transaction_time: "2023-01-03".into(),
                value_date: "2023-01-03".into(),
                balance_after: 3000.0,
                status: "FAILED".into(),
            },
            Transaction {
                id: "5".into(),
                amount: 5000000.0,
                channel: "ATM".into(),
                authorization_token: "x".into(),
                transaction_type: "TRF".into(),
                debit_credit: "CREDIT".into(),
                narration: "Salary".into(),
                reference: "ref1".into(),
                transaction_time: "2023-01-01".into(),
                value_date: "2023-01-01".into(),
                balance_after: 5000.0,
                status: "SUCCESSFUL".into(),
            },
            Transaction {
                id: "6".into(),
                amount: 5000000.0,
                channel: "ATM".into(),
                authorization_token: "x".into(),
                transaction_type: "TRF".into(),
                debit_credit: "CREDIT".into(),
                narration: "Salary".into(),
                reference: "ref1".into(),
                transaction_time: "2023-01-01".into(),
                value_date: "2023-01-01".into(),
                balance_after: 5000.0,
                status: "SUCCESSFUL".into(),
            },
        ];

        let score = calculate_heuristic_score(&history);
        println!("Customer Credit Score: {:.2}", score);

        // let score = calculate_recency_score(&history);
        // println!("Customer Credit Score[RecentScoring]: {:.2}", score);
    }
}
