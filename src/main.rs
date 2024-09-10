use std::env;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    // 현재 실행 중인 프로그램의 경로를 기준으로 설정
    let current_dir = env::current_dir()?;
    println!("Current directory: {:?}", current_dir);

    // 원본 디렉토리: 현재 디렉토리의 상대 경로
    let source_dir = current_dir.join("./");

    // 목적지 디렉토리: 동적으로 결정
    let mut dest_dir = current_dir.join("./");

    // 디렉토리가 존재하는지 확인하고 없으면 생성
    if !dest_dir.exists() {
        fs::create_dir_all(&dest_dir)?;
    }

    // 원본 디렉토리에서 PDF 파일들을 읽어옵니다.
    for entry in fs::read_dir(&source_dir)? {
        let entry = entry?;
        let path = entry.path();

        // 파일이 PDF인지 확인
        if path.is_file() && path.extension().map_or(false, |ext| ext == "pdf") {
            let file_name = path.file_name().unwrap().to_string_lossy();

            if file_name.starts_with("25 Beautiful Homes ") {
                dest_dir = current_dir.join("./__Future Plc/25 Beautiful Homes");
            } else if file_name.starts_with("TypeB_") {
                dest_dir = current_dir.join("./sorted/type_b");
            } else {
                // 해당하지 않으면 continue로 다음 파일로 넘어갑니다.
                continue;
            }

            // 목적지 디렉토리가 없다면 생성
            if !dest_dir.exists() {
                fs::create_dir_all(&dest_dir)?;
            }

            // 파일을 지정된 경로로 이동
            let dest_file = dest_dir.join(entry.file_name());
            fs::rename(&path, &dest_file)?;
            println!("Moved {:?} to {:?}", file_name, dest_dir);
        }
    }

    Ok(())
}
