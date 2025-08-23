# rust で buffer を作る時は

## スタック上

スタック上におくのが一番安定

```rust
let buf = [0u8; 10];
```

## ヒープ上(Vec 使用時)

Vec など、ヒープにおくバッファーがほしい時は

```rust
let buf = vec![0u8; 10];
```

このように、0 であらかじめ埋めておくこと

### NG 例 1 new を使ってしまう

new は容量も決めないし、0 埋めもしないので一番あかんやつ

```rust
let buf = Vec::new();
```

### NG 例 2 with_capacity を使ってしまう

with_capacity で容量だけ決めておいて、初期化してないパターン

```rust
let buf = Vec::with_capacity(10);
```

### 妙案例 イテレーターを使う

イテレーターで初期化することもできる。

```rust
let buf = (0..10).map(|_| 0).collect::<Vec<u8>>();
```

`vec![]`は初期値を clone するのに対して、こちらは map で一つずつ指定していく。
buf は u8 なので問題ないが、clone の動作が特殊な Rc,Arc を使う時はイテレーターを使用することになる。
詳しくは、こちらの記事を参照されたし

[俺が知ってる rust の vec](https://qiita.com/tamkame123/items/f92bc46c441745aadf72)

## サイズ指定が必要な理由と 0 埋めが必要な理由

```rust, editable
fn read(buf: &mut [u8]) {
    buf[0] = b'h';
    buf[1] = b'l';
    buf[2] = b'l';
    buf[3] = b'o';
    buf[4] = b'w';
    buf[5] = b'\n';
}

fn main() {
    let mut buf = Vec::new();
    // 変更してみて。
    // let mut buf = Vec::with_capacity(10);
    read(&mut buf);
    println!("{:?}", buf);
}
```

これを実行すればわかる。

## ご意見募集中

[当サイトのリポジトリ](https://github.com/tam1192/notepad.md/issues)にて、issue 募集中です!

- 投稿には github アカウントが必要です。
- テンプレート用意してます。 ぜひ活用してください。
  - [感想用テンプレート](https://github.com/tam1192/tam1192/issues/new?template=感想-コメント.md)
  - [誤解を招く内容への指摘](https://github.com/tam1192/tam1192/issues/new?template=誤解を招く内容への指摘.md)
