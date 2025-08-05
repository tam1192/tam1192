# HTTP であそぼう part3.5

いつまで経っても詳しいと言える自信はないです。どんな分野でも。

# 🎵 本日の一曲

<iframe width="312" height="176" src="https://ext.nicovideo.jp/thumb/sm42537184" scrolling="no" style="border:solid 1px #ccc;" frameborder="0"><a href="https://www.nicovideo.jp/watch/sm42537184">星灯の在処／紙崎ねい feat.SynthV Saki</a></iframe>

かっこいい

# 存在する http ライブラリを実際に試してみる

今回は本題から外れ、実際に存在する rust 製 http ライブラリを試そうと思います。

私が発見したのでは

- [httparse](https://docs.rs/httparse/latest/httparse/)
- [reqwest](https://docs.rs/reqwest/latest/reqwest/)
  です。 今回は二つ試そうかと

## ところで[cookbook](https://rust-lang-nursery.github.io/rust-cookbook/intro.html)はご存知で？

[rust-cookbook](https://rust-lang-nursery.github.io/rust-cookbook/intro.html)は、料理レシピ本のように色々なプログラミング例がのってるサイトです。  
[rust-example](https://doc.rust-jp.rs/rust-by-example-ja/)との違いは、example は基本的な構文の理解のためにあるのに対し、  
cookbook は よく使うクレート（ライブラリ）の例を載せてあります。

# httparse

http/1.x のリクエスト、レスポンスを解析するためのパーサーです。 SIMD と呼ばれる**一回の計算で複数の式を実行できる**的なやつに対応してる模様。  
パース後は構造体で管理されるそうです。

## 一例

request のみ検証しました。

```rust, ignore
let buf = b"POST / HTTP/1.1\r\nHost: localhost\r\nSec-Fetch-Dest: document\r\nUser-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.5 Safari/605.1.15\r\nUpgrade-Insecure-Requests: 1\r\nAccept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8\r\nSec-Fetch-Site: none\r\nSec-Fetch-Mode: navigate\r\nAccept-Language: ja\r\nPriority: u=0, i\r\nAccept-Encoding: gzip, deflate\r\nConnection: keep-alive\r\nContent-Type: text/plain\r\n\r\nhello world";

let mut headers = [EMPTY_HEADER; 64];
let mut request = Request::new(&mut headers);

let head_size = request.parse(buf).unwrap().unwrap();

assert_eq!(request.method.unwrap(), "POST");
assert_eq!(request.path.unwrap(), "/");

// 1.0だと0, 1.1だと1
// 1.0, 1.1以外はnoneになる?
assert_eq!(request.version.unwrap(), 1);

assert_eq!(String::from_utf8_lossy(&buf[head_size..]), "hello world");
```

シンプルです。 headers の中身を debug でのぞいてみると

```
[Header { name: "Host", value: "localhost" }, Header { name: "Sec-Fetch-Dest", value: "document" }, Header { name: "User-Agent", value: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.5 Safari/605.1.15" }, Header { name: "Upgrade-Insecure-Requests", value: "1" }, Header { name: "Accept", value: "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8" }, Header { name: "Sec-Fetch-Site", value: "none" }, Header { name: "Sec-Fetch-Mode", value: "navigate" }, Header { name: "Accept-Language", value: "ja" }, Header { name: "Priority", value: "u=0, i" }, Header { name: "Accept-Encoding", value: "gzip, deflate" }, Header { name: "Connection", value: "keep-alive" }, Header { name: "Content-Type", value: "text/plain" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }, Header { name: "", value: "" }]
```

となってます。
なお、私が探した限り、検索などの機能は備わってません。

改行コードが`\r\n`でないとパースされませんでしたが、おそらくプロトコルの仕様がそうなってるんだと思います、と思って調べてたら面白い記事を発見。
[CRLF インジェクションとは？ -CyberMatrix](https://www.cybermatrix.co/post/crlf-injection)

おもろいというか、巧妙な手口だなと思いました。  
この例で Cookie を扱ってますが、その辺は今後検証したいと思います。

(この記事書いてる目的は、http を理解するとともに cookie を理解したいから)

# reqwest

reqwest は js でいう fetch のように、http クライアントとして通信するのに特化したクレートです。

```rust
use anyhow::Result;
use std::io::Read;

fn main() -> Result<()> {
    let mut res = reqwest::blocking::get("http://httpbin.org/get")?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);

    Ok(())
}
```

そのまま cookbook から持ってきました。

- [anyhow](https://docs.rs/anyhow/latest/anyhow/)  
  エラー処理を簡単にするクレート
- [httpbin.org](http://httpbin.org)
  http 関係のテストができるサイト  
  [http://httpbin.org/get](http://httpbin.org/get)では、送った get リクエストのヘッダー情報や、url パラメータがそのまま返却される。

reqwest の feature で blocking が必要でした。

# まとめ

外部のライブラリに触れると、思わぬ発見があってたのしい。
