# クレート紹介 part1

書くネタないわけでは）ないです。

# 本日の一言

<iframe width="312" height="176" src="https://ext.nicovideo.jp/thumb/sm45329405" scrolling="no" style="border:solid 1px #ccc;" frameborder="0"><a href="https://www.nicovideo.jp/watch/sm45329405">………</a></iframe>

これはやばい。 浄化されて消えかける。

# anyhow

エラー処理がぐーんと楽になるやつです。 標準で Error トレイトが存在するのは知ってましたが、使わざる得なくなりました。

## anyhow::Result

anyhow::Result を使えば、他のことを考えないで済むようになりました。

## anyhow::anyhow

anyhow マクロです。

```rust
# extern crate anyhow;
use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let reason = "エラープログラムだから";
    Err(anyhow!("原因は{}", reason))
}
```

Error トレイトを実装したオブジェクトを文字列から簡単に作れます。  
format も使えるので、`{:?}`でオブジェクトの中身を表示させられたりすることもできます。

## anyhow::bail ensure

```rust
# extern crate anyhow;
# extern crate rand;
use anyhow::{Result, bail};

fn main() -> Result<()> {
    if rand::random() {
        bail!("random error");
    }
    Ok(())
}

```

`anyhow!`に加え、`return Err()`まで実装してくれるマクロです。

```rust
# extern crate anyhow;
# extern crate rand;
use anyhow::{Result, ensure};

fn main() -> Result<()> {
    ensure!(rand::random::<bool>(), "random error");
    Ok(())
}

```

`assert!`と同じような効果を持ってます。

## 使い方

- [anyhow の docs.rs](https://docs.rs/anyhow/latest/anyhow/index.html)

# strum

enum を使いやすくしてくれます。

## enum との変換が楽になる

```rust, ignore
#[derive(Debug, Clone, PartialEq, Eq, strum_macros::Display, strum::EnumString)]
pub enum Version {
    #[strum(serialize = "HTTP/1.0")]
    Http10,
    #[strum(serialize = "HTTP/1.1")]
    Http11,
    #[strum(serialize = "HTTP/2.0")]
    Http20,
    #[strum(serialize = "HTTP/3.0")]
    Http30,
}
```

`EnumString`は`FromStr`トレイトを実装してくれます。  
おかげで、

```rust, ignore
Version::from_str("HTTP/1.0");
```

と、文字列から enum オブジェクトを生成できる他

`Display`によって、トレイトのディスプレイを自動で実装できます。

上記のように、オブジェクトを持つというより、**バージョンや定数を管理する時に**楽になります。

## 使い方

- [strum の docs.rs](https://docs.rs/strum/latest/strum/)
