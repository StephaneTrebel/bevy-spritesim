# About

This folder has all the assets used by the application

## Sprites

Sprites are located in the [`./sprites/`](./sprites) directory.

### Naming Convention

Files are named : `sprite_<TYPE>_<NAME>_<VARIANT>_<ANIMATION_INDEX>.png`

Where :

- `<TYPE>` is the sprite type (`terrain`, etc.)
- `<NAME>` is the sprite common name (`desert`, `plain`, etc.). Sprite should
  be in a folder having their name, for convenience
- `<VARIANT>` is the sprite "variant". This might be confusing at first, but
  sprites often have variants to account for their shape relative to other
  tiles. You may have encounter the term "tile set". This is the same thing
- `<ANIMATION_INDEX>` is sprite animation index (for instance it will be
  between `0` and `3`, if the sprite has a 4-steps animation)

### Sprites loading process

Sprites are all loaded at once from the `assets` folder at startup.
