# rust-scf-facefinder
腾讯云函数SCF Nodejs + Wasm 部署Rust人脸检测

![](https://user-images.githubusercontent.com/251222/174422986-2cb455ad-1d69-4c87-b8c9-8b48a9bcc7b5.png)


### 打包编译



```bash
## install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

## build as wasm
./build.sh

```

构建产物为 `pkg.zip`

### 部署

#### 函数配置

```bash
函数类型	Event函数
运行环境	Nodejs 16.13
内存	512MB
初始化超时时间	65秒
执行超时时间	10秒

```

#### 触发器配置

```bash
鉴权方式	免鉴权
启用集成响应	已启用
启用Base64编码	未启用
支持CORS	是
后端超时	15s
公网访问路径  https://service-xxx.gz.apigw.tencentcs.com/release/facefinder_wasm_demo
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
- 参考链接：[在函数计算FunctionCompute中使用WebAssembly](https://developer.aliyun.com/article/713613)

