# Rust cli:generate simple password

## 运行方式

### 执行命令

```bash
cargo run -- genpass -l 16 --no-uppercase
生成密码:
    -?h8=#tdx*-2;^3(
```

```bash

cargo run -- genpass -l 16
生成密码:
    #WVmBC5M/Z$2vP7y
```

```bash
cargo run -- genpass -l 16 --no-uppercase --no-number
生成密码:
    wj/ddj{_ndkjt)?k
```