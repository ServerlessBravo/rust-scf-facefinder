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

### *测试*

- 修改测试 html 文件中的远程API的地址信息

  index.html#L87

    ```js

    # 替换地址为 APIGW 触发器地址

    const url = "https://service-xxxx.gz.apigw.tencentcs.com/release/rust-facedector-demo";

    img.onload = function () {
      console.log('img=', img);
      canvas.width = img.width;
      canvas.height = img.height;
      canvas.style.width = img.width + 'px';
      canvas.style.height = img.height + 'px';
      ctx.drawImage(img, 0, 0);
    };
    img.src = "faces.png"

    ```
- 打开本地的 test 目录 `index.html` 文件
- 选择 png 文件，目录 `test/pngs` 有部分样本文件
- 点击 按钮进行人脸监测

### *其他*

- 为了方便测试，添加触发器的时候不开启鉴权
- 添加 APIGW 触发器的时候，需要选择集成响应模式，SCF 返回 APIGW 指定的内容格式
- 编译应用的时候，需要基于 Linux x86_64 架构进行编译，推荐采用 Centos 系统
- 需要在 APIGW 控制台开启允许 `CORS` 跨域请求


### *引用*

- 参考链接：[在腾讯云上部署一个Rust人脸检测云函数](https://zhuanlan.zhihu.com/p/476715251)
