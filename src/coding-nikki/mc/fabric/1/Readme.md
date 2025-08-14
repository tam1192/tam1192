# fabric 編 part1 ブロック作ってみた

生まれて 23 年、まさかのブロックすら作ったことないってマ？

# 🎥 今日の RT

<iframe width="312" height="176" src="https://ext.nicovideo.jp/thumb/sm43425638" scrolling="no" style="border:solid 1px #ccc;" frameborder="0"><a href="https://www.nicovideo.jp/watch/sm43425638">きりたんと弱小ワンオペスーパー経営【Supermarket Simulator ①】</a></iframe>

どるるるるる

# 我サーバーにて

我、個人サーバーやってるナリ  
気づいたら使わないブロックだと行って無理やりスポンジが畳に変貌してたナリ  
テクスチャもらってきたので、fabric でブロックにしてしまうナリ

> [!WARNING]
> ※なお、当サイトにあるテクスチャ画像の無断使用は NG でお願いします。  
>  一応。

# fabric の wiki を参考にアイテムを追加してみる。

```kt
package org.adw39.myfabricmods

import net.fabricmc.api.ModInitializer
import net.minecraft.item.Item
import net.minecraft.registry.Registries
import net.minecraft.registry.Registry
import net.minecraft.util.Identifier

class Myfabricmods : ModInitializer {

    companion object {
        lateinit var CUSTOM_ITEM: Item
    }

    override fun onInitialize() {
        CUSTOM_ITEM = Item(Item.Settings())
        Registry.register(Registries.ITEM, Identifier.of("myfabricmods", "custom"), CUSTOM_ITEM)
    }
}
```

> [!TIP]  
> **Identifier の属性が Private とエラー！**
> copilot 曰く**1.21 以降は`Identifier.of()`でやるんだってさ。** ~~しるかそんなもん~

> [!TIP]  
> kotlin では**java と初期化のタイミングが異なり、**lateinit を使う方が賢い場合があります。

なんて TIP を書くのに 30 分。 これくらいならすぐ起動するやろとやってみらー

```
Caused by: java.lang.NullPointerException: Item id not set
```

あまりにも有名で、かつシンプルなエラーが帰ってきた。 原因がわからん。  
**kotlin のせいか？**

# ドキュメントがあった

[Creating Your First Item - fabric documentation](https://docs.fabricmc.net/develop/items/first-item)

ドキュメントを見つけたらまるでコードが違くてびっくり。バージョン違いでこんくらい変わるの？  
急いで kotlin で書き直す。

```kt
package org.adw39.myfabricmods

import net.fabricmc.api.ModInitializer
import net.minecraft.item.Item
import net.minecraft.registry.Registries
import net.minecraft.registry.Registry
import net.minecraft.registry.RegistryKey
import net.minecraft.registry.RegistryKeys
import net.minecraft.util.Identifier

class Myfabricmods : ModInitializer {

    companion object {
        lateinit var item: Item
    }

    override fun onInitialize() {
        val itemKey = RegistryKey.of<Item>(RegistryKeys.ITEM, Identifier.of("myfabricmods", "myfabricmods"))
        var settings = Item.Settings()
        settings.registryKey(itemKey)

        item = Item(settings)

        Registry.register(Registries.ITEM, itemKey, item);

    }
}
```

とりあえず 1 つのファイルに無理やり納めてみるとこうなった。
