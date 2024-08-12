一、功能

- 使用 rust 语言调用 pcre2 (源码地址：https://github.com/PhilipHazel/pcre2)            使用 FFI 绑定 C 接口
- 从 **目标文本** 中筛选出符合 **筛选规则** 的 **结果字符串**，
- 将 **结果字符串** 发送给一个 bash 脚本，
- 在这个 bash 脚本中接收并打印接收到的结果，
- 传输过程中使用 UDP 协议。

二、目标文本

"a;jhgoqoghqoj0329 u0tyu10hg0h9Y0Y9827342482y(Y0y(G)_)lajf;lqjfgqhgpqjopjqa=)*(^!@#$%^&*())9999999"

三、筛选规则

1. **结果字符串** 自身不包含数字和任何类型的空白字符（如空格、回车等等），其长度为 3 至 11 个字符
2. **结果字符串** 左侧相邻的字符串是4个数字
3. **结果字符串** 右侧相邻的字符串不为空
4. 正则匹配的次数越少越好，尽可能只使用一个正则表达式

注：以上 4 条规则须同时满足。

注意事项:

```shell
MacOS 的 arm64 芯片

先安装

$ arch -arm64 brew install pcre2

再检查 

$ file /opt/homebrew/opt/pcre2/lib/libpcre2-8.dylib

/opt/homebrew/opt/pcre2/lib/libpcre2-8.dylib: Mach-O 64-bit dynamically linked shared library arm64

最后在 .cargo/config.toml 文件中添加

[target.aarch64-apple-darwin]
linker = "clang"
rustflags = ["-L", "/opt/homebrew/opt/pcre2/lib"]

```

```shell
$ cargo version
cargo 1.80.0 (376290515 2024-07-16)
$ rustc --version
rustc 1.80.0 (051478957 2024-07-21)
```

* 编译

```shell
$ cargo build --release
```

* 运行单元测试

```shell
$ cargo test
```

* 运行

```shell
在一个终端窗口运行
$ ./receiver.sh

在另一个终端窗口运行
% ./target/release/reg_ex_udp_transmitter
```
