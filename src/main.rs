mod utils;

use calamine::{open_workbook, Error, Ods, Reader};


fn main() -> Result<(), Error> {
    let mut budget_wb: Ods<_> = open_workbook("/home/rainman/Documents/Budget.ods")?;

    for sheet in budget_wb.worksheets() {
        for row in sheet.1.rows() {
            println!("{:?}", row);
        }
    }

    Ok(())
}
