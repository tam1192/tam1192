# トレイトで遊ぶ

トレイトとかいう奥深いシステムについて

# 本日のひとこと

ニコニコの新検索ページ使いやすいですね。 アプリと表示が近くなったような。

# 基本的な

```rust
pub trait Element {
    type Number;
    fn first_pos(&self) -> (Self::Number, Self::Number);
    fn last_pos(&self) -> (Self::Number, Self::Number);
}
```

画面の要素って意味で Element というトレイトを作ってみた。 0 を基準とし、
