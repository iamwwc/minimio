实现一个 mini 的 MIO
乃至实现 Tokio
1. 先实现单线程 Selector socket 模型
2. 实现多线程的 file read 支持


MIO 的核心是 Selector，select 创建的socket设置为noblock，然后继续添加到 select 去轮询

单线程的 mio 没有异步的概念，只有衍生出多线程模型后（tokio）就需要 Future wake 来唤醒就绪的任务

### MIO 项目分析

```
src/
├── event
├── macros
├── net // 通用 net实现，封装了 sys 下平台相关的实现
│   ├── tcp
│   └── uds
└── sys
|    ├── shell
|    ├── unix // unix 下 net selector 实现
|    │   ├── selector
|    │   └── uds
|    └── windows //windows 下 net selector 实现
├── token.rs
└── waker.rs
└── poll.rs // poll实现，封装 sys 下 select 的实现
```

tokio 项目代码复用了 mio，不一样的是 tokio 将 mio 的同步代码封装成 async

比如 tokio 的 TcpStream 实现

```rs
async fn connect_addr(addr: SocketAddr) -> io::Result<TcpStream> {
    // 使用 mio
    let sys = mio::net::TcpStream::connect(addr)?;
    // 将 mio 的 Socket 转换为 Future
    TcpStream::connect_mio(sys).await
}

pub(crate) async fn connect_mio(sys: mio::net::TcpStream) -> io::Result<TcpStream> {
    let stream = TcpStream::new(sys)?;

    // Once we've connected, wait for the stream to be writable as
    // that's when the actual connection has been initiated. Once we're
    // writable we check for `take_socket_error` to see if the connect
    // actually hit an error or not.
    //
    // If all that succeeded then we ship everything on up.
    poll_fn(|cx| stream.io.registration().poll_write_ready(cx)).await?;

    if let Some(e) = stream.io.take_error()? {
        return Err(e);
    }

    Ok(stream)
}
```

类似的还有 tokio driver io

### 感悟

总体而言，mio只是 epoll 相关概念的封装，只需要掌握 epoll 的使用就可以自己封装，

