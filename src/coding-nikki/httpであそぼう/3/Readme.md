# HTTP であそぼう part3

お世話になってます。 日記は書いてないけど日記と申します。
毎日投稿すれば実質日記ですよね？ (極力)毎日投稿するために内容は薄めです。

## 本日の 1 曲 🎶

恒例のやつです。

<iframe width="312" height="176" src="https://ext.nicovideo.jp/thumb/sm44052214" scrolling="no" style="border:solid 1px #ccc;" frameborder="0"><a href="https://www.nicovideo.jp/watch/sm44052214">ラ・ラ・ラ・ジャーニー！／紡乃世詞音's</a></iframe>

紡乃世さんはやっぱ明るい方が好き。 ヤンデレ動画も好きやけども...

## ★ について (★★★)

★ はこの記事の中でとりあえず読んで欲しいところに 3、自分用のメモレベルで 1 をつけてます。  
なので、★1 の内容が理解できなくても問題ないです! 書いてるやつが悪い。  
一方 ★3 は頑張って書きましたので読んで欲しいな〜

# 本題　(★★☆)

前回作った擬似 TCP 関数により、こういうサイトや、PlayGround、test コードで手にとる様な感覚で通信を取り扱える様になりました。
今回は、1,2 でやったことを一旦混ぜてみて、どういったことができるのかを再確認します。

## ファイルの整理 (★☆☆)

```bash
>tree src
src
├── http_util
│   ├── method.rs
│   ├── mod.rs
│   ├── path.rs
│   ├── request.rs
│   ├── utils.rs
│   └── version.rs
├── lib.rs
└── vnet -> ../../2/code/src/vnet
```

~~response は少し待って欲しい。~~一応やりました。  
とりあえず 40 分くらいかけてファイルを整理してみました。 40 分か。  
というのも、テストも大量に増やしてみたからです。ドキュメント書こうとすると 2 時間ぐらい取られそうですね。

自分か感じる以上にものを書くのは時間がかかるということです。まぁ手書きよりかはましですが...

## テストについて (★★★)

主に mac 版 safari と curl コマンドのリクエストを書きました。

> [!NOTE]
> curl のリクエスト文
>
> ```
> GET / HTTP/1.1
> Host: localhost
> User-Agent: curl/8.7.1
> Accept: */*
> ```

> [!NOTE]
> safari のリクエスト文
>
> ```
> GET / HTTP/1.1
> Host: localhost
> Sec-Fetch-Dest: document
> User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.5 Safari/605.1.15
> Upgrade-Insecure-Requests: 1
> Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8
> Sec-Fetch-Site: none
> Sec-Fetch-Mode: navigate
> Accept-Language: ja
> Priority: u=0, i
> Accept-Encoding: gzip, deflate
> Connection: keep-alive
> ```

safari のリクエスト分は、ヘッダーが長いですね。

使ってるのは M4 mac なのですが、Intel Mac となっているのが興味深いですね。 KHTML は KDE プロジェクトのやつですね。  
また、firefox 開発元の Mozilla/5.0 というの興味深いです。 User-Agent は何でアクセスしているかではなく、**なんのフレームワークに対応しているか書かれていると感じますね。**

`Accept-Language: ja` によって表示言語が送信されます。 しかし、実際にはそれだけでなく、IP アドレスや、位置情報など、さまざまな手段で言語を確定させます。

`Accept-Encoding: gzip, deflate` はファイル圧縮技術ですね。 送受信するファイルのサイズを小さくできるので、通信費圧縮に貢献しそうです。

> [!TIP]
> まだ HTTP の頃、ziproxy と呼ばれる proxy ソフトが存在した。 これは通信費圧縮のために、中間サーバーでパケットを圧縮するものです。

`Connection: keep-alive` は**HTTP**の keepAlive 設定です。 **TCP の KeepALive ではありません**。 TCP の KeepALive ではないというのは**重要な問題を含んでいます。**

**KeepALive を実装しないといけない**

