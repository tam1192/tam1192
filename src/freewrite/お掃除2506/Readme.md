# お掃除2506
お掃除がてら数年前の自分を振り返りたいなと思ったので描きました。

# 命名規則や公開範囲とかについて
副操縦士に聞いたところ、`-`区切りで書くのが良いらしい。

> 面白いですね！遊びで試したプロジェクトやメモを整理するなら、**カテゴリ**ごとに名前を決めると管理しやすいかもしれません。例えば：
>
> - **sandbox-***：試作や遊びで作ったコード
> - **prototype-***：アイデアを形にしたもの
> - **experiments-***：技術検証用
> - **archive-***：過去のプロジェクトやメモの保存庫
> - **notes-***：設計やアイデアのメモ
> 
> 例えば、Tailwindの新しいユーティリティを試したなら `sandbox-tailwind-experiments` 、Nuxt×Ollamaのアイデアなら `prototype-ai-gui` みたいな感じ。
> 
> あとは**用途**別に分けるのもいいですね：
> - `fun-projects`
> - `small-tools`
> - `random-ideas`
> 
> 何か気に入るものはありますか？一緒にいい名前考えましょうか！

そもそも遊びのコードをそのまま放置するのは良くない、それこそ[tmp使えや！](/freewrite/tmp使えや!)ですよ！  
とはいえ、残したいと言う気分があるので、ネタに残せるレベルだったら残す方針で。

# 思い出話
整理しながら懐かしいrepo触ってるので、せっかくだし思い出話をしてしまおうと思います。

## vox-ttsシリーズの開発と試行錯誤  

**vox-tts**は、VoicevoxのREST APIを活用したDiscord TTSボットで、私が最初に「書き切った！」と感じたプログラムのひとつ。Pythonで始め、のちにJSへ移行しました。この過程で得た知見を振り返ります。  

開発のきっかけは、VoicevoxがREST API経由で音声合成できると知った瞬間の衝動。「これはやらねばならぬ」とコードを書き始めました。Python (`discord.py`) を使いましたが、後にJS (`discord.js`) へ移行。その理由は、ライブラリの不安定さよりも**応答速度の改善を期待**してた。しかし、最終的にはVoicevoxの生成時間がボトルネックだったため、この変更はそんなに意味がなかったです。

とはいえ、JSを選んだことで、イベント駆動型の設計が活きるようになりました。Discordのような「いつ発生するかわからない処理」に向いていて、非同期処理の管理がしやすい。特に**Promise.all**が便利で、APIコールの最適化に活用しました。例えば、複数のデータ取得を並列化することで、待機時間を短縮できます。  

```js
const [userData, posts, comments] = await Promise.all([
  fetch('/api/user').then(res => res.json()),
  fetch('/api/posts').then(res => res.json()),
  fetch('/api/comments').then(res => res.json()),
]);

console.log(userData, posts, comments);
```
(Promise.allの一例)

マルチスレッドと比較すると、JSの非同期処理は**考えることが少なくて済む**のが強み。スレッド管理や共有変数の競合を気にする必要がなく、`await`を必要な箇所だけに書けばいいのが直感的です。  

**vox-tts**の開発を通じて、PythonとJSそれぞれの特性や非同期処理を深く理解できました。イベント駆動型の設計は、リアルタイム処理を必要とするコードには最適です。  

### リポジトリへのリンク
- [vox-tts.py](https://github.com/tam1192/vox-tts.py)
- [vox-tts.js](https://github.com/tam1192/vox-tts.js)

### 参考など
- [voicevox - 無料で使える中品質なテキスト読み上げ・歌声合成ソフトウェア](https://voicevox.hiroshiba.jp)
- [discord.py](https://discordpy.readthedocs.io/ja/latest/index.html)
- [discord.js](https://discord.js.org)

## node-dirtools: 非同期処理を活用したファイル管理ツール
node-dirtools は、vox-ttsのために開発したディレクトリ処理ツールで、Promise.allをフル活用した設計になっています。 主な機能はシンプルで、ディレクトリ内のファイルを非同期的に読み込み、特定の処理を実施すること。特に、複数の設定ファイルを同時に読み込む仕組みを作ることで、柔軟なファイル管理が可能になりました。

### リポジトリへのリンク
- [node-dirtools](https://github.com/tam1192/node-dirtools)

## experiments-blockview.rs: Rustとの出会いと試作コード  

Rustに触れ始めたきっかけは、**ゲームのmod修正**でした。  
致命的なバグが放置されていて、誰も修正しようとしていなかったので、「ならば自分で直してみよう」と思ったのが始まり。Rustの本を読みつつ、当時流行っていたAIも活用しながらコードを書き進めました。  

最初は試行錯誤の連続で、書いたコードも拙いものでした。**unsafeを使いまくる方法**を取っていたので、PRを出した際に大量の指摘を受けました。しかし、それが結果的にRustの型システムや所有権の概念を深く理解する良い機会になったと思っています。  

### Rustで書いた試作コード  

Rustを学ぶために書いたコードのひとつに、**8色程度の色データを配列に入れ、それを標準出力に四角絵文字で表示するプログラム**があります。例えば：

🟥🟦🟨🟩  

実装方法としては、色データを**enum**で管理し、出力時には対応する絵文字を使う形を採用しました。  
一見シンプルですが、後から見返すと**swapを使ってデータを入れ替えたり、二次元配列で管理すべきデータを一次元配列に縛ったり**と、なかなか無茶な設計をしていたことに気付きました。  

とはいえ、この試作を通じてRustの型管理や所有権の扱い、さらにはデータ構造の設計について多くの学びが得られました。  
今後はbmpに保存できるようにしたい...(現在やってる)

### リポジトリへのリンク  
- [experiments-blockview.rs](https://github.com/tam1192/experiments-blockview.rs)  
- [gm_turbostroi_rust(修正したmod)](https://github.com/tam1192/gm_turbostroi_rust)

## experiments-wasm-vue
名前変えました。 そして書いてたら長くなったのでページ分けました
-> [Vue × WASM × Rust—試行錯誤とこれからの展望](./coding-nikki/Vue_×_WASM_×_Rust_試行錯誤とこれからの展望/Readme.md)  

