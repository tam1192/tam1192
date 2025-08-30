# 最後の夏に毎日投稿してみた件

投稿日：2025/08/31

日記さん今年で学生生活終わるってよ

# 毎日投稿？

[このページを管理しているリポジトリにある main ブランチのコミット履歴](https://github.com/tam1192/tam1192/commits/main/)をみてみると...

```
44d63a8 2025-08-30 New/freewrite/寝たら体が痛くなる (#107)
fd4ee69 2025-08-29 New/coding-nikki/parser-2 (#106)
40a5997 2025-08-29 Change/coding-nikki/parser-1 (#105)
c0b5519 2025-08-28 New/coding-nikki/構文解析1 (#104)
0fb5ec0 2025-08-27 New/coding-nikki/crate-introduce/1/Readme.md (#103)
2890b2b 2025-08-26 書き終えた (#102)
71d1f10 2025-08-25 New/freewrite/新幹線で作業してみた
c4b1c51 2025-08-24 New/ひとくちメモ/リポジトリを作り直した件 (#100)
e69b0ed 2025-08-23 New/mc/RenderScaleはいいぞ
df16378 2025-08-23 RenameCate/ひとくちメモ/short-note
d64d3b9 2025-08-22 New/freewrite/究極のimmutableを追え
a7f3851 2025-08-21 New/Games/ブルアカ4.5周年
a351ec9 2025-08-20 New/FW/tam1192_repoのルール変えました
04553a6 2025-08-19 New/mc/テクスチャを貼る (#94)
bd0cc6d 2025-08-18 New/MC/fabric_2_
f7f08ef 2025-08-17 New/freewrite/kaikatu (#91)
504f049 2025-08-16 マインクラフト記事をCoddingNikkiから分離
ae75363 2025-08-16 New/gm/ba/hard (#89)
11cde77 2025-08-15 Fix/fabric/1 (#88)
5492d30 2025-08-15 New/mc/fabric/block作ってみた (#87)
5acce5a 2025-08-14 New/構造体で遊ぶ (#86)
513754c 2025-08-13 new/ひとくちメモ/github組織
09591d2 2025-08-12 書き終えた (#84)
2953f8f 2025-08-11 クリア後追記 (#83)
0dc1c8e 2025-08-11 Merge pull request #82 from tam1192:new/game/simulator/yknk
d873e04 2025-08-11 Merge branch 'main' into new/game/simulator/yknk
37d6e7e 2025-08-10 New/hitomemo/wine (#81)
08a3ec2 2025-08-09 可変変数の可変参照の項目を追加 (#80)
aa7816e 2025-08-09 alertの一部が正しい構文ではなかったため修正 (#79)
b1f40b1 2025-08-09 書きおわっった
aca0ad5 2025-08-09 強い怒りを書き終わった (#78)
f3d7e96 2025-08-08 書き終わった (#75)
ff44692 2025-08-07 書き終わった (#74)
168babb 2025-08-06 書き終わり (#73)
8ff012c 2025-08-06 fix/games/ba/nagituyo
5088b69 2025-08-05 end of content (#71)
7b1e0f3 2025-08-04 New/coding/crossterm (#70)
0a997fe 2025-08-03 Merge pull request #69 from tam1192:new/game/ba/nagituyo
b62cc66 2025-08-03 Merge branch 'main' into new/game/ba/nagituyo
cb76b75 2025-08-03 書き終わった
6f4d8a1 2025-08-03 games/baカテゴリ作成
141adce 2025-08-03 New/ひとくちメモ/おまえは存在しなかったをgitでやる方法 (#68)
6a59583 2025-08-03 新規ページ作成
75e5491 2025-08-03 gamesカテゴリを新規作成
83eb13f 2025-08-02 New/mc/paper/cmd-1 (#67)
ce66491 2025-08-01 New/server-nikki/openwrt/wireguard (#66)
c730d32 2025-08-01 New/book/ipの基礎知識 (#65)
```

(画像サイズ圧縮のため、`git log main --pretty=format:"%h %ad %s" --date=short`の出力で許してね)
こんな感じで大体毎日投稿してました。

途中からコミットメッセージを固定にしたりしてます。

# 実際に増えた記事

```diff
- [プロフィール](./profile.md)
- [コーディング日記](./coding-nikki/Readme.md)
   - [MyParserProject](./coding-nikki/my_parser_project/Readme.md)
   - [手を動かしながら git を使う](./coding-nikki/手を動かしながらgitを使う/Readme.md)
   - [Vue × WASM × Rust—試行錯誤とこれからの展望](./coding-nikki/Vue_×_WASM_×_Rust_試行錯誤とこれからの展望/Readme.md)
+  - [イケてる tui を作ってみる part1](./coding-nikki/crossterm/1/Readme.md)
+  - [イケてる tui を作ってみる part2](./coding-nikki/crossterm/2/Readme.md)
+  - [構造体で遊ぶ](./coding-nikki/fun_struct/Readme.md)
+  - [クレート紹介 part1](./coding-nikki/crate-introduce/1/Readme.md)
+  - [構文解析 part1](./coding-nikki/parser/1/Readme.md)
+  - [構文解析 part2](./coding-nikki/parser/2/Readme.md)

+- [Minecraft 系](./mc/Readme.md)
+  - [基礎編 part1](./mc/1/Readme.md)
+  - [基礎編 part2](./mc/2/Readme.md)
+  - [基礎編 part2+](./mc/2plus/Readme.md)
+  - [paper 編](./mc/paper/Readme.md)
    - [paper 編 part1 コマンド補完](./mc/paper/1/Readme.md)
  - [fabric 編](./mc/fabric/Readme.md)
    - [fabric 編 part1 仕様が大きく変わっちまったぜ](./mc/fabric/1/Readme.md)
    - [fabric 編 part2 とりまブロックを追加してみる](./mc/fabric/2/Readme.md)
  - [テクスチャ編 part1](./mc/tx1/Readme.md)
  - [RenderScale はいいぞ](./mc/RenderScaleはいいぞ/Readme.md)

 - [サーバー系](./server-nikki/Readme.md)
   - [Ceph-mon って何？](./server-nikki/ceph-monってなに？/Readme.md)
+  - [openwrt 同士で wireguard を張ってみる](./server-nikki/openwrt/wireguard/Readme.md)

 - [自由記述型](./freewrite/Readme.md)
   - [携帯電話はもはや電話じゃない](./freewrite/携帯電話はもはや電話じゃない/Readme.md)
   - [スマホ投稿テスト](./freewrite/スマホ投稿テスト/Readme.md)
   - [playground について](./freewrite/playgroundについて/Readme.md)
+  - [コミットしろよ](./freewrite/commit_must/1/Readme.md)
+  - [快活で作業してみた](./freewrite/kaikatu/Readme.md)
+  - ["tam1192"repo のルール変えました](./freewrite/tam1192_repo_2508/Readme.md)
+  - [究極の immutable を追え](./freewrite/究極のimmutableを追え/Readme.md)
+  - [新幹線で作業してみた](./freewrite/new_main_line/Readme.md)
+  - [関越道の工事が多くて草](./freewrite/kanetu/Readme.md)
+  - [練ればねれほど疲れる](./freewrite/long_sleep/Readme.md)

- [ひとくちメモ](./short-note/Readme.md)

  - [rust で buffer を作る時は](./short-note/rustでbufferを作る時は/Readme.md)
  - [ライフタイムを意識してコードを書く、その 1.rs](./short-note/ライフタイムを意識してコードを書く、その1.rs/Readme.md)
  - [mac のパッケージについて](./short-note/macのパッケージについて/Readme.md)
  - [(rust の)enum ってどうやって比較すんの](./short-note/enumってどうやって比較すんの/Readme.md)
  - [(rust の)from トレイトについて](./short-note/rustのfromトレイトについて/Readme.md)
  - [openwrt で ra を受け取る時に注意すること](./short-note/openwrt_ra/Readme.md)
+  - [git で誕生日をずらしてもらう方法](./short-note/gitで誕生日をずらしてもらう方法/Readme.md)
+  - [&mut 型と let mut の違い](./short-note/rust_mut_or_andmut/Readme.md)
+  - [mac の wine と、retina や d3d11 とかの話](./short-note/mac_wine/Readme.md)
+  - [Tailscale を使ってみた](./short-note/tailscale/Readme.md)
+  - [github で組織を形成する](./short-note/githubOrganizations/Readme.md)
+  - [リポジトリを作り直した件](./short-note/remake_repos/Readme.md)

+- [日記本](./book/Readme.md)
+  - [ネットワークってたぶんこんなもん](./book/networkって多分/Readme.md)
+    - [ネットワークってたぶんこんなもん part1](./book/networkって多分/1/Readme.md)
  - [HTTP であそぼう](./book/httpであそぼう/Readme.md)
    - [HTTP であそぼう part1](./book/httpであそぼう/1/Readme.md)
    - [HTTP であそぼう part2](./book/httpであそぼう/2/Readme.md)
    - [HTTP であそぼう part3](./book/httpであそぼう/3/Readme.md)
+    - [HTTP であそぼう part3.5](./book/httpであそぼう/3_5/Readme.md)

+- [games](./games/Readme.md)
+  - [simulator](./games/simulator/Readme.md)
+    - [焼肉シミュレーターやってみた](./games/simulator/yknk/Readme.md)
+  - [bluearchive](./games/bluearchive/Readme.md)
+    - [なぎちゃん強くしたら 400 位になったじゃんね ⭐︎](./games/bluearchive/nagituyo/Readme.md)
+    - [チャレンジミッション全クリを目指して](./games/bluearchive/hard/Readme.md)
+    - [4.5 周年](./games/bluearchive/45/Readme.md)
+  - [subnautica やってみた](./games/subnautica/Readme.md)
```

# なぜやってみたの？

ニコニコ動画で毎日投稿している人がいるのは知ってました。すげぇなと思ってみてました。  
動画は無理だけで記事だけならいけるんじゃね？って思ってやってみました。

# 大変だったところ

基本夏休みなので時間はたくさんあります。 しかし、途中でお盆やバイトなどのイベントもあり、この時に毎日投稿を途切らせない方法を考えてました。  
一番無難な方法だと思うのですが、記事を貯めておくことでした。  
記事は**長くすると書くのに時間がかかるし、読むのにも時間がかかります。**

少しずつ行ってますが、ひとくちメモなどに切り分けて行くといいのかもしれないです。

# 良かったこと

とりあえず毎日やることがあったのは良かった。

# 今後について

今後も適度に更新していきます。  
必要と思ったら増やしていきます。
