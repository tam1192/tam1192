# [Algorithm.rs について](https://tam1192.github.io/algorithm_rs/index.html)
アルゴリズム勉強のメモを残すのをnotepad.mdとは別に作ってしまった。

ただ、ネタがない。 多分atcoderとかの問題をひねったやつをそのまま書いてたり、
もしくはマジモンのwriteupになったりすると思います。

## mdbookをさらに強くしてみた。
まずLatex...数式を導入してみた。  
みました。

$$ \frac {6} {3} = 2 $$

Latexとの出会いはそんな最近ではなく、数年前から数式のメモをするために使っていました。  
...綺麗だね。　自分がペンで書くより綺麗に描画されるから、質の良いノートを取れる気分になるよ。

## かさねてと
mdbookとcargo docをかさねてみと   

mainブランチで試行錯誤してました。 mdbookとdoc生成をそれぞれ別のjobで行い、　`actions/upload-artifact@v4` 、 `actions/download-artifact@v4`で合成しました。
jobを分けたのは、まぁめんどかったからです。

mdbookの中にドキュメントを埋め込む形にしました。　最初にmdbook側のページが出て、パスを深くするとdocが出てきまし。 mdbook側で誘導します。

あんま綺麗にできなかったよね。　研究の余地ありです。

[workflowについてはこちら](https://github.com/tam1192/algorithm_rs/blob/main/.github/workflows/gh-pages.yml)

## editableでrunnableなコードブロック
mdbookの実力をまだ試してませんでした。　実はnodejsとか、vueとかでもできるらしいですが、  
まぁまずはrustだけでいいやろと。

```rust,editable
fn main() {
    let x = 10; //変更してみて
    println!("{}", x);
}
```

表現の幅が広がりそうです。
