mod excel_read;
mod excel_write;
mod number_convert;

use excel_read::Excel;
use excel_write::save_file;
use std::env;

/* TODO
* 파일 이름 입력 받기
* 열 위치 확인하기
* 최종 모습 선택하기 (나중에))
* 새로운 파일로 생성하기
** 파일을 읽어서 그대로 새로운 파일로 저장하기.(Sheet1만)
*/

fn main() {
    /* [1] 파일 이름, [2] 연락처 열 */
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        println!("파일명과 연락처 열을 입력해 주세요.");
        return;
    }

    let path = &args[1];
    let target: usize = args[2].parse().unwrap();

    match Excel::new(path, target) {
        Ok(mut excel) => {
            // do something with the Excel instance
            excel.init();
            let converted_item = excel.convert_item();
            match save_file(&excel, &converted_item, path) {
                Err(e) => println!("{:?}", e),
                _ => (),
            }
        }
        Err(_err) => {
            // handle the error
            println!(
                "파일을 열 수 없습니다. 이름을 확인해 주세요. [입력한 파일 이름 : {:?}] ",
                path
            );
        }
    }
}
