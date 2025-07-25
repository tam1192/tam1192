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

response は少し待って欲しい。  
とりあえず 40 分くらいかけてファイルを整理してみました。 40 分か。  
というのも、テストも大量に増やしてみたからです。ドキュメント書こうとすると 2 時間ぐらい取られそうですね。

自分か感じる以上にものを書くのは時間がかかるということです。まぁ手書きよりかはましですが...

## テストについて

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
一応 mdbook の機能で`{{#include file.rs}}`というのが存在するので、うまく活用すれば...
