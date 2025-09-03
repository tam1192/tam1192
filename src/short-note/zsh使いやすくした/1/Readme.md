# zsh使いやすくした 1回目

コマンド入力めんどい

# 本日の一言

[ナマステト / 重音テトSV](https://www.nicovideo.jp/watch/sm44594927)という曲を見つけてしまい、思わずクリックしてしまった。

# aliasで使いやすくする
zshって結構カスタマイズしたいことがあって、.zshrcを編集します。  
`vi ~/.zshrc` `source ~/.zshrc` 別に普通に入力すればいいだけなんですが、めんどかったので**前者を`vrc`、後者を`src`に短縮します**。

(~/.zshrcの先頭あたり)
```sh
# EDITOR環境変数を設定してます
export EDITOR="/usr/bin/vi"
alias vrc="$EDITOR ~/.zshrc"
alias src="source ~/.zshrc"
```

`source ~/.zshrc`で完成。　次回以降はvrcとsrcが使用可能になります。

# 頻繁に使うディレクトリのショートカットを作る
aliasでやってもいいですが、zshの機能でもっと簡単にできました。

`hash -d <ショートカット名>=<ディレクトリ>`というのがあり、これは`~<ショートカット名`となります。  
ディレクトリで使用可能となり、cdコマンドの入力が楽になります。

`hash -d t=/tmp` は `cd ~t`となります。`~t`だけでも飛べます。  
(zshでは、cdコマンドは省略ができる)  


```sh
vrc
```
```
# EDITOR環境変数を設定してます
export EDITOR="/usr/bin/vi"
alias vrc="$EDITOR ~/.zshrc"
alias src="source ~/.zshrc"
# hashを追加
hash -d t=/tmp
```

`src`コマンドで適用すればok

# まとめ
これでキー入力スピード勝負を持ち込まれた時に自信持って負けられます。  
...🤔

