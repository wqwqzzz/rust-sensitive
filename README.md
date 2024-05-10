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
    ✅ Support memory storage
    🔲 Support mysql storage
    🔲 Support mongo storage
- Support multiple filtering algorithms
    ✅ **DFA** Use `HashMap` to match sensitive words.
    🔲 **AC automated machine**


## ⚙ Usage

```rust

```

## ✔ Get

```
```

## 📂 Import

```go
```

## 

## 📌 TODO

- [ ] add mongo data source support
- [ ] add  bloom algorithm