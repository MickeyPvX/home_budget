use chrono::prelude::{Date, Utc};


pub struct BudgetRow {
    date: Date<Utc>,
    description: String,
    category: String,
    amount: f64,
}
