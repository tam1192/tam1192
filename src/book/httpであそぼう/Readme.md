# HTTP であそぼう

HTTP を理解しつつ、HTTP サーバーを地道に作ります。

[なおこの本に惹かれたのはいうまでもありません。](https://doc.rust-jp.rs/book-ja/ch20-00-final-project-a-web-server.html)

## 基礎知識

### rust について

この記事では rust を主に取り扱います。 しかし説明は不要です。 というのもこのページを見る人が rust を知らないわけがないです。

### http について

この記事では http を主に取り扱います。 しかし説明は不要(ry)
**決してめんどくさいわけではありません。**

## markdown って HTTP 色付けできるんだぁ

```http
GET / HTTP/1.1
Content-Type: text/html; charset=utf-8;

<!DOCTYPE html>
<html>
  <head>
    <title>こんにちは</title>
  </head>
  <body>
    <h1>こんにちは</h1>
    <p>私は日記と申します。</p>
  </body>
</html>
```

discord だと HTML 部分も色付けされてた記憶があるので、mdbook は対応していると思ったのですが...
まぁこれでも十分です。

## この記事のメインコンテンツについて

rust 関係の話題より HTTP 系の話題の方がメインです。 rust 関係はこの後死ぬほど追求できるので。

## ご意見募集中

[当サイトのリポジトリ](https://github.com/tam1192/notepad.md/issues)にて、issue 募集中です!

- 投稿には github アカウントが必要です。
- テンプレート用意してます。 ぜひ活用してください。
  - [感想用テンプレート](https://github.com/tam1192/tam1192/issues/new?template=感想-コメント.md)
  - [誤解を招く内容への指摘](https://github.com/tam1192/tam1192/issues/new?template=誤解を招く内容への指摘.md)

# 作ったコードを置いてます

[github](https://github.com/tam1192/experiments-http-server.rs)にて作ってきたコードをまとめたものを置いてます。  
よかったら参考に...できるかな
