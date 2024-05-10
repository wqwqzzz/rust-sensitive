use std::collections::HashMap;
use std::sync::OnceLock;
pub struct DfaSensitiveWordMap {
    pub is_end: bool,                                 // 是否是最后一个字符
    pub data: String,                                 // 数据
    pub children: HashMap<char, DfaSensitiveWordMap>, // 子节点
}

static DFASENSITIVEMAP: OnceLock<DfaSensitiveWordMap> = OnceLock::new();

impl DfaSensitiveWordMap {
    pub fn new() -> Self {
        DfaSensitiveWordMap {
            is_end: false,
            data: "".to_string(),
            children: HashMap::new(),
        }
    }
    pub fn init_dfa_dic_from_file(file_path: &str) -> &'static DfaSensitiveWordMap {   
        DFASENSITIVEMAP.get_or_init(||{ 
            let mut tmp = DfaSensitiveWordMap::new();
            match tmp.t_init_dfa_dic_from_file(file_path){
                Ok(_) => {
                    return tmp;
                }
                Err(e) => {
                    eprintln!("Error initializing DfaSensitiveWordMap: {}", e);
                    return DfaSensitiveWordMap::new()  
                }
            }
        })
    }
    pub fn get_dfa_dic() -> &'static DfaSensitiveWordMap{
        DFASENSITIVEMAP.get_or_init(||{
            DfaSensitiveWordMap::new()
        })
    }
    
}
