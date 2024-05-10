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
    ✅ 支持内存存储
    🔲 支持mysql存储
    🔲 支持mongo存储
- 支持多种过滤算法
    ✅ **DFA** 使用 `HashMap` 数据结构匹配敏感词
    🔲 **AC 自动机**

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
- [ ] add  tests example