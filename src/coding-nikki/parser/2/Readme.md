# 構文解析 part2

今回は一瞬で終了します。 ひとくちメモレベル

# 本日の一言

ペロロジラのおかげで、自分が持つキャラの中でも神秘キャラが一番弱いことに気づきました。
思えば振動キャラが少ないと思ってたので、そればかり強化してたし、神秘キャラとをほぼ同一視してたのでこうなることは仕方なかったと思います。  
現時点では、取り急ぎワカモと臨戦ホシノを強化することにしました。 アビドスのノートほぼ残ってない...

<iframe width="312" height="176" src="https://ext.nicovideo.jp/thumb/sm45328090" scrolling="no" style="border:solid 1px #ccc;" frameborder="0"><a href="https://www.nicovideo.jp/watch/sm45328090">六花ちゃんとお近づきになりたい葵ちゃん</a></iframe>

びびりっか

# パーサークレートについて

rust には nom というパーサークレートが存在する。  
chumsky というパーサークレートも存在する。

使い分け、わからん。

[When to use this instead of nom? (chumsky の issue より)](https://github.com/zesterer/chumsky/discussions/406)曰く、

> (要約)  
> nom はバイナリ向け。 chumsky は文字列向け。

とのこと。

あと、Logos というクレートも存在し、これは**Lexer**と呼ばれるもの。

[Understanding the Differences – Lexer vs. Parser Explained](https://skillapp.co/blog/understanding-the-differences-lexer-vs-parser-explained/)

どれも文字列やバイナリ列などのコードを解釈し、オブジェクトやトークンに直すもの、そう理解してる。

# Logos を使ってみる

```rust, ignore
use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
enum Token {
    // Tokens can be literal strings, of any length.
    #[token("+")]
    Add,

    #[regex("[0-9]")]
    Number,
}

fn main() {
    let mut lex = Token::lexer("1 + 1");
    assert_eq!(lex.next(), Some(Ok(Token::Number)));
    assert_eq!(lex.next(), Some(Ok(Token::Add)));
    assert_eq!(lex.next(), Some(Ok(Token::Number)));
}
```

こんな感じで、大雑把にそれが何なのかを表してくれる。
文字列で出力するようにすれば

```rust, ignore
fn main() {
    let mut lex = Token::lexer("1 + 1");
    println!("{:?}, {:?}", lex.next().unwrap(), lex.span());
    println!("{:?}, {:?}", lex.next().unwrap(), lex.span());
    println!("{:?}, {:?}", lex.next().unwrap(), lex.span());
}
```

```
Ok(Number), 0..1
Ok(Add), 2..3
Ok(Number), 4..5
```

## example の json を動かしてみる

[コードはこれ](https://github.com/maciejhirsz/logos/blob/master/examples/json.rs)
git clone で持ってきた方が楽。

`cargo run --example json -- <filename>.json`
で動かせる。

```json
{
  "name": "yjsnpi",
  "age": 24
}
```

```
Object(
    {
        "\"name\"": String(
            "\"yjsnpi\"",
        ),
        "\"age\"": Number(
            24.0,
        ),
    },
)
```

## 感想(2025/08/29 時点)

ほぼ私が想像するパーサー？  
言語はもちろん、json みたいな独自のデータ記述言語を扱う時には Logos を使った方が手っ取り早いかもしれない。

# chumsky を使ってみる

```rust,ignore
use chumsky::{
    Parser,
    error::Simple,
    extra,
    prelude::{any, end, just},
};

fn main() {
    let base = ":helloworld";
    let parser = just::<_, _, extra::Err<Simple<char>>>(':')
        .then(any().repeated())
        .then(end());
    let x = parser.parse(base);

    println!("{:?}", x);
}
```

parse というメソッドを使うことでパースされます。  
them メソッドでパーサーを連結させることができます。

## ドキュメント

[ガイド](https://docs.rs/chumsky/latest/chumsky/guide/_00_getting_started/index.html)
をベースに軽く触れてみました。

## repeated について

repeated メソッドを使うことによって、パーサーを繰り返し実行することができるようです。any パーサーは終端文字以外の全てにマッチするようなので、**マッチする間は繰り返されます**。

ところでこれ、イテレーターに近い動作をします。

## IterParser

[IterParser](https://docs.rs/chumsky/latest/chumsky/trait.IterParser.html) repeated メソッドをつけると、実装されたパーサーになります。  
と言いつつ、使えるのは fold および collect のようです。 map や filter は**repeated の手前で**行うらしい。

## 感想(2025/08/29 時点)

まだ正気よくわかってない。

# nom

だいぶ前から使ってきた。

## stream vs complete

最初のページに書かれている通り、**一部のパーサーで動作が変わってくる。**  
alpha0 は 0 以上のアルファベットキャラクターをパースする。正規表現で言う`[a-zA-Z]*`である。

`"abcd"`のように仮に全てパースに成功する場合でも、stream は続きがあると想定され、パースが失敗するまで(`;`などの文字にぶち当たるまで)はエラーを出力する。
