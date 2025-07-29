# playground について

プログラミング言語を試せるサイト「Playground」と、mdbook についてメモるために書きました。

# 🎵 本日の一曲

<iframe width="312" height="176" src="https://ext.nicovideo.jp/thumb/sm33546451" scrolling="no" style="border:solid 1px #ccc;" frameborder="0"><a href="https://www.nicovideo.jp/watch/sm33546451">少女レイ／初音ミク</a></iframe>　

夏です。暑いです。

# rust の playground

[rust の playground](https://play.rust-lang.org)

シンプルな見た目ですが、割と機能が豊富です。

## rustfmt

rustfmt は、実際の PJ では`cargo fmt`で実行できるコードフォーマッターです。  
Playground でもこれが実行できます。 右上の`tools` -> `rustfmt`です。

## あのキーバインドが使える

`vi`,`emacs`などが使えます。 内部で[ace エディタ](https://ace.c9.io)が動いている模様?  
右上の`config` -> `editor`を ace に -> `config`の内容が代わり、`keybind`が選べるようになる

## mdbook 埋め込み対応

[mdbook](https://github.com/rust-lang/mdBook)に埋め込みができる。

```
# fn main() {
    println!("hello");
# }
```

というコードブロックを書くと、

```rust
# fn main() {
    println!("hello");
# }
```

必要な部分だけを見せることができるとか、

```rust,editable
fn main() {
    println!(""); // ここを変更
}
```

`rust,editable`とすれば、読者が編集できるようになったりと、独自機能がたくさんついてる。

# kotlin の playground

[play.kotlinlang.org](https://play.kotlinlang.org/)にて利用可能。

<iframe width="400" height="200" style="border:solid 1px #ccc;" frameborder="0" src="https://pl.kotl.in/SIbCigmPc" scrolling="no" ></iframe>

埋め込みにも対応してる。 (iframe タグで埋め込む)

# html/css/js の playground

[livecoders](https://v46.livecodes.io)が有能だと思う。
これも埋め込み対応。

<iframe title="Untitled Project" scrolling="no" loading="lazy" style="height:300px; width: 100%; border:1px solid black; border-radius:6px;" src="https://v46.livecodes.io/?x=id/vtsm4ije59j&embed=true">
  See the project <a href="https://v46.livecodes.io/?x=id/vtsm4ije59j" target="_blank">Untitled Project</a> on <a href="https://livecodes.io" target="_blank">LiveCodes</a>.
</iframe>

いろいろな方法で埋め込められるけど、mdbook なら iframe がおすすめかな?
