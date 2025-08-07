# イケてる tui を作ってみる part2

今回は標準入力を試してみます

# 🎵 本日の一曲

<iframe width="312" height="176" src="https://ext.nicovideo.jp/thumb/sm32492001" scrolling="no" style="border:solid 1px #ccc;" frameborder="0"><a href="https://www.nicovideo.jp/watch/sm32492001">何でも言うことを聞いてくれるアカネチャン</a></iframe>

たまたま目に入ったので

<details><summary>百合注意</summary>

やっぱ琴葉姉妹百合最高だよな

<iframe width="312" height="176" src="https://ext.nicovideo.jp/thumb/sm43928543" scrolling="no" style="border:solid 1px #ccc;" frameborder="0"><a href="https://www.nicovideo.jp/watch/sm43928543">何を言っても琴葉姉妹？【VOICEROID劇場】</a></iframe>

...

</details>

# リアルタイムで標準入力を受け取ってみよう

こんなかんじ

```rust, ignore
use std::{
    io::{Result, Write, stdin, stdout},
    thread::sleep,
    time::Duration,
};

use crossterm::{
    cursor::{MoveTo, MoveToNextLine},
    event::{
        EnableBracketedPaste, EnableFocusChange, EnableMouseCapture, Event, KeyEvent,
        KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags, poll,
        read,
    },
    execute, queue,
    style::Print,
    terminal::{Clear, ClearType, enable_raw_mode, size},
};

fn main() -> Result<()> {
    // 色々な機能を無効にする
    _ = enable_raw_mode();
    // printlnなどの動作がおかしくなるので、crosstermの関数を使うようにする。

    let mut stdout = stdout();

    loop {
        // 入力イベントの処理
        if poll(Duration::from_millis(100))? {
            if let Event::Key(event) = read()? {
                execute!(stdout, MoveTo(0, 0), Print(format!("{:?}", event)))?
            }
        }
    }
}
```

とりあえずキーイベント only

```rust, ignore
KeyEvent {
    code: Char('a'),
    modifiers: KeyModifiers(0x0),
    kind: Press,
    state: KeyEventState(0x0)
}
```

こんな感じの構造体が event の中に入ってる。

> [!NOTE]
> raw モードにしないと、**enter を受け取るまで入力を待機してしまう**

> [!NOTE]
> poll 関数を挟むと、(上記の例だと)100ms まって入力がなければ、false を返す(=イベントの処理をスキップする)

# イベント楽しい

main 関数に設置した loop 文で、入力、処理、出力を繰り返す。
これを応用すると、ゲームとか作れる

# おわり

subnautica、シーモス食われました。  
資材集め大変です。
