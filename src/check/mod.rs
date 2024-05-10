use crate::model::DfaSensitiveWordMap;
impl DfaSensitiveWordMap {
    // 检查敏感词
    // 返回替换后的文本
    pub fn replace(&self, text_copy: &str, repl: &str) -> String {
        let mut text = text_copy.to_string();
        let mut i = 0;
        while i < text.len() {
            let length = check_sensitive_words(text_copy, i, &self);

            if length > 0 {
                //存在敏感词,修改可变变量text_copy的内容
                let start = text.char_indices().nth(i).unwrap().0;
                let end = if i + length <= text.len() {
                    text.char_indices()
                        .nth(i + length)
                        .map(|(idx, _)| idx)
                        .unwrap_or(text.len())
                } else {
                    text.len()
                };
                let replacement = repl.repeat(length);
                text.replace_range(start..end, &replacement);
                i += length - 1;
            }
            i += 1
        }

        text
    }
    // 返回过滤后的文本
    pub fn filter(&self, text_copy: &str) -> String {
        let mut text = text_copy.to_string();
        let mut i = 0;
        while i < text.len() {
            let length = check_sensitive_words(text_copy, i, &self);

            if length > 0 {
                //存在敏感词,修改可变变量text_copy的内容
                let start = text.char_indices().nth(i).unwrap().0;
                let end = if i + length <= text.len() {
                    text.char_indices()
                        .nth(i + length)
                        .map(|(idx, _)| idx)
                        .unwrap_or(text.len())
                } else {
                    text.len()
                };
                let replacement = " ".repeat(length);
                text.replace_range(start..end, &replacement);
                i += length - 1;
            }
            i += 1
        }

        text.chars().filter(|&c| !c.is_whitespace()).collect()
    }

    // 检查文本是否包含敏感词
    pub fn is_sensitive(&self, text: &str) -> bool {
        let mut i = 0;
        while i < text.len() {
            let length = check_sensitive_words(text, i, &self);

            if length > 0 {
                //存在敏感词,修改可变变量text_copy的内容
                return true;
            }
            i += 1
        }

        false
    }

    // find_one() 返回匹配到的第一个敏感词
    pub fn find_one(&self, text: &str) -> String {
        let mut i = 0;
        while i < text.len() {
            let length = check_sensitive_words(text, i, &self);

            if length > 0 {
                // 存在敏感词，
                let start = text.char_indices().nth(i).unwrap().0;
                let end = if i + length <= text.len() {
                    text.char_indices()
                        .nth(i + length)
                        .map(|(idx, _)| idx)
                        .unwrap_or(text.len())
                } else {
                    text.len()
                };

                return  text[start..end].to_string()
            }
            i += 1
        }

        "".to_string()
    }

    // find_all() 返回匹配到的所有敏感词
    pub fn find_all(&self, text: &str) -> Vec<String> {
        let mut sensitive_word_list = Vec::new();
        let mut i = 0;
        while i < text.len() {
            let length = check_sensitive_words(text, i, &self);

            if length > 0 {
                let start = text.char_indices().nth(i).unwrap().0;
                let end = if i + length <= text.len() {
                    text.char_indices()
                        .nth(i + length)
                        .map(|(idx, _)| idx)
                        .unwrap_or(text.len())
                } else {
                    text.len()
                };

                let word = &text[start..end];
                sensitive_word_list.push(format!("{}", word));
                i += length - 1;
            }
            i += 1
        }

        sensitive_word_list
    }
}

// 检查敏感词
fn check_sensitive_words(
    target_text: &str,
    begin_index: usize,
    sensitive_word_map: &DfaSensitiveWordMap,
) -> usize {
    let mut match_flag = 0; // 敏感词长度
    let mut now_map = sensitive_word_map;
    let mut flag = false; 
    for i in begin_index..target_text.len() {
        let word = match target_text.chars().nth(i) {
            Some(ch) => ch,
            None => break,
        };
        now_map = match now_map.children.get(&word) {
            Some(map) => {
                match_flag += 1;

                if map.is_end {
                    flag = true;
                    //break;
                }
                map
            }
            None => {
                break;
            }
        };
    }
    if flag {
        return match_flag;
    }
    0
}
