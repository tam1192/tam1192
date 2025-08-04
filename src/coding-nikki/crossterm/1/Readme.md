# イケてる tui を作ってみる part1

tui: gui ではないが、コマンドラインで操作できる ui のこと

# 🎵 本日の一曲

<iframe width="312" height="176" src="https://ext.nicovideo.jp/thumb/sm44374182" scrolling="no" style="border:solid 1px #ccc;" frameborder="0"><a href="https://www.nicovideo.jp/watch/sm44374182">牛タン☆クラブナイト / 初音ミク</a></iframe>

ぎゅうたんたん

# とりあえず一句

```rust
use std::io::{Result, stdout};

use crossterm::{ExecutableCommand, cursor::MoveDown, execute, style::Print};

fn main() -> Result<()> {
    stdout()
        .execute(MoveDown(4))?
        .execute(Print("hello world"))?;
    Ok(())
}
```
