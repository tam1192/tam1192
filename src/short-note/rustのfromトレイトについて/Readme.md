# (rust の)from トレイトについて

from トレイトについて書き綴ります。

# 🎵 本日の一曲

<iframe width="312" height="176" src="https://ext.nicovideo.jp/thumb/sm44581955" scrolling="no" style="border:solid 1px #ccc;" frameborder="0"><a href="https://www.nicovideo.jp/watch/sm44581955">キミにしか描けない世界 / 音街ウナ</a></iframe>

元気で可愛くて好き

# From トレイトってなんだよ

> From
>
> From トレイトは、ある型に対し、別の型からその型を作る方法を定義できるようにするものです。そのため、複数の型の間で型変換を行うための非常にシンプルな仕組みを提供しています。標準ライブラリでは、基本データ型やよく使われる型に対して、このトレイトが多数実装されています。
>
> [引用](https://doc.rust-jp.rs/rust-by-example-ja/conversion/from_into.html)

from 以外だと、as というのが存在します。 これはキャストに近いやつですね。  
from は自分が作った型(struct, enum)に対してキャストする手段を提供するものです。

## トレイトにジェネリクスがあるという意味

from はトレイトにジェネリクスがついています。

```rust, ignore
impl From<&str> for MyStruct {
    fn from(s: &str) {}
}

impl From<i32> for MyStruct {
    fn from(s: i32) {}
}
```

実装時は上記のように、ジェネリクスに型をぶち込みます。(i32, str など)  
そうすると、**from メソッドの引数の型がそれに合わせて変わります**  
**型が違えば同じ型に From トレイトを複数適用可能です。**

**このような方法で、型によって処理の異なる、同名なメソッドを実装できます。**

## トレイトってすごいよね。

トレイトは、他言語でいうインターフェイスとよく例えるのですが、機能はそれ以上です。  
あと、同時にジェネリクスも強いですよね。

## 以上

すまない！　毎日投稿を実現したくて急遽書いた！
内容が薄いから今度追記します...
