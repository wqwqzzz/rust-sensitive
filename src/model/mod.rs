use std::collections::HashMap;

pub struct DfaSensitiveWordMap {
    pub is_end: bool, // 是否是最后一个字符
    pub data: String, // 数据
    pub children: HashMap<char, DfaSensitiveWordMap>,  // 子节点
}

impl DfaSensitiveWordMap {
    pub fn new() -> Self {
        DfaSensitiveWordMap {
            is_end: false,
            data: "".to_string(),
            children: HashMap::new(),
        }
    }
}