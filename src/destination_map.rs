use std::collections::HashMap;

// 공통 경로를 상수로 선언
const BASE_PATH: &str = "./Downloads/_DONE/_Magazines/";

pub fn get_destination_map() -> HashMap<String, String> {
    let mut destination_map: HashMap<String, String> = HashMap::new();

    // format!을 사용하여 각 경로를 문자열로 생성
    destination_map.insert(
        "3D World ".to_string(), 
        format!("{}__Future Plc/3D World", BASE_PATH),
    );
    destination_map.insert(
        "25 Beautiful Homes ".to_string(), 
        format!("{}__Future Plc/25 Beautiful Homes", BASE_PATH),
    );
    destination_map.insert(
        "TypeC_".to_string(), 
        "pdfs/sorted/type_c".to_string(),
    );

    // 더 많은 경로 추가 가능
    destination_map
}
