# 🚫 rust-sensitive

[![build](https://img.shields.io/badge/build-1.00-brightgreen)](https://github.com/wqwqzzz/rust-sensitive)

[English](README.md) | [中文](README_ZH.md)

> Sensitive word filtering, support multiple data source loading, multiple filtering algorithms, multiple operation functions

## 🌟 Feature

- Supports a wide range of operating functions
    - `filter()` returns the filtered text.
    - `replace()` Returns the text after replacing sensitive words.
    - `is_sensitive()` Returns whether the text contains sensitive words.
    - `find_one()` Returns the first sensitive word matched.
    - `find_all()` returns all the sensitive words matched.
- Support multiple data sources loading
    - ✅ Support memory storage
    - 🔲 Support mysql storage
    - 🔲 Support mongo storage
- Support multiple filtering algorithms
    - ✅ **DFA** Use `HashMap` to match sensitive words.
    - ✅ **AC automated machine**


## ⚙ Usage
### first use
```rust
    use rust_sensitive::model::DfaSensitiveWordMap;

    fn test(){
        // Initialize the sensitive_map
        let map =DfaSensitiveWordMap::init_dfa_dic_from_file("./data.txt");
        /* because use once_cell,you can use 
         DfaSensitiveWordMap::get_dfa_dic() 
         to get the sensitive_map 
        */

        /*
        if use ac algorithm,you can use
        use rust_sensitive::model::AcSensitiveWordMap;

        let map =AcSensitiveWordMap::init_ac_dic_from_file("./data.txt"); 
        and because use once_call,you can also use
        AcSensitiveWordMap::get_ac_dic()
        to get the sensitive_map
        */
    }
    fn test2(){
        let map = DfaSensitiveWordMap::get_dfa_dic();
        // let map = AcSensitiveWordMap::get_ac_dic();
    }
```
### filter()
```rust
    fn filter(&self, text_copy: &str) -> String
    // "hello" is sensitive word
    fn test(){
        let map = DfaSensitiveWordMap::get_dfa_dic();
        let result = map.filter("hello,world!");
        println!("{}",result); //",world"

    }
```
### replace()
```rust
    replace(&self, text_copy: &str, repl: &str) -> String
    // "hello" is sensitive word
    fn test(){
        let map = DfaSensitiveWordMap::get_dfa_dic();
        let result = map.replace("hello,world!","*");
        // repl is like "!"、"*".a single word
        println!("{}",result); //"*****,world"

    }
```
### is_sensitive()
```rust
    fn is_sensitive(&self, text: &str) -> bool
    // "hello" is sensitive word
    fn test(){
        let map = DfaSensitiveWordMap::get_dfa_dic();
        let result = map.is_sensitive("hello,world!");
        println!("{}",result); //"true"

    }
```

### find_one()
```rust
    fn find_one(&self, text: &str) -> String
    // "hello" is sensitive word
    fn test(){
        let map = DfaSensitiveWordMap::get_dfa_dic();
        let result = map.find_one("hello,world!");
        println!("{}",result); //"hello"

    }
```

### find_all()
```rust
    find_all(&self, text: &str) -> Vec<String>
    // "hello" "world" are sensitive words
    fn test(){
        let map = DfaSensitiveWordMap::get_dfa_dic();
        let result = map.find_all("hello,world!");
        println!("{}",result); //"hello","world"

    }
```

## ✔ Get

Run the following Cargo command in your project directory:
```
    cargo add rust_sensitive
```
Or add the following line to your Cargo.toml:
```
    [dependencies]
    rust_sensitive = "1.1.0"
```


## 

## 📌 TODO

- [ ] add mongo data source support
- [ ] add  bloom algorithm