# openwrt は fw で 9 割だと感じた件

題名の通りです

# 注意

このページは技術的な情報の共有を目的として**作られておりません**。  
あくまで、１利用者として感じたことをそのまま書いております。(一般的には※個人的な感想ですってやつです！)

# 🎵 本日の一曲

<iframe width="312" height="176" src="https://ext.nicovideo.jp/thumb/sm45124783" scrolling="no" style="border:solid 1px #ccc;" frameborder="0"><a href="https://www.nicovideo.jp/watch/sm45124783">T氏の話を信じるな / 初音ミク・重音テト</a></iframe>

T 氏の話は信じてはいけない(戒め)

# なぜそう感じたか

ワイの openwrt 機、通信できないトラブルの大体が fw だった。

## 具体例

- ping が通らない => ICMP が許可されてなかった
- ra が降ってこない => ICMP が許可されてなかった

## device/interface/zone がややこしい

私があった経験だと、v4 と v6 でインターフェイス分けて、v6 は wan みたいなもんなので、zone を wan にして、  
うまく設定できてると思ったけどできてなかった。
(売る覚えで書いた。 そんなようなこともあった。)

# nftables について

iptables の次世代的なやつです。 最新の openwrt には iptables に変わって、nftables が fw を担当しています。
たいていこれ見ると解決することが多いです。

# やっぱ 6 割かもしれない

uci と仲良くするのにあと数年必要かもしれない。
