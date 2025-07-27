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

from 以外だと、as というのが存在します。 これはキャストに近いやつですね。しかしそっちは安全じゃないです。
