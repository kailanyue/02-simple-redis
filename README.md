## 一个简单的 redis server 实现

### 01.resp 协议
参考文档: [resp 协议](https://redis.io/docs/latest/develop/reference/protocol-spec/)

encode / decode

添加 cargo 依赖
enum_dispatch：可以用于轻松地重构动态分派的 trait 访问，从而将其性能提高多达 10 倍
bytes：提供了一组宏和数据结构，用于处理字节数据，具有零拷贝网络编程、避免借用检查器问题和高性能等优势
thiserror：提供了一个方便的 derive 宏，用于标准库的 std::error::Error 特征，简化了自定义错误类型的实现，使你的代码更加简洁和表达力强。
```sh
cargo add enum_dispatch
cargo add bytes
cargo add thiserror
```

需要解析的命令
```sh
- simple string: "+OK\r\n"
- error: "-Error message\r\n"
- bulk error: "!<length>\r\n<error>\r\n"
- integer: ":[<+|->]<value>\r\n"
- bulk string: "$<length>\r\n<data>\r\n"
- null bulk string: "$-1\r\n"
- array: "*<number-of-elements>\r\n<element-1>...<element-n>"
    - "*2\r\n$3\r\nget\r\n$5\r\nhello\r\n"
- null array: "*-1\r\n"
- null: "_\r\n"
- boolean: "#<t|f>\r\n"
- double: ",[<+|->]<integral>[.<fractional>][<E|e>[sign]<exponent>]\r\n"
- map: "%<number-of-entries>\r\n<key-1><value-1>...<key-n><value-n>"
- set: "~<number-of-elements>\r\n<element-1>...<element-n>"
```

创建 encode trait 和 decode trait