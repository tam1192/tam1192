# ceph-monってなに？
ceph-mon
私もこの記事を書いている時点ではあまりよくわかってないんですが、
わかる範囲で例えるなら「図書館司書」

ところで、私の好きなゲームの話なのですが、図書館に引きこもりなんですが、本が、とても大好きな子がいるんです。
コミュ障なところがほんと好き。
~~ちなみに、コスト削減してくれるので結構有能です。~~

# cephの概要
Cephは、複数のサーバーが連携して動作するクラスタシステムで、ストレージを共有・管理できるシステムのこと。
つまり、各サーバーが持つストレージを、一つのストレージとしてみることができます。

## オブジェクト
Cephはオブジェクトストレージ。
データ一つ一つがオブジェクトと呼ばれています。

## レプリケーション
![](./figure/fig01.svg)

冗長性を保つため、各サーバーストレージに複製を保存する機能です。
kubernetesでも似た様な技術が見られます。

## OSD
Cephでは、ストレージ単体のことをOSD（Object Storage Device）と言います。
OSDは**サーバー自体ではなく、サーバー内のストレージデバイスを指す**ので注意。

## Ceph Monitor(ceph-mon)
Ceph Monitor (以下mon) は**cephシステムの入口的存在**。
役割は以下の通り
- 監視機能：OSDの状態を常に監視
- オブジェクト管理：オブジェクトがどのOSDに記録されているかを記録

ここが消えてしまえば、ストレージにアクセスすることはできません。
そのためmonは冗長化されています。

![](./figure/fig03.svg)

## Map
地図です。データの配置や構造を記録されてます。

- ObjectMap: オブジェクトの位置を記録するMap
- MonMap: 冗長化されているmon同士が、どの様に働いているかを記録するMap

全てのmonは、このMapを同期しあい、ユニークになる様にしています。

### 図書館で例えると
- オブジェクトは本。
- OSDは書棚。　
- monは司書。

書棚には本が置かれています。
そして（優秀な）司書は、本がどこに置かれているのか、
はっきりわかるわけです。
また、司書は仕事として、本の質を確認して回っています。

## mapの場合は
Mapは司書が持っているツールです。
ObjectMapは本がどの書棚にあるかを記録
MonMapは司書同志の連絡帳ですね。

Mapの同期が取れてないと、別の司書が本を移した時、
「そこにあると思われていた本がない」といったトラブルにつながるわけです。

![](./figure/fig4.svg)

# MonMapとIP
monmapは図書館で例えている時、連絡帳と書きました。
この連絡帳にはIPが書かれています。

```
epoch 16
fsid 12345678-abcd-1234-zyxw-9876-abcdefghijklm
last_changed 2025-01-20T12:40:16.621338+0000
created 2024-10-27T05:57:33.339382+0000
min_mon_release 18 (reef)
election_strategy: 1
0: [v2:192.168.1.10:3300/0,v1:192.168.1.10:6789/0] mon.node1
1: [v2:192.168.1.20:3300/0,v1:192.168.1.20:6789/0] mon.node2
dumped monmap epoch 16
```
これがmonmap。　至ってシンプルです。

```
epoch <monmapのバージョン>
fsid <cephシステムに使われる固有の識別子, id>
last_changed <最終更新日>
created <作成日>
min_mon_release 18 (reef) <おそらくmonの最低要件バージョン>
election_strategy: 1 <よくわからん...>
0: <monのipと名前>
...
dumped monmap epoch 16
```

monmapは、内部でバージョン管理されていて、
やろうと思えばロールバックができる構造になってます。
(...やり方知りませんが...)

## monmapの取得方法
たくさんあります
### `ceph`コマンドが生きてる場合
cephコマンドが使える場合は、
```
ceph mon dump
```
で取得可能です。

### `ceph`コマンドが生きてない場合
`ceph-mon`コマンドを使います。
そもそもこのコマンドは、monそのものなので
systemctlで立ち上げる場合、内部でコマンドが用いられます。

```
ceph-mon -i <monの名前> --extract-monmap <出力したい場所, ex: ~/monmap>
```

次に、`monmaptool`を使います。
```
monmaptool --print <出力した場所, ex: ~/monmap>
```

### その他
ほかにも `ceph-monstore-tool`を用いる方法もあります。
```
ceph-monstore-tool <mon storeの場所> get monmap -r
```

## monstore
monstoreとは、monが持つ小さなデータベースです。
MonMapとObjectMapが保管されています。

### 場所
- microceph: `/var/snap/microceph/common/data/mon/<monの名前>`
- ceph: `/var/lib/ceph/mon/<monの名前>`

monの名前を忘れた時、ここをみるとわかるかもしれません。

## monmapを手動書き換えする
`ceph-mon`コマンドで取得したmonmapは、`monmaptool`を用いることで
書き換えることが可能です。
monmap関係でトラブルが発生した時はこの方法を用いることで解決できると思います。

また、書き換えたmonmapを再びmonstoreに戻す時は
```
ceph-mon -i <monの名前> --inject-monmap <出力した場所, ex: ~/monmap>
```
とすることで対応可能です。 なお、injectしたmonmapのバージョンは、自動的に引き上げられます。

# コマンドリファレンス
- [ceph-mon](https://docs.ceph.com/en/latest/man/8/ceph-mon/)
- [monmaptool](https://docs.ceph.com/en/latest/man/8/monmaptool/)
- [ceph-monstore-tool](https://docs.ceph.com/en/latest/man/8/ceph-monstore-tool)

# 参考
- [ceph: troubleshooting-mon](https://docs.ceph.com/en/latest/rados/troubleshooting/troubleshooting-mon/)
- [redhat: Ceph Monitor のトラブルシューティング](https://docs.redhat.com/ja/documentation/red_hat_ceph_storage/8/html/troubleshooting_guide/troubleshooting-ceph-monitors)


# で、結局お前何やらかしたの？
```
ceph mon set-addrs <name> <addrs>  
```

こういうコマンドがあります。　このコマンドはアドレスを変えるコマンドです。
うちでmonが動いているサーバーには複数のipがあります。

二個目のipに対応させるため、いい方法を探してたらこのコマンドを見つけたわけです。

もともと3ノードあったのですが、3ノード目で1ノードと同じipを設定したら、
1,3ノードともに動作しなくなりました。

結局、monmaptoolで3ノード目を消して解決しました。

...二週間。 学生でよかったと思った瞬間でした。