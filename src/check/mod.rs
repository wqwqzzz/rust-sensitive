use crate::model::DfaSensitiveWordMap;
use std::collections::HashMap;
use utf8_slice;
impl DfaSensitiveWordMap{
    // 检查敏感词
    // 返回过滤后的文本
    pub fn filter(&self, text: &str,repl: char) -> String{
        let mut text_copy = text.to_string();
        let mut i = 0;
        while i < text.len() {
            let length= check_sensitive_words(text,i,&self);

            if length > 0 {
                //存在敏感词,修改可变变量text_copy的内容
                let start = text.char_indices().nth(i).unwrap().0;
                let end = if i + length<= text.len(){
                    text.char_indices()
                        .nth(i+ length)
                        .map(|(idx, _)| idx)
                        .unwrap_or(text.len())
                }else{
                    text.len()
                };
                let replacement = repl.to_string().repeat(end-start);
                text_copy.replace_range(start..end, &replacement);
                i += length-1;
            }
            i +=1
        }
        
        text_copy
        
    }

}

// 检查敏感词
fn check_sensitive_words(
    target_text: &str,
    begin_index: usize,
    sensitive_word_map: &DfaSensitiveWordMap,
) -> usize{
    let mut match_flag = 0 ; // 敏感词长度
    let mut flag = false ; // 标识位，匹配结果
    let mut now_map = sensitive_word_map;

    for i in begin_index..target_text.len() {
        let word = match target_text.chars().nth(i) {
            Some(ch) => ch,
            None => break,
        };
        now_map = match now_map.children.get(&word) {
            Some(map) => {
                match_flag +=1;
                
                if map.is_end {
                    flag = true;
                    if flag{
                        break;
                    }
                }
                map
            }
            None => {
                break;
            }
        };  
    }
    match_flag 
}
