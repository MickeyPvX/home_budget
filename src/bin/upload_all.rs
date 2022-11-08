use std::env;

use calamine::{open_workbook, Ods, Reader};
use chrono::NaiveDate;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;

use home_budget::*;

fn main() {
    // use self::schema::transactions::dsl::*;

    let mut budget_wb: Ods<_> = open_workbook("/home/rainman/Documents/Budget.ods").unwrap();
    let connection = &mut establish_pg_connection();

    for worksheet in budget_wb.worksheets() {
        // Turn Worksheet names into parseable dates - skips sheets not used for budget
        let mut ws_date = str::replace(worksheet.0.as_str(), ' ', "-");
        ws_date.push_str("-01");

        let range = match NaiveDate::parse_from_str(&ws_date, "%B-%Y-%d") {
            Ok(_) => worksheet.1,
            Err(e) => {
                println!("Worksheet '{}' skipped - {:?}", ws_date, e);
                continue;
            }
        };

        println!("Working on Worksheet - {}", worksheet.0);

        // Skip first row of each Worksheet
        let mut rows = range.rows();
        rows.next();

        for row in rows {
            let transaction_option = parse_transaction_row(Some(row));

            if transaction_option != None {
                let parsed_transaction = transaction_option.unwrap();
                create_transaction(
                    connection,
                    &parsed_transaction.0,
                    &parsed_transaction.1.as_ref(),
                    &parsed_transaction.2,
                    &parsed_transaction.3,
                );
            }
        }
    }
}

pub fn establish_pg_connection() -> PgConnection {
    dotenv().ok();

    let db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");

    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}
