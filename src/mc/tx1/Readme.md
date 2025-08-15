# テクスチャ編 part1

> [!NOTE]  
> minecraft 本体のパス(multimc 系の場合)  
> `./libraries/com/mojang/minecraft/<version>/minecraft-<version>-client.jar`

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

`assets/minecraft/models/stone.json`

```json
{
  "parent": "minecraft:block/cube_all",
  "textures": {
    "all": "minecraft:block/stone"
  }
}
```

`assets/minecraft/models/cube_all.json`

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
