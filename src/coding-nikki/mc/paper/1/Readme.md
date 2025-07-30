# paper 編 part1 コマンド定義

基本中の基本でありながら早速引っ掛けがあったのでメモ

# 🎵 本日の一曲

<iframe width="312" height="176" src="https://ext.nicovideo.jp/thumb/sm44423841" scrolling="no" style="border:solid 1px #ccc;" frameborder="0"><a href="https://www.nicovideo.jp/watch/sm44423841">「ブランディングができない」feat.KafU</a></iframe>

pv がしぬほどかわいい。

# CommandExecutor vs TabExecutor

```mermaid
---
config:
  class:
    hideEmptyMembersBox: true
---
classDiagram
direction TB
    class CommandExecutor {
	    onCommand()
    }
    class TabExecutor {
	    onTabComplete()
    }

	<<Interface>> CommandExecutor
	<<Interface>> TabExecutor

    TabExecutor --|> CommandExecutor

```

```kt
import org.bukkit.command.Command
import org.bukkit.command.CommandSender
import org.bukkit.command.TabExecutor

class ExampleCommand : TabExecutor {
    override fun onTabComplete(
        p0: CommandSender,
        p1: Command,
        p2: String,
        p3: Array<out String>?
    ): List<String?>? {
        TODO("Not yet implemented")
    }

    override fun onCommand(
        p0: CommandSender,
        p1: Command,
        p2: String,
        p3: Array<out String>?
    ): Boolean {
        TODO("Not yet implemented")
    }
}
```

TabExecutor を実装すれば、**CommandExecutor と TabExecutor 両方を実装することになります**。  
TabExecutor には、onTabComplete というメソッドがあります。 これは**コマンド補完機能を提供**するものです。

**基本的には`TabExecutor`を使おう!**

## Tab 補完あれこれ

onCommand メソッドに関する情報はたくさんあると思うので、ここでは onTabComplete のメモです

基本的な使い方はこんな感じ

```kt
override fun onTabComplete(
    sender: CommandSender,
    command: Command,
    label: String,
    args: Array<out String>
): MutableList<String>? {
    return when {
        args[0] == "add" -> Material.entries.filter { it.isBlock }.map { it.name.lowercase() }.toMutableList()
        args.isEmpty() -> "add,del,list".split(",").toMutableList()
        else -> null
    }
}
```

kotlin の場合は MutableList で返すことになります。 とりあえず java では ArrayList、rust では mut Vec ですね。  
`when`が**式**なのを生かして、**when の結果をそのまま return に流してます。**

**kotlin を使うメリット 1 ですね。**

> [!NOTE]
> rust の使い勝手に近いならそれは**メリット**ですよね！(圧)

### onTabComplete が呼ばれるタイミング

実際に調べてみます。

```kt
package org.adw39.examplePlugin2

import org.adw39.examplePlugin2.commands.ExampleCommand
import org.bukkit.plugin.java.JavaPlugin

class ExamplePlugin2 : JavaPlugin() {

    override fun onEnable() {
        getCommand("example")?.setExecutor(ExampleCommand())
    }

    override fun onDisable() {
        // Plugin shutdown logic
    }
}
```

```
package org.adw39.examplePlugin2.commands

import org.bukkit.command.Command
import org.bukkit.command.CommandSender
import org.bukkit.command.TabExecutor

class ExampleCommand : TabExecutor {
    override fun onTabComplete(
        p0: CommandSender,
        p1: Command,
        p2: String,
        p3: Array<out String>?
    ): List<String?>? {
        p3?.forEach {
            println(it)
        }
        return null
    }

    override fun onCommand(
        p0: CommandSender,
        p1: Command,
        p2: String,
        p3: Array<out String>?
    ): Boolean {
        return true
    }

}
```

> [!TIP]
> array は配列なので、forEach で中身を分解して出力します。

> [!TIP]
> 内部に Logger オブジェクトが存在しますが、println も使用できます。(多分非推奨)

`resources/plugin.yml`

```yaml
name: MyPaperPlugin
version: 0.0.1
main: org.adw39.examplePlugin2.ExamplePlugin2
description: An example plugin
author: nikki
website: https://adw39.org
api-version: "1.21.0"
commands:
  example:
    description: テスト
    usage: "/example <arg>"
    permission: org.adw39.examplePlugin2.example
```

> [!NOTE]
> ファイルの位置は、package 文をみていただくのが早いかと思います。  
> 複雑になり始めたら tree を載せます。

> [!WARNING]  
> **plugin.yaml の編集を忘れずに!!**

### 実行結果

`onTabComplete`は、**キー入力を受ける度に呼び出される**ようです！ **tab を押されてから呼び出すわけではない模様**  
これは深刻な問題です。 もし`onTabComplete`のロジックを複雑にしてしまえば、**キー入力を受ける度にサーバーが重くなる可能性がある**ことを意味します。

> [!NOTE]
> と、書いたけど実際どーなんでしょうねぇ。

## ブロック名で補完させる

`worldedit`のようなプラグインを作りたくなった場合、ブロック名の補完　は必須です。
