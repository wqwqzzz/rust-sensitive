# 🚫 rust-sensitive

[![build](https://img.shields.io/badge/build-1.00-brightgreen)](https://github.com/wqwqzzz/rust-sensitive)

[English](README.md) | [中文](README_ZH.md)

> 敏感词过滤, 支持多种数据源加载, 多种过滤算法, 多种操作功能

## 🌟 Feature

- 支持多种操作功能
    - `filter()` 返回过滤后的文本
    - `replace()` 返回替换了敏感词后的文本
    - `is_sensitive()` 返回文本是否含有敏感词
    - `find_one()` 返回匹配到的第一个敏感词
    - `find_all()` 返回匹配到的所有敏感词
- 支持多种数据源加载
    - ✅ 支持内存存储
    - 🔲 支持mysql存储
    - 🔲 支持mongo存储
- 支持多种过滤算法
    - ✅ **DFA** 使用 `HashMap` 数据结构匹配敏感词
    - ✅ **AC 自动机**

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
在项目中运行以下 Cargo 命令 directory:
```
    cargo add rust_sensitive
```
或者在 Cargo.toml 中添加以下一行：
```
    [dependencies]
    rust_sensitive = "1.1.0"
```



## 📌 TODO

- [ ] add mongo data source support
- [ ] add  bloom algorithm
- [ ] add  tests example