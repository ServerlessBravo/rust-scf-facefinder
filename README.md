# Rust and WebAssembly Demo for SCF


![](https://user-images.githubusercontent.com/251222/174422986-2cb455ad-1d69-4c87-b8c9-8b48a9bcc7b5.png)


### Rust Demo


```bash
## 静态链接编译
RUSTFLAGS='-C target-feature=+crt-static' cargo build --target=x86_64-unknown-linux-gnu --release
## 复制编译好的文件
cp target/x86_64-unknown-linux-gnu/release/scf-server pkg/bootstrap
cd pkg
## 给文件添加执行权限
chmod 755 ./bootstrap
## 打包zip
zip pkg.zip bootstrap
```

通过 `Custom Runtime` 执行 [bootstrap](https://github.com/ServerlessBravo/rust-scf-facefinder/blob/main/rust-demo/src/main.rs)


### Wasm Demo



```bash

curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
wasm-pack build --target nodejs --release
zip -r pkg.zip index.js pkg  -x "*.git*"

```

通过 `Nodejs Runtime` 执行：

```js
const wasm = require("./pkg/rust_scf_facefinder");

exports.main_handler = async (event, context, callback) => {
  var body = event['body']
  var finder_result = wasm.process_event(body);

  return {
    isBase64Encoded: false,
    statusCode: 200,
    headers: { 'Content-Type': 'application/json' },
    body: finder_result,
  }
}

```

### *引用*

- 参考链接：[在腾讯云上部署一个Rust人脸检测云函数](https://zhuanlan.zhihu.com/p/476715251)
