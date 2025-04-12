# Sayaka

Sayaka 是一个轻量级的 Rust 调试工具箱，提供简洁实用的宏来辅助你进行开发调试。

## 特性

- 彩色格式化的调试输出
- 可配置的调试信息显示
- 简单易用的宏，用于打印变量、函数调用信息

## 安装

在你的 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
sayaka = "0.2.0"
```
或者在命令行中使用cargo命令

```
cargo add sayaka
```

## 使用

### 基本用法

```rust
use sayaka::{debugln, debug_var, debug_fn};

fn main() {
    let name = "world";
    
    // 打印简单的调试信息
    debugln!("Hello, {}!", name);
    
    // 打印变量的值和类型
    debug_var!(name);
    
    // 跟踪函数调用及其参数
    my_function(42);
}

fn my_function(value: i32) {
    debug_fn!(value);
    // 函数主体...
}
```

### 环境变量配置

Sayaka 可通过两个环境变量进行配置：

- `SAYAKA_DEBUG`：设为非零值以启用调试输出
- `SAYAKA_NO_COLOR`：设为非零值以禁用彩色输出

示例：

```sh
# 启用调试输出
SAYAKA_DEBUG=1 cargo run

# 启用调试但禁用彩色输出
SAYAKA_DEBUG=1 SAYAKA_NO_COLOR=1 cargo run
```

## API 参考

### 宏

- `debugln!($fmt, ...)`：打印调试信息，格式类似于 `println!`
- `debug_var!($var1, $var2, ...)`：详细打印一个或多个变量的值
- `debug_fn!($arg1, $arg2, ...)`：打印当前函数名称及其参数

## 许可证

MIT