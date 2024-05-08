# Rust cli:base64编码和解码

## 运行方式

### 执行base encode

```bash
cargo run -- base64 encode
win10下用回车，ctrl+z结束输入。

```

```bash
cargo run -- base64 encode --format urlsafe

```

```bash
cargo run -- base64 encode --input .\Cargo.toml 
cargo run -- base64 encode --input .\Cargo.toml >b64.txt

```
### 执行base decode

```bash
cargo run -- base64 decode
cargo run -- base64 decode --format urlsafe
cargo run -- base64 decode --input b64.txt
```
### 执行单元测试
```bash
 cargo nextest run  -- test_process_decode
```