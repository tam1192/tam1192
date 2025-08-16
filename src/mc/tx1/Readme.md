# テクスチャ編 part1

テクスチャ以前に画像編集能力高くないから

> [!WARNING]  
> この記事に**テクスチャ追加方法、変更方法について、直接の方法で**記載されていません。  
> (間接的に書いてますが)
>
> 純粋にテクスチャ追加するなら、別記事か、日記さんの次回作にご期待ください。

# 本日の一言

<iframe width="312" height="176" src="https://ext.nicovideo.jp/thumb/sm45297506" scrolling="no" style="border:solid 1px #ccc;" frameborder="0"><a href="https://www.nicovideo.jp/watch/sm45297506">【バトルボイスロイド】襲撃者を返り討ちにするつくよみちゃん【Aviutl2】</a></iframe>

Aviutl2 でましたね。 ~~wine 経由で入れようとして失敗しました。~~  
Aviutl 好きだったので、Aviutl2 もどうにかして使えるようにしたい。(wine で無理やりやるか、win 機買うか)

# テクスチャ編集について

リソースパックとテクスチャパックと言う単語が入り混じってて、昔はテクスチャパックだったらしい。  
**この記事ではリソースパックに統一します。**

ブロックの見た目を画像ファイルとして保存している。  
**一般的な 3D ゲームと異なり、一面分あればブロックテクスチャとして十分使える**。

形式は`.png`。 立体を表現する、モデルは`.json`で書かれている。

**だいぶ単純な仕組みで動作してる。**

## 画像ファイルのサイズについて

標準は**16x16(pixel、以下単位は pixel)**

**重くなるのを覚悟の上**、32x32、64x64...1024x1024 サイズと、高品質テクスチャにすることもできる。

## 編集ソフトについて

