# mac のパッケージについて

mac のパッケージ管理が結構複雑だと感じたので書きました。

> [!NOTE]
> ある程度 unix などの知識があることを前提で書いてます。  
> あと備忘録というかメモなので、主観とかめっちゃ入ってます。

## 🎵 本日の一曲

<iframe width="312" height="176" src="https://ext.nicovideo.jp/thumb/sm45017533" scrolling="no" style="border:solid 1px #ccc;" frameborder="0"><a href="https://www.nicovideo.jp/watch/sm45017533">『 MYA◎KU  』歌唱:初音ミク/ 製作:ユゥレイの怪奇譚</a></iframe>

そういえば大阪万博行きたいな。 関東民から見て大阪のイメージといえばやっぱ新快速ではないでしょうか。  
街中を $130km/h$ で飛ばす爽快感がいいですよね。  
(多分そこ気にする人はあんまりいない。)

## app ファイルについて

app ファイルは、dmg(仮想ディスクイメージ)をダウンロードしてきて、それを Application ディレクトリなどに展開しておけば ok です。  
app ファイルは実はディレクトリで、必要なファイルがその中に収まってる。  
それ以上操作しなくても、アプリケーションは起動する。 消す際は app ファイルを削除すればいいだけ。

ただ、削除時は見て欲しいファイルが 3 つある。

1. `~/Library/Containers`
1. `~/Library/Caches`
1. `~/Library/Application\ Support`

明らかに、削除するアプリケーションのやつだったら消していいと思う。容量食うだけやし。

## pkg について

インストーラーでインストールするやつ。 今回これについてメモるために急遽書いた。
インストールしたファイルたちは`pkgutil`で管理可能な模様。

本題ではあるものの、ここではリンクのみに納める。　というのも私も理解しきってないから。  
[Snow Leopard の新コマンド「pkgutil」でパッケージを削除する ](https://www.next-season.net/mac/124/)

## brew について

mac 版パッケージマネージャー。 apt や pacman に例えられるが、だいぶ違う。

ところで、パッケージマネージャとはなんだろうか。 パッケージという単位でソフトウェアやライブラリを管理するプログラムで、**パッケージ依存**や**パッケージ更新**などの管理を行う。
パッケージマネージャーは二(+1)種類に分けて考えることができる。

- システムファイルを直接いじるやつ
  システムディレクトリで直接パッケージを管理するもの。  
  パッケージによっては削除するだけで**OS が起動しなくなる**などのトラブルが付き纏う。
  - プラットフォーム固有  
    `apt pacman dnf`など、**デフォルトでインストールされてる奴ら**
  - 共通
    `pip` これもシステムと密接に関係していることが多いのでこちらに分類。
- 独立してるやつ
  システムファイルとは独立したディレクトリでパッケージを管理するもの。  
  パッケージを削除しても**直接システムには影響しないことが多い。**  
  `brew snap pip cargo sdkman! npm` など

brew はシステムファイルとは独立しており、最新版だと`/opt/homebrew`でパッケージを管理する。

## ご意見募集中

[当サイトのリポジトリ](https://github.com/tam1192/notepad.md/issues)にて、issue 募集中です!

- 投稿には github アカウントが必要です。
- テンプレート用意してます。 ぜひ活用してください。
  - [感想用テンプレート](https://github.com/tam1192/tam1192/issues/new?template=感想-コメント.md)
  - [誤解を招く内容への指摘](https://github.com/tam1192/tam1192/issues/new?template=誤解を招く内容への指摘.md)
