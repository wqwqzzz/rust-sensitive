use std::str;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use trie_rs::{Trie, TrieBuilder};

pub struct DfaSensitiveMap{
    pub trie : Trie<u8>
}
pub fn init_dictionary(dir_path : &str) -> Result<DfaSensitiveMap, Box<dyn std::error::Error>>{
    let path = Path::new(dir_path);
    // 读文件
    let mut file = File::open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // 初始化tried树
    let mut builder = TrieBuilder::new();

    for word in contents.split_whitespace() {
        builder.push(word);
    }

    let trie = builder.build();
    Ok(DfaSensitiveMap{trie})
}
