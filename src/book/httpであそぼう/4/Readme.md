# HTTP であそぼう part4

早くサーバーにしてよ って思ったので。

# 本日の一言

(公開時に埋める)

# TCP を取り扱おう

rust の標準クレートの一つ、net モジュールは優秀だと思います。  
おかげで簡単に通信ができるのです。

```rust, ignore
# use anyhow::{Result, anyhow};
# use std::{
#     collections::HashMap,
#     io::{Read, Write},
#     net::TcpListener,
# };
#
#
fn main() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:80")?;
}
```

TcpListener インスタンスにある accept メソッドで、受信したパケットを受け取ることができます。

## ブロッキング

その前に、ブロッキングについて知っておく必要があります。  
関数は、極力成功するように作られています。 **処理を成功させるためなら無限に時間をとるものもあります。**  
例えば、時間のかかる補助記憶装置からの読み込み時にブロッキングされます。
通信であれば、**次のパケットを受け取るまでブロッキング**など。

## バッファ

バッファも知っておく必要があります。  
OS は一度、全ての通信をバッファに蓄えます。 そこから、アプリケーションごとに分配を行っていくのです。  
[linux だと sysctl で調整できます。](https://access.redhat.com/ja/solutions/504383)

なお、OS のバッファからデータを拾ってくるのは**アプリケーション側**です。  
アプリケーションがモタモタしてたら、OS のバッファを使い尽くしてしまいます。  
受信したパケットは素早く処理できるようにするのが理想です。

> [!NOTE]  
> Accept して read しないで待機しつつ、大量に通信送ってみたら、（つまり OS のバッファを埋めるように待っていたら）  
> いろんな通信ができなくなった経験がある。経験則。

# 受信してみる

実は 3 章とかで少し触れてます。 今回は実際の TcpListener を使うだけです。

```rust, ignore
# use anyhow::{Result, anyhow};
# use std::{
#     collections::HashMap,
#     io::{Read, Write},
#     net::TcpListener,
# };
#
# pub mod http_util;
#
fn main() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:4000")?;
    {
        let (mut stream, _) = listener.accept()?;
        let mut buf = [0u8; 512];

        stream.read(&mut buf)?;

        let buf = String::from_utf8_lossy(&buf);
        println!("{}", buf);
    }
    Ok(())
}
```

バッファを作り、そこに移動してもらうようにします。 mut 付き参照渡しをします。  
戻り値は実際に読み込めたサイズとなります。 なお最大値(バッファを 512 で作ってるなら 512)で返ってきた場合、**まだデータが残ってる恐れがあることを検討しましょう。**

とりまこの形になるとこまで。

## HTTP にパースしてみる

HTTP パーサーでパースしてみます。

```rust, ignore
# use anyhow::{Result, anyhow};
# use std::{
#     collections::HashMap,
#     io::{Read, Write},
#     net::TcpListener,
# };
#
# pub mod http_util;
#
fn main() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:4000")?;
    let (mut stream, _) = listener.accept()?;
    let mut buf = [0u8; 512];

    stream.read(&mut buf)?;

    let buf = String::from_utf8_lossy(&buf);

    let req = http_util::HttpRequest::from_str(&buf).ok_or(anyhow!("parse error"))?;
    println!("{} {}", req.method, req.path);
    Ok(())
}
```

エラー処理は anyhow に任せてます。~~めんどいし。~~  
HttpRequest の各要素は **public**です。そのまま取り出すことができます。

> [!NOTE]  
> rust は基本不変ですので、この設計でいいと思いますがいかがでしょうか。  
> 使用者が HttpRequest を mut で作って変更してたら、そういう事情があると考えていいと私は思います。

> [!TIP]  
> `curl http://localhost:4000`の実行結果
>
> (サーバー側)
>
> ```
> GET /
> ```
>
> (クライアント側)
>
> ```
> curl: (52) Empty reply from server
> ```

## 返信をする

もらったリクエストにはレスポンスで返すようにします。

```rust, ignore
# use anyhow::{Result, anyhow};
# use std::{
#     collections::HashMap,
#     io::{Read, Write},
#     net::TcpListener,
# };
#
# pub mod http_util;
#
fn main() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:4000")?;
    // (中略)
#        let (mut stream, _) = listener.accept()?;
#        let mut buf = [0u8; 512];
#
#        stream.read(&mut buf)?;
#
#        let buf = String::from_utf8_lossy(&buf);
#
#        let req = http_util::HttpRequest::from_str(&buf).ok_or(anyhow!("parse error"))?;
#        println!("{} {}", req.method, req.path);

    let mut res_header = HashMap::new();
    res_header.insert("Content-Type", "text/html; charset=utf-8");
    let res = http_util::HttpResponse::new(
        req.version,
        (200, "Ok"),
        res_header,
        String::from("<h1>hello world</h1>"),
    );
    stream.write_all(res.to_string().as_bytes())?;

    stream.flush()?;
    Ok(())
}
```

...safari だとページがロードされない。実装が適当だから。
