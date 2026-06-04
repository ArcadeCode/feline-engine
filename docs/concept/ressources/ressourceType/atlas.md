# `Ressource<Atlas>` ressource type
This ressource represent an optimized atlas of multiple `Ressource<Texture>`. Atlas are highly abstracted, no need to etablish size or anything.

## When Atlas are preferable
Atlas are usefull for any light texture highly mutliplied for example : Textures of each voxel in a game, each items texture, ... Atlas are less usefull for big textures such as the complete world plane texture.

## How to etablish Atlas
Atlas can be static or dynamic, it's the only action done by the developer. By default Atlas are static, them are signed like this `Ressource<Atlas<FORMAT, MODE>>`. Atlas texture **need to be the same format**, don't worry, the engine handle exporting a format to another, [see `Ressource<Texture>` documentation](./texture.md).

> [!TIPS]
> You may ask : What the point of dynamic atlas ? It's actually usefull only in game where new objects are generate dynamically during the play.

### Creating atlas from folder
The simple way and most "idiomatic" way to load an atlas is by a folder, simply have an `atlas/` folder or subsitute another name which is logical with what insind and run `Ressource<Atlas>.newFromFolder()` this by default will load `Ressource<Atlas<FORMAT, STATIC>>`