use crate::model::DfaSensitiveWordMap;
use std::fs::File;
use std::io::Read;


// 初始化map，阅读文件？根据参数不同选择不同的阅读方式？
pub fn read_file(file_path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}

impl DfaSensitiveWordMap {
    pub fn t_init_dfa_dic_from_file(
        &mut self,
        file_path: &str,
    ) -> Result<(), std::io::Error> {
        if !self.children.is_empty(){
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Already been initialized",
            ));
        }
        // 在这里操作map
        let sensitive_content = read_file(file_path)?;
        let split_content: Vec<&str> = sensitive_content
            .split(|c| c == '\n' || c == ' ' || c == ',')
            .filter(|s| !s.is_empty())
            .collect(); // 空格、换行、逗号为分隔符都可以
    
        let mut sensitive_map = DfaSensitiveWordMap::new();
        // 初始化字符串表，然后把 sensitive的子树赋予map
        for word in split_content {
            let key = word.to_string();
    
            let mut now_point = &mut sensitive_map;
            for (i, key_char) in key.chars().enumerate() {
                let word_map = now_point.children.get(&key_char);
                match word_map {
                    Some(_child_map) => {
                        now_point = now_point.children.get_mut(&key_char).unwrap();
                    }
                    None => {
                        let  new_map = DfaSensitiveWordMap::new();
                        now_point.children.insert(key_char, new_map);
                        now_point = now_point.children.get_mut(&key_char).unwrap();
                    }
                }
    
                if i == key.chars().count() - 1 {
                    now_point.is_end = true;
                }
            }
        }
        self.children = sensitive_map.children;
    
        Ok(())
    }
}

