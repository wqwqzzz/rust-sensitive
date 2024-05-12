use std::collections::HashMap;
use std::sync::OnceLock;
use aho_corasick::AhoCorasick;
use std::io::Read;

pub struct DfaSensitiveWordMap {
    pub is_end: bool,                                 // 是否是最后一个字符
    pub data: String,                                 // 数据
    pub children: HashMap<char, DfaSensitiveWordMap>, // 子节点
}

pub type AcSensitiveWordMap =AhoCorasick;

static DFASENSITIVEMAP: OnceLock<DfaSensitiveWordMap> = OnceLock::new();
static ACSENSITIVEMAP : OnceLock<AcSensitiveWordMap> = OnceLock::new();

fn read_file(file_path: &str) -> String{
    let mut content = String::new();
    match std::fs::File::open(file_path){
        Ok(mut file) => {
            file.read_to_string(&mut content).unwrap();
        }
        Err(e) => {
            println!("{}", e);
        }
    }
    let split_content: Vec<&str> = content
    .split(|c| c == '\n' || c == ' ' || c == ',')
    .filter(|s| !s.is_empty())
    .collect();
    return split_content.join(" ");

}

pub trait MyTrait{
    fn init_ac_dic_from_file(file_path: &str) -> &'static AcSensitiveWordMap;
    fn get_ac_dic() -> &'static AcSensitiveWordMap;
    fn filter(&self, text: &str) -> String;
    fn replace(&self, text: &str,repl:&str) -> String;
    fn is_sensitive(&self, text: &str) -> bool;
    fn find_one(&self, text: &str) -> String;
    fn find_all(&self, text: &str) -> Vec<String>;
}
impl MyTrait for AcSensitiveWordMap{
    fn init_ac_dic_from_file(file_path: &str) -> &'static AcSensitiveWordMap{
        ACSENSITIVEMAP.get_or_init(||{
            let content = read_file(file_path);
            let pattern = content.split(" ").collect::<Vec<&str>>();
            let ac = AhoCorasick::builder().build(pattern).unwrap();
            return ac;
        })
    }
    fn get_ac_dic() -> &'static AcSensitiveWordMap{
        ACSENSITIVEMAP.get_or_init(||{
            AcSensitiveWordMap::builder().build(&[""]).unwrap()
        })
    }
    fn filter(&self, text: &str) -> String {
        let mut text_copy = text.to_string();
        let mut matches = vec![];
        for mat in self.find_iter(text) {
            matches.push((mat.pattern(), mat.start(), mat.end()));
        }
        matches.reverse();
        for (_, start, end) in matches {
            let chars_count = text_copy[start..end].chars().count();
            let replacement: String = "".repeat(chars_count);
            text_copy.replace_range(start..end, &replacement);
        }
        return text_copy;
    }
    fn replace(&self, text: &str,repl:&str) -> String {
        let mut text_copy = text.to_string();
        let mut matches = vec![];
        for mat in self.find_iter(text) {
            matches.push((mat.pattern(), mat.start(), mat.end()));
        }
        matches.reverse();
        
        for (_, start, end) in matches {
            let chars_count=text_copy[start..end].chars().count();
            let replacement: String = repl.to_string().repeat(chars_count);
            text_copy.replace_range(start..end, &replacement);
        }
        
        return text_copy;
    }
    fn is_sensitive(&self, text: &str) -> bool {
        if self.find_iter(text).count() > 0{
            true
        }else{
            false
        }
    }
    fn find_one(&self, text: &str) -> String {
        for mat in self.find_iter(text) {
            return text[mat.start()..mat.end()].to_string()
        }
        "".to_string()
    }
    fn find_all(&self, text: &str) -> Vec<String> {
        let mut sensitive_word_list = Vec::new();       
        for mat in self.find_iter(text){
            let word = &text[mat.start()..mat.end()];
            sensitive_word_list.push(format!("{}", word));
        }
        return sensitive_word_list;
    }
}



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
