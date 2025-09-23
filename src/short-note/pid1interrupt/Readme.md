# pid1 に割り込む奴

お久しぶりです。

# 本日の一言

だいぶ涼しくなりましたね。私は冬の方が好きです。

<iframe width="312" height="176" src="https://ext.nicovideo.jp/thumb/sm40027376" scrolling="no" style="border:solid 1px #ccc;" frameborder="0"><a href="https://www.nicovideo.jp/watch/sm40027376">姉二人、雪遊び。【VOICEROID劇場】</a></iframe>

こういう光景も冬ならではですよね。

<iframe width="312" height="176" src="https://ext.nicovideo.jp/thumb/sm45379528" scrolling="no" style="border:solid 1px #ccc;" frameborder="0"><a href="https://www.nicovideo.jp/watch/sm45379528">ミライシナリオ/feat.東北ずん子&初音ミク</a></iframe>

最近、「イラスト統一祭」っていう面白い祭りやってました。 その中で神曲見つけました。

# pid1 に割り込みたくなる時もある

とはどういうことか。  
Docker では、PID1 プロセスの標準出力をそのままログとして保存する仕組みがあります。
しかし、PID1 以外のソフトウェア（例えば pid1 がシェルスクリプトであり、スクリプト中に本来必要なプログラムを動作させる時）のログを残すにはどうすれば良いでしょうか

# proc を使う

`/proc`は仮想的なファイルです。実在しません。  
proc は linux 特有の仕組みです。proc ディレクトリの中にある数字のディレクトリは、pid となっています。

## 中身にあるもの： environ

environ というファイルには、環境変数が表示されます。  
環境変数にパスワード設定すると、ルートユーザーから容易にバレるわけです。まぁルートユーザーなんでね。

## 中身にあるもの: fd

fd というディレクトリですが、正式名称は**ファイルディスクリプター**というそうです。  
中を開くとこれまた数字のディレクトリがたくさんあります。

## ファイルディスクリプター

よく、0 は標準入力、1 は標準出力、2 はエラー出力などと聞きます。 ディレクトリの数字はそれです。

しかし、実際のソフトウェアではその常識は通用しません。

```
l-wx------ 1 www-data www-data 64 Sep 23 17:20 2 -> /var/log/nginx/error.log
l-wx------ 1 www-data www-data 64 Sep 23 17:20 3 -> /var/log/nginx/error.log
l-wx------ 1 www-data www-data 64 Sep 23 17:20 4 -> /var/log/nginx/access.log
```

nginx だと、2 が error.log へ、4 は access.log へと、自由に設定できます。

## docker のロガー

docker のロガーは、pid1 の fd1 をログとして残すそうです。

# まとめ

コンテナ内で

```
echo hello > /proc/1/fd/1
```

とすれば、docker のログに`hello`という文字列が残る
