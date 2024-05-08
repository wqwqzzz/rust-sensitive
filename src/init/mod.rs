use std::char;

pub mod init_dic;

impl init_dic::DfaSensitiveMap {
    // Replace() 返回替换了敏感词后的文本
    pub fn Replace(&self, text: &str,repl:char) -> String {
        self.trie.common_prefix_search().collect()
    }
    // IsSensitive() 返回文本是否含有敏感词
    // FindOne() 返回匹配到的第一个敏感词
    // FindAll() 返回匹配到的所有敏感词
    // FindAllCount() 返回匹配到的所有敏感词及出现次数
}