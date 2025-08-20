# 究極の immutable を追え

そういうお話です。

# 🎵 本日の一曲

<iframe width="312" height="176" src="https://ext.nicovideo.jp/thumb/sm44759090" scrolling="no" style="border:solid 1px #ccc;" frameborder="0"><a href="https://www.nicovideo.jp/watch/sm44759090">Cat Swing feat. 小夜／SAYO</a></iframe>

ねこすい

# イミュータブルとは？

不変を意味する。

## rust の例

rust の変数は、`mut`をつけないと変更できない。
変更されているように見えるのは**シャドーイングされてるだけ**。

```rust
# fn main() {
    let x = 3;
    let x = 4; // シャドーイング
    let mut x = 3;
    x = 4; // ミュータブル
# }
```

## kotlin の例

`val`をつけると immutable になる。 `var`にするとミュータブルになる。
**シャドーイングは推奨されてない。**

## OS での例

[ハーベスター](https://w.wiki/F5on)という仮想化基盤では、ベアメタル上の OS がイミュータブルとなってる。  
言い換えると、**再起動すると中身がリセットされる。**

代わりに、Kubernetes の設定を etcd に、一部の設定ファイルのみを可変とすることで、**必要最低限の可変が許容されてる**  
言い換えると、**変更できるところだけ残るようになってる。**

ルーター機なども rom と ram、nvram、フラッシュメモリと必要に応じてメモリを使い分けている。

メリットは**ランサムウェアへの耐性が強くなること**  
不変の世界じゃ、暗号化する(変更する)、消すという作業もできないからね。

#
