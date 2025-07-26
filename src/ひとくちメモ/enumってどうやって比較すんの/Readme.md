# (rust の)enum ってどうやって比較すんの

初歩的な記事だなぁ?と思ったやってみろ!

# 🎵 本日の一曲

<iframe width="312" height="176" src="https://ext.nicovideo.jp/thumb/sm40345422" scrolling="no" style="border:solid 1px #ccc;" frameborder="0"><a href="https://www.nicovideo.jp/watch/sm40345422">それでも人が好き feat.可不</a></iframe>

制服可不ちゃんかわいいね。  
同位体メンバーの中でも特に有名なのが可不ちゃんだと思うけれど、可不ちゃんってｾｲｼｭﾝ！って雰囲気があまりない気がします。  
~~でも可不ちゃんとｾｲｼｭﾝ!したかったなぁ~~(※決してやましい意味ではない)

# enum で比較してみろよ！

え？これだけでしょ？

```rust
enum 同位体 {
    可不,
    星界,
    裏命,
    狐子,
    羽累,
}

fn main() {
    println!("{}", 同位体::可不 == 同位体::可不);
}
```

できた？できなかった？

# derive でトレイトをつける

ああ、そうだ。 思い出した。  
**ユーザー定義型(struct/enum)は初期では一切のトレイトがついてないから、自分でつけないといけないんだったな**

じゃあ、`==`演算子に対応するトレイトはなんだっけ...

**`PartialEq`だな**

```rust
#[derive(PartialEq)]
enum 同位体 {
    可不,
    星界,
    裏命,
    狐子,
    羽累,
}

fn main() {
    println!("{}", 同位体::可不 == 同位体::可不);
}
```

よしできた。 てか、日本語も変数名に使えるんだな。(今更)

# derive を展開して意味を説明して？って言われた

derive は確かに便利なマクロだが、**たまに手動で実装したくなることもあるよな**

じゃあ、手動で実装してみよう。

```rust
enum 同位体 {
    可不,
    星界,
    裏命,
    狐子,
    羽累,
}
impl PartialEq for 同位体 {
    fn eq(&self, other: &Self) -> bool {
        self==other
    }
}

fn main() {
    println!("{}", 同位体::可不 == 同位体::可不);
}
```

...なんかコードがオーバーフローしたぞ？  
あそっか。 **そもそもそのコードは再帰関数を含んでるもんな**

```rust
enum 同位体 {
    可不,
    星界,
    裏命,
    狐子,
    羽累,
}
impl PartialEq for 同位体 {
    fn eq(&self, other: &Self) -> bool {
        self.eq(other) // 再帰関数になってる
    }
}

fn main() {
    println!("{}", 同位体::可不 == 同位体::可不);
}
```

...ん？ NN? え、じゃあどうやって比較しろと？

# enum を一意に比較する

[discriminant](https://doc.rust-lang.org/std/mem/fn.discriminant.html) という関数があるらしい。 これは**enum の値を一意に識別できる**ようだ。

```rust
use std::mem;

enum 同位体 {
    可不,
    星界,
    裏命,
    狐子,
    羽累,
}

fn main() {
    println!("{:?}", mem::discriminant(&同位体::可不));
    println!("{:?}", mem::discriminant(&同位体::星界));
    println!("{:?}", mem::discriminant(&同位体::裏命));
    println!("{:?}", mem::discriminant(&同位体::狐子));
    println!("{:?}", mem::discriminant(&同位体::羽累));
}
```

これは比較可能な`Discriminant`オブジェクトを返してくれる。 **これで比較すれば良さそうだ。**

```rust
enum 同位体 {
    可不,
    星界,
    裏命,
    狐子,
    羽累,
}
impl PartialEq for 同位体 {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

fn main() {
    println!("{}", 同位体::可不 == 同位体::可不);
}
```

**できた！**

# std::mem の紹介

`std::mem`は少し高度な用途で使う。 名前の通り、メモリ操作に特化している。

## drop: 手動で変数を drop する

```rust
// 引用: https://doc.rust-lang.org/std/ops/trait.Drop.html
struct HasDrop(i32);

impl Drop for HasDrop {
    fn drop(&mut self) {
        println!("Dropping HasDrop {}!", self.0);
    }
}

fn main() {
    let x = HasDrop(0);
    let y = HasDrop(1);
    std::mem::drop(x); // xを手動で初期化する
    println!("hello");
}
```

## swap: 内容を入れ替える

```rust
fn main() {
    let mut x = 10;
    let mut y = 20;
    println!("x: {}, y: {}", x, y);
    std::mem::swap(&mut x, &mut y);
    println!("x: {}, y: {}", x, y);
}
```

単純な関数に見えてしまうが、この関数の存在は、ここでは書ききれないほど大きい。  
まず、**両方の型が同じであればどの型でも使用できる**

> [!NOTE]
>
> ```rust, ignore
> pub const fn swap<T>(x: &mut T, y: &mut T) {
>     // SAFETY: `&mut` guarantees these are typed readable and writable
>     // as well as non-overlapping.
>     unsafe { intrinsics::typed_swap_nonoverlapping(x, y) }
> }
> ```
>
> こいつはポインタを操作して入れ替える。 わざわざメモリを初期化しないので**動作が早いのだ**
