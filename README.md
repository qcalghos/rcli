# Rust cli:base64编码和解码

## 运行方式

### 进行签名

```bash
 cargo run -- text sign  -k .\fixtures\blake3.txt

```
### 验证签名
```bash
cargo run -- text verify -k .\fixtures\blake3.txt --sig QRHorjpemcqvocTNKUahsD-EMdDZmFIa__9GV4z6atI

```

```bash
cargo run -- text generate-key

```
### 生成key
```bash
cargo run -- text generate -o ./fixtures/
cargo run -- text generate -o ./fixtures/ --format ed25519
```