...そのうちやりますね。

## 問題が生じた (★☆☆)

ファイルを分けるとこのページで実行するファイルを作るのが大変!!!
ってことです。

実行時は[rust の playground](https://play.rust-lang.org/)を使うのですが、まぁ大変になります。
これを回避する方法は見つけられませんでした。 諦めて、全結合したコードを使います。 通信費かかると思うけど許して...  
[Minify Rust Code – Rust Minifier](https://unminifyall.com/minify-rust/) でコード圧縮するから!!

## 通信を再現してみる(★★☆)

本題です。 実はこれを書くのに 2 日ぐらいかけました。  
とりあえず、せっかくファイルを整理しましたが、無理やり抽出してここで実行できるようにしてみましたので試してみてね。

```rust
# mod http_util { mod path { use std::fmt::{self, Debug}; pub struct HttpPath<'a>(&'a str); impl<'a> Debug for HttpPath<'a> { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "HttpPath({})", self.0) } } impl<'a> PartialEq for HttpPath<'a> { fn eq(&self, other: &Self) -> bool { self.0 == other.0 } } impl<'a> Clone for HttpPath<'a> { fn clone(&self) -> Self { HttpPath(self.0) } } impl<'a> HttpPath<'a> { pub fn from_str(s: &'a str) -> Option<Self> { let mut c = s.chars(); if c.next() != Some('/') { return None; } if c.find(|c| { !(c.is_ascii_alphanumeric() || *c == '/' || *c == '-' || *c == '_' || *c == '.' || *c == '=' || *c == '?' || *c == '&' || *c == '%' || *c == '#') }) == None { Some(HttpPath(s)) } else { None } } } impl<'a> From<&'a str> for HttpPath<'a> { fn from(s: &'a str) -> Self { HttpPath::from_str(s).unwrap_or(HttpPath("/")) } } impl<'a> fmt::Display for HttpPath<'a> { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.0) } } } mod method { use std::fmt; pub enum HttpMethod { Get, Post, Put, Delete, } impl fmt::Debug for HttpMethod { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "HttpMethod({})", self) } } impl PartialEq for HttpMethod { fn eq(&self, other: &Self) -> bool { core::mem::discriminant(self) == core::mem::discriminant(other) } } impl Clone for HttpMethod { fn clone(&self) -> Self { match self { Self::Get => Self::Get, Self::Post => Self::Post, Self::Put => Self::Put, Self::Delete => Self::Delete, } } } impl HttpMethod { pub fn from_str(s: &str) -> Option<Self> { match s.to_lowercase().as_str() { "get" => Some(HttpMethod::Get), "post" => Some(HttpMethod::Post), "put" => Some(HttpMethod::Put), "delete" => Some(HttpMethod::Delete), _ => None, } } } impl From<&str> for HttpMethod { fn from(s: &str) -> Self { HttpMethod::from_str(s).unwrap_or(HttpMethod::Get) } } impl fmt::Display for HttpMethod { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!( f, "{}", match self { Self::Get => "GET", Self::Post => "POST", Self::Put => "PUT", Self::Delete => "DELETE", } ) } } } pub mod version { use std::fmt; pub enum HttpVersion { Http10, Http11, Http20, Http30, } impl fmt::Debug for HttpVersion { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "HttpVersion({})", self) } } impl Clone for HttpVersion { fn clone(&self) -> Self { match self { Self::Http10 => Self::Http10, Self::Http11 => Self::Http11, Self::Http20 => Self::Http20, Self::Http30 => Self::Http30, } } } impl PartialEq for HttpVersion { fn eq(&self, other: &Self) -> bool { core::mem::discriminant(self) == core::mem::discriminant(other) } } impl HttpVersion { pub fn from_str(s: &str) -> Option<Self> { match s.to_lowercase().as_str() { "http/1.0" => Some(HttpVersion::Http10), "http/1.1" => Some(HttpVersion::Http11), "http/2.0" => Some(HttpVersion::Http20), "http/3.0" => Some(HttpVersion::Http30), _ => None, } } } impl From<&str> for HttpVersion { fn from(s: &str) -> Self { HttpVersion::from_str(s).unwrap_or(HttpVersion::Http10) } } impl fmt::Display for HttpVersion { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!( f, "{}", match self { Self::Http10 => "HTTP/1.0", Self::Http11 => "HTTP/1.1", Self::Http20 => "HTTP/2.0", Self::Http30 => "HTTP/3.0", } ) } } } pub mod request { use super::{utils::*, *}; use std::{collections::HashMap, fmt}; pub struct HttpRequest<'a> { pub m: HttpMethod, pub p: HttpPath<'a>, pub v: HttpVersion, pub h: HashMap<&'a str, &'a str>, pub b: String, } impl PartialEq for HttpRequest<'_> { fn eq(&self, other: &Self) -> bool { self.m == other.m && self.p == other.p && self.v == other.v && self.h == other.h && self.b == other.b } } impl Clone for HttpRequest<'_> { fn clone(&self) -> Self { Self { m: self.m.clone(), p: self.p.clone(), v: self.v.clone(), h: self.h.clone(), b: self.b.clone(), } } } impl fmt::Debug for HttpRequest<'_> { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.debug_struct("HttpRequest") .field("method", &self.m) .field("path", &self.p) .field("version", &self.v) .field("header", &self.h) .field("body", &self.b) .finish() } } impl<'a> HttpRequest<'a> { pub fn from_str(s: &'a str) -> Option<Self> { let mut ls = s.lines(); let mut ps = { let l = ls.next().unwrap_or(""); l.split_whitespace() }; let m = HttpMethod::from_str(ps.next().unwrap_or(""))?; let p = HttpPath::from_str(ps.next().unwrap_or(""))?; let v = HttpVersion::from_str(ps.next().unwrap_or(""))?; if ps.next().is_some() { return None; } let mut h: HashMap<&str, &str> = HashMap::new(); loop { let ls = ls.next().unwrap_or(""); match line_parse_http_header(ls) { Some((k, v)) => { _ = h.insert(k, v); } None => break, } } let b = ls.collect::<String>(); Some(Self { m, p, v, h, b }) } } impl<'a> fmt::Display for HttpRequest<'a> { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{} {} {}\r\n", self.m, self.p, self.v)?; for (k, v) in &self.h { write!(f, "{}: {}\r\n", k, v)?; } write!(f, "\r\n{}", self.b) } } } pub mod response { use super::{utils::*, *}; use std::{collections::HashMap, fmt};  pub struct HttpResponse<'a> { pub v: HttpVersion, pub s: (u16, &'a str), pub h: HashMap<&'a str, &'a str>, pub b: String, } impl PartialEq for HttpResponse<'_> { fn eq(&self, other: &Self) -> bool { self.v == other.v && self.s == other.s && self.h == other.h && self.b == other.b } } impl Clone for HttpResponse<'_> { fn clone(&self) -> Self { Self { v: self.v.clone(), s: self.s.clone(), h: self.h.clone(), b: self.b.clone(), } } } impl fmt::Debug for HttpResponse<'_> { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.debug_struct("HttpResponse") .field("version", &self.v) .field("status", &self.s) .field("header", &self.h) .field("body", &self.b) .finish() } } impl<'a> HttpResponse<'a> { pub fn from_str(s: &'a str) -> Option<Self> { let mut ls = s.lines(); let mut ps = { let l = ls.next().unwrap_or(""); l.split_whitespace() }; let v = HttpVersion::from_str(ps.next().unwrap_or(""))?; let sc = ps.next().and_then(|s| s.parse::<u16>().ok())?; let sm = ps.next().unwrap_or(""); if ps.next().is_some() { return None; } let mut h: HashMap<&str, &str> = HashMap::new(); loop { let l = ls.next().unwrap_or(""); match line_parse_http_header(l) { Some((k, v)) => { _ = h.insert(k, v); } None => break, } } let body = ls.collect::<String>(); Some(Self { v, s: (sc, sm), h, b: body, }) } } impl<'a> fmt::Display for HttpResponse<'a> { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{} {} {}\r\n", self.v, self.s.0, self.s.1)?; for (k, v) in &self.h { write!(f, "{}: {}\r\n", k, v)?; } write!(f, "\r\n{}", self.b) } } } pub mod utils { pub fn line_parse_http_header(s: &str) -> Option<(&str, &str)> { let i = s.find(':')?; Some((&s[0..i], &s[i + 1..].trim())) } } pub use self::{ method::HttpMethod, path::HttpPath, request::HttpRequest, response::HttpResponse, version::HttpVersion, };}mod vnet { use std::{ collections::VecDeque, io::{Read, Result, Write}, net::{SocketAddr, ToSocketAddrs}, thread, }; pub struct TcpListener { a: SocketAddr, r: VecDeque<&'static [u8]>, } impl PartialEq for TcpListener { fn eq(&self, other: &Self) -> bool { self.a == other.a && self.r == other.r } } impl Clone for TcpListener { fn clone(&self) -> Self { Self { a: self.a.clone(), r: self.r.clone(), } } } impl std::fmt::Debug for TcpListener { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.debug_struct("TcpListener") .field("addr", &self.a) .field("requests", &self.r) .finish() } } impl TcpListener { pub fn bind<A>(addr: A) -> Result<TcpListener> where A: ToSocketAddrs, { let a = addr.to_socket_addrs()?.next().ok_or(std::io::Error::new( std::io::ErrorKind::InvalidInput, "No address found", ))?; Ok(TcpListener { a, r: VecDeque::new(), }) } pub fn local_addr(&self) -> Result<SocketAddr> { Ok(self.a) } pub fn add_request(&mut self, request: &'static [u8]) { self.r.push_back(request); } pub fn accept(&mut self) -> Result<(TcpStream, SocketAddr)> { loop { if let Some(rd) = self.r.pop_front() { let s = TcpStream { rd, wd: Vec::new(), fd: false, }; return Ok((s, self.a)); } thread::sleep(std::time::Duration::from_millis(100)); } } } pub struct TcpStream<'a> { rd: &'a [u8], wd: Vec<u8>, fd: bool, } impl PartialEq for TcpStream<'_> { fn eq(&self, other: &Self) -> bool { self.rd == other.rd && self.wd == other.wd && self.fd == other.fd } } impl Clone for TcpStream<'_> { fn clone(&self) -> Self { Self { rd: self.rd, wd: self.wd.clone(), fd: self.fd.clone(), } } } impl std::fmt::Debug for TcpStream<'_> { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.debug_struct("TcpStream") .field("read_data", &self.rd) .field("write_data", &self.wd) .field("is_flushed", &self.fd) .finish() } } impl TcpStream<'_> { pub fn new() -> TcpStream<'static> { TcpStream { rd: &[], wd: Vec::new(), fd: false, } } pub fn get_write_data(&self) -> Option<&[u8]> { if self.fd { Some(&self.wd) } else { None } } } impl Read for TcpStream<'_> { fn read(&mut self, buf: &mut [u8]) -> Result<usize> { let b = self.rd; let l = b.len().min(buf.len()); buf[..l].copy_from_slice(&b[..l]); Ok(l) } } impl Write for TcpStream<'_> { fn write(&mut self, b: &[u8]) -> Result<usize> { if self.fd { return Err(std::io::Error::new( std::io::ErrorKind::WriteZero, "Stream is flushed, cannot write", )); } self.wd.extend_from_slice(b); Ok(b.len()) } fn flush(&mut self) -> Result<()> { println!("Flushing data: {:?}", self.wd); self.fd = true; Ok(()) } }}
#
# use std::io::{Read, Write};
#
# fn main() {
// setup
let mut listener = vnet::TcpListener::bind("127.0.0.1:8080").unwrap();
listener.add_request("GET / HTTP/1.1\r\nHost: localhost\r\n\r\n".as_bytes());
let (mut stream, _) = listener.accept().unwrap();

// read
let mut read_buf = [0u8; 512];
_ = stream.read(&mut read_buf).unwrap();
let binding = String::from_utf8_lossy(&read_buf);
let request = http_util::HttpRequest::from_str(&binding).unwrap();
println!("{}", request);

// write
let response = http_util::HttpResponse::from_str(
    "HTTP/1.1 200 Ok\r\nContent-Type: text/plain\r\n\r\nhelloworld!\r\n",
)
.unwrap();
_ = stream.write(response.to_string().as_bytes());
_ = stream.flush();
println!(
    "{}",
    String::from_utf8_lossy(stream.get_write_data().unwrap())
)
# }
```

## 簡易的な web サーバーを立ててみる(★★★)

やっと本題だよ。  
HTTP パケットを見ればわかるように、サーバーにはたくさんの情報が送られます。  
サーバーは、送られた情報を元に**ユーザーが求めているのは何か？**というのを考え、答えなければなりません。

> [!NOTE]
> 求めているものは**ページはもちろん、フォーマット(html なのか、json なのか)は？、言語は？、ブラウザの環境は？ などなど**

今回 3 つのページをやりとりします。

### `GET / HTTP/1.1`の場合

このファイルを返します

<div id="livecodes-tze0iktyfgi"></div>
<script src="https://cdn.jsdelivr.net/npm/livecodes@0.11.1/livecodes.umd.js"></script>
<script>
  const options = {
    "appUrl": "https://v46.livecodes.io/",
    "config": {
      "mode": "simple"
    },
    "import": "id/7wwgbzyygcj"
  };
  livecodes.createPlayground("#livecodes-tze0iktyfgi", options);
</script>

### `GET /profile HTTP/1.1`の場合

このファイルを返します

<div id="livecodes-aczpxwusqi"></div>
<script>
  const options2 = {
    "appUrl": "https://v46.livecodes.io/",
    "config": {
      "mode": "simple"
    },
    "import": "id/39kpdcwdqdn"
  };
  livecodes.createPlayground("#livecodes-aczpxwusqi", options2);
</script>

### それ以外の場合

みんな大好きあれを返します

<div id="livecodes-myws2y78d9"></div>
<script>
  const options3 = {
    "appUrl": "https://v46.livecodes.io/",
    "config": {
      "mode": "simple"
    },
    "import": "id/edy9rmsmvac"
  };
  livecodes.createPlayground("#livecodes-myws2y78d9", options3);
</script>

### 実際にコードにしてみる

```rust
# mod http_util { mod path { use std::fmt::{self, Debug}; pub struct HttpPath<'a>(&'a str); impl<'a> Debug for HttpPath<'a> { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "HttpPath({})", self.0) } } impl<'a> PartialEq for HttpPath<'a> { fn eq(&self, other: &Self) -> bool { self.0 == other.0 } } impl<'a> Clone for HttpPath<'a> { fn clone(&self) -> Self { HttpPath(self.0) } } impl<'a> HttpPath<'a> { pub fn from_str(s: &'a str) -> Option<Self> { let mut c = s.chars(); if c.next() != Some('/') { return None; } if c.find(|c| { !(c.is_ascii_alphanumeric() || *c == '/' || *c == '-' || *c == '_' || *c == '.' || *c == '=' || *c == '?' || *c == '&' || *c == '%' || *c == '#') }) == None { Some(HttpPath(s)) } else { None } } } impl<'a> From<&'a str> for HttpPath<'a> { fn from(s: &'a str) -> Self { HttpPath::from_str(s).unwrap_or(HttpPath("/")) } } impl<'a> fmt::Display for HttpPath<'a> { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.0) } } } mod method { use std::fmt; pub enum HttpMethod { Get, Post, Put, Delete, } impl fmt::Debug for HttpMethod { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "HttpMethod({})", self) } } impl PartialEq for HttpMethod { fn eq(&self, other: &Self) -> bool { core::mem::discriminant(self) == core::mem::discriminant(other) } } impl Clone for HttpMethod { fn clone(&self) -> Self { match self { Self::Get => Self::Get, Self::Post => Self::Post, Self::Put => Self::Put, Self::Delete => Self::Delete, } } } impl HttpMethod { pub fn from_str(s: &str) -> Option<Self> { match s.to_lowercase().as_str() { "get" => Some(HttpMethod::Get), "post" => Some(HttpMethod::Post), "put" => Some(HttpMethod::Put), "delete" => Some(HttpMethod::Delete), _ => None, } } } impl From<&str> for HttpMethod { fn from(s: &str) -> Self { HttpMethod::from_str(s).unwrap_or(HttpMethod::Get) } } impl fmt::Display for HttpMethod { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!( f, "{}", match self { Self::Get => "GET", Self::Post => "POST", Self::Put => "PUT", Self::Delete => "DELETE", } ) } } } pub mod version { use std::fmt; pub enum HttpVersion { Http10, Http11, Http20, Http30, } impl fmt::Debug for HttpVersion { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "HttpVersion({})", self) } } impl Clone for HttpVersion { fn clone(&self) -> Self { match self { Self::Http10 => Self::Http10, Self::Http11 => Self::Http11, Self::Http20 => Self::Http20, Self::Http30 => Self::Http30, } } } impl PartialEq for HttpVersion { fn eq(&self, other: &Self) -> bool { core::mem::discriminant(self) == core::mem::discriminant(other) } } impl HttpVersion { pub fn from_str(s: &str) -> Option<Self> { match s.to_lowercase().as_str() { "http/1.0" => Some(HttpVersion::Http10), "http/1.1" => Some(HttpVersion::Http11), "http/2.0" => Some(HttpVersion::Http20), "http/3.0" => Some(HttpVersion::Http30), _ => None, } } } impl From<&str> for HttpVersion { fn from(s: &str) -> Self { HttpVersion::from_str(s).unwrap_or(HttpVersion::Http10) } } impl fmt::Display for HttpVersion { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!( f, "{}", match self { Self::Http10 => "HTTP/1.0", Self::Http11 => "HTTP/1.1", Self::Http20 => "HTTP/2.0", Self::Http30 => "HTTP/3.0", } ) } } } pub mod request { use super::{utils::*, *}; use std::{collections::HashMap, fmt}; pub struct HttpRequest<'a> { pub m: HttpMethod, pub p: HttpPath<'a>, pub v: HttpVersion, pub h: HashMap<&'a str, &'a str>, pub b: String, } impl PartialEq for HttpRequest<'_> { fn eq(&self, other: &Self) -> bool { self.m == other.m && self.p == other.p && self.v == other.v && self.h == other.h && self.b == other.b } } impl Clone for HttpRequest<'_> { fn clone(&self) -> Self { Self { m: self.m.clone(), p: self.p.clone(), v: self.v.clone(), h: self.h.clone(), b: self.b.clone(), } } } impl fmt::Debug for HttpRequest<'_> { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.debug_struct("HttpRequest") .field("method", &self.m) .field("path", &self.p) .field("version", &self.v) .field("header", &self.h) .field("body", &self.b) .finish() } } impl<'a> HttpRequest<'a> { pub fn from_str(s: &'a str) -> Option<Self> { let mut ls = s.lines(); let mut ps = { let l = ls.next().unwrap_or(""); l.split_whitespace() }; let m = HttpMethod::from_str(ps.next().unwrap_or(""))?; let p = HttpPath::from_str(ps.next().unwrap_or(""))?; let v = HttpVersion::from_str(ps.next().unwrap_or(""))?; if ps.next().is_some() { return None; } let mut h: HashMap<&str, &str> = HashMap::new(); loop { let ls = ls.next().unwrap_or(""); match line_parse_http_header(ls) { Some((k, v)) => { _ = h.insert(k, v); } None => break, } } let b = ls.collect::<String>(); Some(Self { m, p, v, h, b }) } } impl<'a> fmt::Display for HttpRequest<'a> { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{} {} {}\r\n", self.m, self.p, self.v)?; for (k, v) in &self.h { write!(f, "{}: {}\r\n", k, v)?; } write!(f, "\r\n{}", self.b) } } } pub mod response { use super::{utils::*, *}; use std::{collections::HashMap, fmt};  pub struct HttpResponse<'a> { pub v: HttpVersion, pub s: (u16, &'a str), pub h: HashMap<&'a str, &'a str>, pub b: String, } impl PartialEq for HttpResponse<'_> { fn eq(&self, other: &Self) -> bool { self.v == other.v && self.s == other.s && self.h == other.h && self.b == other.b } } impl Clone for HttpResponse<'_> { fn clone(&self) -> Self { Self { v: self.v.clone(), s: self.s.clone(), h: self.h.clone(), b: self.b.clone(), } } } impl fmt::Debug for HttpResponse<'_> { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.debug_struct("HttpResponse") .field("version", &self.v) .field("status", &self.s) .field("header", &self.h) .field("body", &self.b) .finish() } } impl<'a> HttpResponse<'a> { pub fn from_str(s: &'a str) -> Option<Self> { let mut ls = s.lines(); let mut ps = { let l = ls.next().unwrap_or(""); l.split_whitespace() }; let v = HttpVersion::from_str(ps.next().unwrap_or(""))?; let sc = ps.next().and_then(|s| s.parse::<u16>().ok())?; let sm = ps.next().unwrap_or(""); if ps.next().is_some() { return None; } let mut h: HashMap<&str, &str> = HashMap::new(); loop { let l = ls.next().unwrap_or(""); match line_parse_http_header(l) { Some((k, v)) => { _ = h.insert(k, v); } None => break, } } let body = ls.collect::<String>(); Some(Self { v, s: (sc, sm), h, b: body, }) } } impl<'a> fmt::Display for HttpResponse<'a> { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{} {} {}\r\n", self.v, self.s.0, self.s.1)?; for (k, v) in &self.h { write!(f, "{}: {}\r\n", k, v)?; } write!(f, "\r\n{}", self.b) } } } pub mod utils { pub fn line_parse_http_header(s: &str) -> Option<(&str, &str)> { let i = s.find(':')?; Some((&s[0..i], &s[i + 1..].trim())) } } pub use self::{ method::HttpMethod, path::HttpPath, request::HttpRequest, response::HttpResponse, version::HttpVersion, };}mod vnet { use std::{ collections::VecDeque, io::{Read, Result, Write}, net::{SocketAddr, ToSocketAddrs}, thread, }; pub struct TcpListener { a: SocketAddr, r: VecDeque<&'static [u8]>, } impl PartialEq for TcpListener { fn eq(&self, other: &Self) -> bool { self.a == other.a && self.r == other.r } } impl Clone for TcpListener { fn clone(&self) -> Self { Self { a: self.a.clone(), r: self.r.clone(), } } } impl std::fmt::Debug for TcpListener { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.debug_struct("TcpListener") .field("addr", &self.a) .field("requests", &self.r) .finish() } } impl TcpListener { pub fn bind<A>(addr: A) -> Result<TcpListener> where A: ToSocketAddrs, { let a = addr.to_socket_addrs()?.next().ok_or(std::io::Error::new( std::io::ErrorKind::InvalidInput, "No address found", ))?; Ok(TcpListener { a, r: VecDeque::new(), }) } pub fn local_addr(&self) -> Result<SocketAddr> { Ok(self.a) } pub fn add_request(&mut self, request: &'static [u8]) { self.r.push_back(request); } pub fn accept(&mut self) -> Result<(TcpStream, SocketAddr)> { loop { if let Some(rd) = self.r.pop_front() { let s = TcpStream { rd, wd: Vec::new(), fd: false, }; return Ok((s, self.a)); } thread::sleep(std::time::Duration::from_millis(100)); } } } pub struct TcpStream<'a> { rd: &'a [u8], wd: Vec<u8>, fd: bool, } impl PartialEq for TcpStream<'_> { fn eq(&self, other: &Self) -> bool { self.rd == other.rd && self.wd == other.wd && self.fd == other.fd } } impl Clone for TcpStream<'_> { fn clone(&self) -> Self { Self { rd: self.rd, wd: self.wd.clone(), fd: self.fd.clone(), } } } impl std::fmt::Debug for TcpStream<'_> { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.debug_struct("TcpStream") .field("read_data", &self.rd) .field("write_data", &self.wd) .field("is_flushed", &self.fd) .finish() } } impl TcpStream<'_> { pub fn new() -> TcpStream<'static> { TcpStream { rd: &[], wd: Vec::new(), fd: false, } } pub fn get_write_data(&self) -> Option<&[u8]> { if self.fd { Some(&self.wd) } else { None } } } impl Read for TcpStream<'_> { fn read(&mut self, buf: &mut [u8]) -> Result<usize> { let b = self.rd; let l = b.len().min(buf.len()); buf[..l].copy_from_slice(&b[..l]); Ok(l) } } impl Write for TcpStream<'_> { fn write(&mut self, b: &[u8]) -> Result<usize> { if self.fd { return Err(std::io::Error::new( std::io::ErrorKind::WriteZero, "Stream is flushed, cannot write", )); } self.wd.extend_from_slice(b); Ok(b.len()) } fn flush(&mut self) -> Result<()> { println!("Flushing data: {:?}", self.wd); self.fd = true; Ok(()) } }}
#
# use std::{io::{Read, Write}, collections::HashMap};
#
// 今回はページをあらかじめハードコードしておく
const MAIN_PAGE: &str = "<h1>Hello world!</h1>";
const PROFILE_PAGE: &str = "<h1>profile</h1>";
const ERROR_PAGE: &str = "<h1>404 Not Found</h1>";

fn main() {
    // setup
    let mut listener = vnet::TcpListener::bind("127.0.0.1:8080").unwrap();
    // この順番でリクエストを4回送る
    listener.add_request("GET / HTTP/1.1\r\n\r\n".as_bytes()); // ホームページ
    listener.add_request("GET /profile HTTP/1.1\r\n\r\n".as_bytes()); // profileページ
    listener.add_request("GET /error HTTP/1.1\r\n\r\n".as_bytes()); // エラーページ
    listener.add_request("GET /e233 HTTP/1.1\r\n\r\n".as_bytes()); // エラーページ

    // 4回分のパケットをリクエストを受け取る
    for _ in 0..4 {
        let (mut stream, _) = listener.accept().unwrap();

        // リクエストを受け取る
        let mut read_buf = [0u8; 512];
        _ = stream.read(&mut read_buf).unwrap();
        let request = String::from_utf8_lossy(&read_buf);
        let request = http_util::HttpRequest::from_str(&request).unwrap();

        // パスごとにページを切り替える
        let (page, status_code, status_msg) = match (request.m, request.p.to_string().as_str()) {
            (http_util::HttpMethod::Get, "/") => (MAIN_PAGE, 200, "Ok"),
            (http_util::HttpMethod::Get, "/profile") => (PROFILE_PAGE, 200, "Ok"),
            _ => (ERROR_PAGE, 404, "Not Found"),
        };

        // レスポンスを作る
        let response = http_util::HttpResponse{
            v: http_util::HttpVersion::Http11,
            s: (status_code, status_msg),
            h: HashMap::new(),
            b: page.to_string(),
        };

        println!("{}", response);

        // bytesに直して送信する
        let response = response.to_string();
        let response = response.as_bytes();
        _ = stream.write(response);
        _ = stream.flush(); // 忘れずに
    }
}

```