- [gimp](https://www.gimp.org/downloads/)  
  でええやろ。画像編集は。（適当）
- jar ファイルを解凍できるやつ(任意)
  7zip とか。 最悪`.jar`を`.zip`にすればいい。

# そもそも本体のテクスチャってどこにある？

~~バニラランチャー使ってないので忘れた~~  
[client.jar](https://ja.minecraft.wiki/w/Client.jar)参考

> [!NOTE]  
> minecraft 本体のパス(multimc 系の場合)  
> `<multimcのパス>/libraries/com/mojang/minecraft/<version>/minecraft-<version>-client.jar`

jar ファイルを展開すると、中身は大量の`.class`ファイルで埋め尽くされている。  
**ファイルブラウザのグループ化、シェルなら`cd`と tab 連打で assets ディレクトリを探そう**

<details><summary>assets 以降のパスについて</summary>

1.21.3 の場合(tree コマンドの結果)

`assets/minecraft/`以降で表示

```
.
├── atlases
├── blockstates
├── font
│   └── include
├── lang
├── models
│   ├── block
│   ├── equipment
│   └── item
├── particles
├── post_effect
├── shaders
│   ├── core
│   ├── include
│   └── post
├── texts
└── textures
    ├── block
    ├── colormap
    ├── effect
    ├── entity
    │   ├── allay
    │   ├── armorstand
    │   ├── axolotl
    │   ├── banner
    │   ├── bear
    │   ├── bed
    │   ├── bee
    │   ├── bell
    │   ├── boat
    │   ├── breeze
    │   ├── camel
    │   ├── cat
    │   ├── chest
    │   ├── chest_boat
    │   ├── conduit
    │   ├── cow
    │   ├── creaking
    │   ├── creeper
    │   ├── decorated_pot
    │   ├── end_crystal
    │   ├── enderdragon
    │   ├── enderman
    │   ├── equipment
    │   │   ├── horse_body
    │   │   ├── humanoid
    │   │   ├── humanoid_leggings
    │   │   ├── llama_body
    │   │   ├── wings
    │   │   └── wolf_body
    │   ├── fish
    │   ├── fox
    │   ├── frog
    │   ├── ghast
    │   ├── goat
    │   ├── hoglin
    │   ├── horse
    │   ├── illager
    │   ├── iron_golem
    │   ├── llama
    │   ├── panda
    │   ├── parrot
    │   ├── pig
    │   ├── piglin
    │   ├── player
    │   │   ├── slim
    │   │   └── wide
    │   ├── projectiles
    │   ├── rabbit
    │   ├── sheep
    │   ├── shield
    │   ├── shulker
    │   ├── signs
    │   │   └── hanging
    │   ├── skeleton
    │   ├── slime
    │   ├── sniffer
    │   ├── spider
    │   ├── squid
    │   ├── strider
    │   ├── tadpole
    │   ├── turtle
    │   ├── villager
    │   │   ├── profession
    │   │   ├── profession_level
    │   │   └── type
    │   ├── warden
    │   ├── wither
    │   ├── wolf
    │   ├── zombie
    │   └── zombie_villager
    │       ├── profession
    │       ├── profession_level
    │       └── type
    ├── environment
    ├── font
    ├── gui
    │   ├── advancements
    │   │   └── backgrounds
    │   ├── container
    │   │   └── creative_inventory
    │   ├── hanging_signs
    │   ├── presets
    │   ├── realms
    │   ├── sprites
    │   │   ├── advancements
    │   │   ├── boss_bar
    │   │   ├── container
    │   │   │   ├── anvil
    │   │   │   ├── beacon
    │   │   │   ├── blast_furnace
    │   │   │   ├── brewing_stand
    │   │   │   ├── bundle
    │   │   │   ├── cartography_table
    │   │   │   ├── crafter
    │   │   │   ├── creative_inventory
    │   │   │   ├── enchanting_table
    │   │   │   ├── furnace
    │   │   │   ├── grindstone
    │   │   │   ├── horse
    │   │   │   ├── inventory
    │   │   │   ├── loom
    │   │   │   ├── smithing
    │   │   │   ├── smoker
    │   │   │   ├── stonecutter
    │   │   │   └── villager
    │   │   ├── gamemode_switcher
    │   │   ├── hud
    │   │   │   └── heart
    │   │   ├── icon
    │   │   ├── notification
    │   │   ├── pending_invite
    │   │   ├── player_list
    │   │   ├── popup
    │   │   ├── realm_status
    │   │   ├── recipe_book
    │   │   ├── server_list
    │   │   ├── social_interactions
    │   │   ├── spectator
    │   │   ├── statistics
    │   │   ├── toast
    │   │   ├── tooltip
    │   │   ├── transferable_list
    │   │   ├── widget
    │   │   └── world_list
    │   └── title
    │       └── background
    ├── item
    ├── map
    │   └── decorations
    ├── misc
    ├── mob_effect
    ├── painting
    ├── particle
    └── trims
        ├── color_palettes
        ├── entity
        │   ├── humanoid
        │   └── humanoid_leggings
        └── items
```

</details>

## 名前空間について

`assets/<名前空間>`となってる。  
mod が入れば、ここが変わる。

> [!NOTE]  
> mod のテクスチャを変えたい時（かなりレアなケースだが）は、この名前空間の部分を ModID に置き換えれば Ok。  
> 詳しくは[fabric の記事参考](../fabric/1/)

## 音楽がない

この中に見つからないものは、著作権などの関係上(?)別のファイルに設置されている模様。

## models を見てみる

block なら、`models/block`である。 アイテムなら、`models/item`である。

> [!TIP]  
> アイテムもモデルあります  
> 平面モデルというのがありまして

`assets/minecraft/models/block/stone.json`

```json
{
  "parent": "minecraft:block/cube_all",
  "textures": {
    "all": "minecraft:block/stone"
  }
}
```

王道の石モデル。 モデルと言いながら**ものすごくシンプルである。**  
`parent`(直訳は「親」)にも model が指定されている。 これを探してみよう。

`assets/minecraft/models/block/cube_all.json`

```json
{
  "parent": "block/cube",
  "textures": {
    "particle": "#all",
    "down": "#all",
    "up": "#all",
    "north": "#all",
    "east": "#all",
    "south": "#all",
    "west": "#all"
  }
}
```

何とこちらもシンプルだった。

`assets/minecraft/models/block/cube_all.json`

```json
{
  "parent": "block/block",
  "elements": [
    {
      "from": [0, 0, 0],
      "to": [16, 16, 16],
      "faces": {
        "down": { "texture": "#down", "cullface": "down" },
        "up": { "texture": "#up", "cullface": "up" },
        "north": { "texture": "#north", "cullface": "north" },
        "south": { "texture": "#south", "cullface": "south" },
        "west": { "texture": "#west", "cullface": "west" },
        "east": { "texture": "#east", "cullface": "east" }
      }
    }
  ]
}
```

少し細かくなった。  
モデルは 1 ブロック 16 ドットあり、ドット単位で操作可能。  
テクスチャは 32x32 と細かくできるが、モデルは 16 より細かくできない

`assets/minecraft/models/block/cube_all.json`

```json
{
  "gui_light": "side",
  "display": {
    "gui": {
      "rotation": [30, 225, 0],
      "translation": [0, 0, 0],
      "scale": [0.625, 0.625, 0.625]
    },
    "ground": {
      "rotation": [0, 0, 0],
      "translation": [0, 3, 0],
      "scale": [0.25, 0.25, 0.25]
    },
    "fixed": {
      "rotation": [0, 0, 0],
      "translation": [0, 0, 0],
      "scale": [0.5, 0.5, 0.5]
    },
    "thirdperson_righthand": {
      "rotation": [75, 45, 0],
      "translation": [0, 2.5, 0],
      "scale": [0.375, 0.375, 0.375]
    },
    "firstperson_righthand": {
      "rotation": [0, 45, 0],
      "translation": [0, 0, 0],
      "scale": [0.4, 0.4, 0.4]
    },
    "firstperson_lefthand": {
      "rotation": [0, 225, 0],
      "translation": [0, 0, 0],
      "scale": [0.4, 0.4, 0.4]
    }
  }
}
```

ここでは手に持った時のモデルなどを指定している模様。

と、json ファイル内部で**変数**が使用できたり、親を指定して**継承**できたりと高機能である。  
一方でかなりシンプルに作られているのもわかる。

**石のように全方面同じテクスチャなら`cube_all`を使えば良い。**

## texture を見てみる

model が cube_all だったので、石のテクスチャは一面で済まされている。

## lang を見てみる

ここも json 形式。

```json
{
  "minecraft:stone": "石ブロック"
}
```

みたいな形で指定できるらしい。

# よくある落とし穴

- blockstates の指定がない  
  [fabric 記事](../fabric/2)見ていただいた方はわかるかと。
- `pack.mcmeta`がない  
  これがないとリソースパックとして認識されない。次回説明。
-

# まとめ

最初見た時は驚いた。 `.obj`ファイルとかあると思ってたので。
