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

# AST

木構造とかいう奴らの一つ。

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

おおよそどこに何があるか、教えてくれる。
