use chrono::prelude::NaiveDate;
use diesel::prelude::*;

use crate::schema::transactions;

#[derive(Queryable)]
pub struct Transaction {
    pub id: i32,
    pub date: NaiveDate,
    pub description: Option<String>,
    pub category: String,
    pub amount: f64,
}

#[derive(Insertable)]
#[diesel(table_name = transactions)]
pub struct NewTransaction<'a> {
    pub date: &'a NaiveDate,
    pub description: Option<&'a String>,
    pub category: &'a String,
    pub amount: &'a f64,
}
