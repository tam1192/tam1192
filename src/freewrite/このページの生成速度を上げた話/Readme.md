# このページの生成速度を上げた話
だいぶ早くなりました。

# mdbookについて
いまこのページを作るのに使っているツール `mdbook`はrust製のソフトウェアで、名前の通り**マークダウン形式で本(資料)を書ける**のが特徴的です。  
加えて、GithubActionsを活用し、ページ生成、GithubPagesへの登録まで一括して行なっています。  
そのため、私は[このページを保管しているリポジトリ](https://github.com/tam1192/tam1192)にマークダウンファイルを配置し、PRしつつ`main`ブランチにマージすれば、ページが追加されます。  
割と便利。

# cargo installについて
rust製のソフトウェア、ライブラリは、[crates.io](https://crates.io)によって登録、管理されています。 **cargo**というツールを活用することで、プロジェクトにライブラリを追加したり、rust製のアプリケーションをPCにインストールできます。  
cargoは、aptなどのパッケージマネージャーと異なり、インストール時にビルドをするため、インストール時間がながくなってしまいがちです。

GithubActionsは、簡単に言えば一時的にlinux環境を借りることができる機能なのですが、処理が終わればその環境はリセットされます。　また、最低限の機能しか内臓されてない(とはいえdockerとかcargoとかは標準である)ので、mdbookをcargoでいちいちインストールする必要があるのです。

# 生成速度を上げるために
つまり、cargo installを回避すれば、高速化できるということです。
そこで、dockerの力を借りることにしました。  
-> [tam1192/MyMdbookContainer](https://github.com/tam1192/MyMdbookContainer) 

あらかじめ、cargo installを済ませたmdbook内臓のdockerイメージを作成しておき、[公開しておくことにした](https://github.com/tam1192/MyMdbookContainer/pkgs/container/mymdbookcontainer)のです。  
このコンテナイメージはmdbookと、このページに使うプラグインを追加して、イメージにしております。  

イメージの公開先は、[ghcr.io](https://ghcr.io)を活用してます。 githubが所有するコンテナリポジトリです。 github Actionsもgithubが所有する機能の一つなので、その繋がりで高速化が期待できそうですね！


# 結論
ページ生成が速くなりました。
