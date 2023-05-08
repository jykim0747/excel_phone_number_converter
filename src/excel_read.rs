use std::{fs::File, io::BufReader, path::Path};

use calamine::{open_workbook, DataType, Error, RangeDeserializerBuilder, Reader, Xlsx};

use crate::number_convert::convert_number;

/*
 * sheet를 읽는다.
 * 연락처 컬럼을 확인한다.
 * 수정한다.
 * 파일을 복사한다. (기존 데이터 + 수정된 데이터)
 */
pub struct Excel {
    excel: Xlsx<BufReader<File>>,
    sheet: String,
    row: usize,
    col: usize,
    target: usize,
}

impl Excel {
    pub fn new<P: AsRef<Path>>(path: P, target: usize) -> Result<Self, Error> {
        let workbook: Xlsx<_> = open_workbook(&path)?;
        Ok(Excel {
            excel: workbook,
            sheet: String::new(),
            row: 0,
            col: 0,
            target,
        })
    }

    pub fn get_column(&self) -> usize {
        self.col
    }

    pub fn convert_item(&mut self) -> Vec<Vec<String>> {
        let range = self
            .excel
            .worksheet_range(&self.sheet)
            .ok_or(Error::Msg("첫 번째 시트를 열 수 없습니다."))
            .unwrap()
            .unwrap();

        let mut header: Vec<String> = Vec::new();
        if let Some(first_row) = range.rows().next() {
            for cell in first_row.iter() {
                match cell {
                    DataType::String(s) => header.push(s.to_string()),
                    _ => println!("Unknown type"),
                }
            }
        }
        let mut iter = RangeDeserializerBuilder::new().from_range(&range).unwrap();

        let mut changed: Vec<Vec<String>> = Vec::new();
        changed.push(header);
        while let Some(result) = iter.next() {
            let mut v: Vec<String> = result.unwrap();
            v[self.target] = convert_number(&v[self.target]);
            changed.push(v);
        }
        changed
    }

    pub fn init(&mut self) {
        self.sheet = self.excel.sheet_names().get(0).unwrap().clone();
        /* 첫 번째 Sheet를 읽는다.(추후 수정) */
        if let Some(Ok(range)) = self.excel.worksheet_range(&self.sheet) {
            self.row = range.get_size().0;
            self.col = range.get_size().1;

            let total_cells = self.row * self.col;
            let non_empty_cells: usize = range.used_cells().count();
            println!(
                "Found {} rows, {} columns, total {} cells in '{}', including {} non empty cells",
                self.row, self.col, self.sheet, total_cells, non_empty_cells
            );
            // alternatively, we can manually filter rows
            assert_eq!(
                non_empty_cells,
                range
                    .rows()
                    .flat_map(|r| r.iter().filter(|&c| c != &DataType::Empty))
                    .count()
            );

            if self.target >= self.col {
                println!(
                    "연락처 열을 정확히 입력해 주세요 [입력한 열 : {:?}, 전체 열 : {:?}]",
                    self.target, self.col
                );
            }
        }
    }
}
