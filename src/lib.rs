use calamine::DataType;
use chrono::NaiveDate;
use diesel::{PgConnection, RunQueryDsl};

use self::models::{NewTransaction, Transaction};

pub mod models;
pub mod schema;

pub fn create_transaction(
    connection: &mut PgConnection,
    date: &NaiveDate,
    description: &Option<&String>,
    category: &String,
    amount: &f64,
) -> Transaction {
    use crate::schema::transactions;

    let new_transaction = NewTransaction {
        date,
        description: description.as_deref(),
        category,
        amount,
    };

    diesel::insert_into(transactions::table)
        .values(&new_transaction)
        .get_result(connection)
        .expect("Error saving new transaction")
}

pub fn parse_transaction_row(
    row: Option<&[DataType]>,
) -> Option<(NaiveDate, Option<String>, String, f64)> {
    let cropped_row = &row.unwrap()[..5];
    if cropped_row.iter().all(|x| x == &DataType::Empty) {
        return None;
    }

    let date = match cropped_row[0].get_string() {
        Some(date_str) => match multi_format_date_parse(date_str) {
            Ok(parsed_date) => parsed_date,
            Err(_) => {
                println!("Date not found in row: {:?}", &cropped_row);
                return None;
            }
        },
        None => {
            println!("Date not found in row: {:?}", &cropped_row);
            return None;
        }
    };
    let description = match cropped_row[1].get_string() {
        Some(desc) => Some(desc.to_owned()),
        None => None,
    };
    let category = match cropped_row[2].get_string() {
        Some(cat) => cat.to_owned(),
        None => {
            println!("Category not found in row: {:?}", &cropped_row);
            return None;
        }
    };
    let amount = match cropped_row[3].get_float() {
        Some(amt) => amt,
        None => match cropped_row[4].get_float() {
            Some(amt) => amt,
            None => {
                println!("No amount found in row: {:?}", &cropped_row);
                return None;
            }
        },
    };

    Some((date, description, category, amount))
}

fn multi_format_date_parse(date_str: &str) -> Result<NaiveDate, String> {
    for format in ["%Y-%m-%d", "%m/%d/%y", "%m/%d/%Y"] {
        match NaiveDate::parse_from_str(date_str, format) {
            Ok(date) => return Ok(date),
            Err(_) => continue,
        }
    }

    Err(String::from(format!(
        "Couldn't parse date from data {}",
        date_str
    )))
}
