use rust_sensitive::model::{AcSensitiveWordMap, MyTrait};
// data.txt 内容为 “敏感 测试 as 832”
#[test]
fn test_filter_case1() {
    let sensitive_word_map = AcSensitiveWordMap::init_ac_dic_from_file("./data.txt");
    let text = "你真敏感";
    let filtered_text = sensitive_word_map.filter(text);
    assert_eq!(filtered_text, "你真");
}

#[test]
fn test_filter_case2() {
    let sensitive_word_map = AcSensitiveWordMap::init_ac_dic_from_file("./data.txt");
    let text = "你真敏感测试asss832";
    let filtered_text = sensitive_word_map.filter(text);
    assert_eq!(filtered_text, "你真ss");
}

#[test]
fn test_replace_case1() {
    let sensitive_word_map = AcSensitiveWordMap::init_ac_dic_from_file("./data.txt");
    let text = "你真敏感测试asss832";
    let replaced_text = sensitive_word_map.replace(text, "*");
    assert_eq!(replaced_text, "你真******ss***");
}
#[test]
fn test_replace_case2() {
    let sensitive_word_map = AcSensitiveWordMap::init_ac_dic_from_file("./data.txt");
    let text = "敏感测试832as";
    let replaced_text = sensitive_word_map.replace(text, "!");
    assert_eq!(replaced_text, "!!!!!!!!!");
}

#[test]
fn test_is_sensitive_case1() {
    let sensitive_word_map = AcSensitiveWordMap::init_ac_dic_from_file("./data.txt");
    let text = "你真敏感测试asss832";
    let is_sensitive = sensitive_word_map.is_sensitive(text);
    assert_eq!(is_sensitive, true);
}

#[test]
fn test_is_sensitive_case2() {
    let sensitive_word_map = AcSensitiveWordMap::init_ac_dic_from_file("./data.txt");
    let text = "这是个测案例试";
    let is_sensitive = sensitive_word_map.is_sensitive(text);
    assert_eq!(is_sensitive, false);
}

#[test]
fn test_find_one_case1() {
    let sensitive_word_map = AcSensitiveWordMap::init_ac_dic_from_file("./data.txt");
    let text = "你真敏感测试asss832";
    let result = sensitive_word_map.find_one(text);
    assert_eq!(result, "敏感".to_string());
}

#[test]
fn test_find_one_case2() {
    let sensitive_word_map = AcSensitiveWordMap::init_ac_dic_from_file("./data.txt");
    let text = "这是个测案例试";
    let result = sensitive_word_map.find_one(text);

    assert_eq!(result, "".to_string());
}

#[test]
fn test_find_all_case1() {
    let sensitive_word_map = AcSensitiveWordMap::init_ac_dic_from_file("./data.txt");
    let text = "你真敏感测试asss832";
    let result = sensitive_word_map.find_all(text);

    assert_eq!(
        result,
        vec![
            "敏感".to_string(),
            "测试".to_string(),
            "as".to_string(),
            "832".to_string()
        ]
    );
}

#[test]
fn test_find_all_case2() {
    let sensitive_word_map = AcSensitiveWordMap::init_ac_dic_from_file("./data.txt");
    let text = "这是个测案例试";
    let result = sensitive_word_map.find_all(text);
    assert_eq!(result, Vec::<String>::new());
}

#[test]
fn test_is_sensitive_case3() {
    let sensitive_word_map = AcSensitiveWordMap::init_ac_dic_from_file("./data.txt");
    let m = AcSensitiveWordMap::get_ac_dic();
    let text = "你真敏感测试asss832";
    let is_sensitive = m.is_sensitive(text);

    assert_eq!(is_sensitive, true);
}
