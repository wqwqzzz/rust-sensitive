use rust_sensitive::model::DfaSensitiveWordMap;
// data.txt 内容为 “敏感 测试 as 832”
#[test]
fn test_filter_case1() {
    let mmap = DfaSensitiveWordMap::init_dfa_dic_from_file("./data.txt");
    let result = mmap.filter("啊实打实的我敏感打算测试as订单832");
    assert_eq!(result, "啊实打实的我打算订单")
}
#[test]
fn test_filter_case2() {
    let mmap = DfaSensitiveWordMap::init_dfa_dic_from_file("./data.txt");
    let result = mmap.filter("832832832");
    assert_eq!(result, "")
}

#[test]
fn test_replace_case1() {
    let mmap = DfaSensitiveWordMap::init_dfa_dic_from_file("./data.txt");
    let result = mmap.replace("啊实打实的我敏感打算测试as订单832", "*");
    assert_eq!(result, "啊实打实的我**打算****订单***".to_string());
}
#[test]
fn test_replace_case2() {
    let mmap = DfaSensitiveWordMap::init_dfa_dic_from_file("./data.txt");
    let result = mmap.replace("啊实打实的我敏感打算测试as订单832", "?");
    assert_eq!(result, "啊实打实的我??打算????订单???".to_string());
}
#[test]
fn test_is_sensitive_case1() {
    let mmap = DfaSensitiveWordMap::init_dfa_dic_from_file("./data.txt");
    let result = mmap.is_sensitive("啊实打实的我敏感打算测试as订单832");
    assert_eq!(result, true);
}
#[test]
fn test_is_sensitive_case2() {
    let mmap = DfaSensitiveWordMap::init_dfa_dic_from_file("./data.txt");
    let result = mmap.is_sensitive("你是谁");
    assert_eq!(result, false);
}
#[test]
fn test_find_one_case1() {
    let mmap = DfaSensitiveWordMap::init_dfa_dic_from_file("./data.txt");
    let result = mmap.find_one("啊实打实的我敏感打算测试as订单832");
    assert_eq!(result, "敏感")
}
#[test]
fn test_find_one_case2() {
    let mmap = DfaSensitiveWordMap::init_dfa_dic_from_file("./data.txt");
    let result = mmap.find_one("啊实打实as的我敏感打算测试as订单832");
    assert_eq!(result, "as")
}
#[test]
fn test_find_all_case1() {
    let mmap = DfaSensitiveWordMap::init_dfa_dic_from_file("./data.txt");
    let result = mmap.find_all("啊实打实的我敏感打算测试as订单832");
    assert_eq!(result, vec!["敏感", "测试", "as", "832"]);
}
#[test]
fn test_find_all_case2() {
    let mmap = DfaSensitiveWordMap::init_dfa_dic_from_file("./data.txt");
    let result = mmap.find_all("啊实打实的我敏感打算测试as订单832");
    assert_ne!(result, vec!["测试", "as", "832"]);
}
