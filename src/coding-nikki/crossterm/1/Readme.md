# イケてる tui を作ってみる part1

tui: gui ではないが、コマンドラインで操作できる ui のこと

# 🎵 本日の一曲

<iframe width="312" height="176" src="https://ext.nicovideo.jp/thumb/sm44374182" scrolling="no" style="border:solid 1px #ccc;" frameborder="0"><a href="https://www.nicovideo.jp/watch/sm44374182">牛タン☆クラブナイト / 初音ミク</a></iframe>

ぎゅうたんたん

# とりあえず一句

```rust
# extern crate crossterm;
use std::io::{Result, stdout};

use crossterm::{ExecutableCommand, cursor::MoveDown, execute, style::Print};

fn main() -> Result<()> {
    stdout()
        .execute(MoveDown(4))?
        .execute(Print("hello world"))?;
    Ok(())
}
```

```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/train_signal_sim`




hello world
```

行列自由に扱えるようになりますね。

## queue について

```rust
# extern crate crossterm;
use std::{
    io::{Result, Write, stdout},
    thread::sleep,
    time::Duration,
};

use crossterm::{QueueableCommand, style::Print};

fn main() -> Result<()> {
    let mut stdout = stdout();
    stdout.queue(Print("hello\n"))?;
    sleep(Duration::from_secs(1));
    stdout.queue(Print("world\n"))?;
    stdout.flush()
}
```

1 秒経って

```
hello
world
```

と表示されると**思っていた**

実際には hello と表示された後に 1 秒待機になったで。

### 改行`\n`を抜いてみた

```rust
# extern crate crossterm;
# use std::{
#     io::{Result, Write, stdout},
#     thread::sleep,
#     time::Duration,
# };
use crossterm::{QueueableCommand, style::Print};

fn main() -> Result<()> {
    let mut stdout = stdout();
    stdout.queue(Print("hello"))?;
    sleep(Duration::from_secs(1));
    stdout.queue(Print("world"))?;

    stdout.flush()?;
    Ok(())
}
```

想定通り、1 秒経った後に`helloworld`が表示された。

### movedown で改行しよう

```rust
# extern crate crossterm;
# use std::{
#     io::{Result, Write, stdout},
#     thread::sleep,
#     time::Duration,
# };
use crossterm::{QueueableCommand, cursor::MoveDown, queue, style::Print};

fn main() -> Result<()> {
    let mut stdout = stdout();
    queue!(stdout, Print("hello"), MoveDown(1))?;
    sleep(Duration::from_secs(1));
    queue!(stdout, Print("world"), MoveDown(1))?;
    stdout.flush()?;
    Ok(())
}
```

```
hello
     world
```

私はそんなスタイリッシュなのを求めていない。

### movetonextline で改行しよう

```rust
# extern crate crossterm;
# use std::{
#     io::{Result, Write, stdout},
#     thread::sleep,
#     time::Duration,
# };
use crossterm::{cursor::MoveToNextLine, queue, style::Print};

fn main() -> Result<()> {
    let mut stdout = stdout();
    queue!(stdout, Print("hello"), MoveToNextLine(1))?;
    sleep(Duration::from_secs(1));
    queue!(stdout, Print("world"), MoveToNextLine(1))?;
    stdout.flush()?;
    Ok(())
}
```

```
hello
world
```

想定通りに動作したね。

# m さんみたいなやつ

```

               #?#

              #?#?#

 o
 |
###################################
```

これを書いてみよう。

```rust
use std::{
    io::{Result, Write, stdout},
    thread::sleep,
    time::Duration,
};

use crossterm::{
    cursor::{MoveTo, MoveToNextLine},
    queue,
    style::Print,
    terminal::{Clear, ClearType, size},
};

fn main() -> Result<()> {
    let mut stdout = stdout();

    // 画面を初期化
    queue!(stdout, Clear(ClearType::All))?;

    // サイズを確認する
    // 左上を1としてスタート
    // https://docs.rs/crossterm/latest/crossterm/terminal/fn.size.html
    let (col, row) = size()?;

    // 床の描画
    for c in 0..col {
        queue!(stdout, MoveTo(c, row), Print("#"))?;
    }

    stdout.flush()?;
    Ok(())
}
```

とりあえず床だけ描画してみることにした。

```



#######################
```
