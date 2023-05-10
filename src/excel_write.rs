use crate::excel_read::Excel;
use chrono::{Duration, NaiveDate, NaiveDateTime};
use rust_xlsxwriter::{Workbook, XlsxError, Format};

fn excel_date_to_datetime(excel_date: f64) -> NaiveDateTime {
    let days = excel_date.floor() as i64;
    let seconds = ((excel_date - days as f64) * 86400.0).round() as i64;
    let epoch = NaiveDate::from_ymd_opt(1900, 1, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    epoch + Duration::days(days) + Duration::seconds(seconds)
    
    // let excel_date = 45051.71161416666;
    // let datetime = excel_date_to_datetime(excel_date);
    // println!("{}", datetime.format("%Y-%m-%d %H:%M:%S"));

}

pub fn save_file(
    excel: &Excel,
    item: &Vec<Vec<String>>,
    filename: &String,
) -> Result<(), XlsxError> {
    // Create a new Excel file object.
    let mut workbook = Workbook::new();

    // Add a worksheet to the workbook.
    let worksheet = workbook.add_worksheet();
    let format = Format::new().set_num_format("dd-mm-yyyy hh:mm::ss");

    // Write a string to cell (0, 0) = A1.
    for row in 0..item.len() {
        for col in 0..excel.get_column() {
            if col == 0 && row != 0 {
                let raw = &item[row][col].to_string();
                worksheet.write_number_with_format(row.try_into().unwrap(), col.try_into().unwrap(), raw.parse::<f64>().unwrap(), &format)?;
            } else {
                worksheet.write(
                    row.try_into().unwrap(),
                    col.try_into().unwrap(),
                    &item[row][col],
                )?;
            }
        }
    }

    // Save the file to disk.
    let mut new_file = String::new();
    new_file.push_str("new_");
    new_file.push_str(&filename);
    workbook.save(&new_file).expect("파일 생성에 실패했습니다.");
    println!("변경이 완료되었습니다.");

    Ok(())
}
