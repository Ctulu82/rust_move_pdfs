//! Release build
//!
//! ```
//! cargo build --release
//! ```

mod destination_map;

use destination_map::get_destination_map;
use std::env;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    // 현재 디렉토리를 기준으로 설정
    let current_dir = env::current_dir()?;
    println!("Current directory: {:?}\n", current_dir);

    // 모듈에서 목적지 디렉토리 맵을 로드
    let destination_map = get_destination_map();

    // 원본 디렉토리 설정 (예: ./pdfs)
    let source_dir = current_dir.join("./Downloads/_DONE/_Magazines");

    // 이동된 파일 개수를 저장할 변수
    let mut moved_count = 0;

    // 원본 디렉토리에서 PDF 파일을 읽어옴
    for entry in fs::read_dir(&source_dir)? {
        let entry = entry?;
        let path = entry.path();

        // 파일이 PDF인지 확인
        if path.is_file() && path.extension().map_or(false, |ext| ext == "pdf") {
            let file_name = path.file_name().unwrap().to_string_lossy();

            // PDF 이름에 맞는 목적지 디렉토리 찾기
            if let Some(dest_dir) = destination_map.iter().find_map(|(prefix, dir)| {
                if file_name.starts_with(prefix) {
                    Some(current_dir.join(dir))
                } else {
                    None
                }
            }) {
                // 목적지 디렉토리가 없다면 생성
                if !dest_dir.exists() {
                    fs::create_dir_all(&dest_dir)?;
                }

                // 파일을 지정된 경로로 이동
                let dest_file = dest_dir.join(entry.file_name());
                fs::rename(&path, &dest_file)?;
                println!("Moved {:?} \n   to {:?}\n", file_name, dest_dir);

                // 이동 성공 시 카운트 증가
                moved_count += 1;
            }
        }
    }

    // 전체 이동된 파일 개수 출력
    println!("총 이동된 PDF 파일 개수: {}", moved_count);

    Ok(())
}
