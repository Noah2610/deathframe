# `deathframe`
An opinionated __rust__ game development framework using the [`amethyst`] game engine.  
It's like a _"goodie-bag"_ of features and boilerplate code for `amethyst` projects.  
`deathframe` abstracts many of `amethyst`'s features, the way I find it easier to use,  
with less boilerplate, setup, copy/paste. Very opinionated.

## Usage
For the latest release from [crates.io], put this in your `Cargo.toml` file ...
```toml
[dependencies]
deathframe = { version = "0.5.1", features = ["vulkan"] }
# Features must include one of:
#     "vulkan", "metal", "empty"
```

For the development version on the [`develop`] branch ...
```toml
[dependencies.deathframe]
version = "*"
git = "https://github.com/Noah2610/deathframe"
branch = "develop"
features = ["vulkan"]
```

## Features
`deathframe` is split-up into a couple `crates`.  
All sub-crates follow a similar structure.  
The root `deathframe` crate re-exports all of these,  
and provides some common `specs` / `amethyst` preludes ...

- `bundles`  
  Every sub-crate gets a `bundle` in the root crate.  
  This makes plugging-in a crate's systems easier.
- `components`, `systems`, `states` preludes  
  These simply re-export a bunch of `specs` and `amethyst` types,  
  which are needed to write _components_, _systems_, and _states_.  
  They also re-export _components_ and _systems_ from all enabled sub-crates.
- `resources`  
  Re-exports _resource_ types from sub-crates.  
  A resource is something that is usually _inserted_ into the `specs` world,  
  and that is usually used in _systems_.

### `deathframe_core`
The main crate. Holds common types and components, used across all sub-crates.  
Also holds miscellaneous data types and traits,  
that I find useful, but don't know where to put.  

A core feature is the _entity loading_ system.  
Using the `Loadable`, `Loaded`, and `Loader` components, together with  
the `EntityLoaderSystem` (or write your own), many systems through-out  
this crate will be enabled or disabled for entities, depending on their  
`Loaded` and `Loadable` components.  
Many systems will only run on entities that either have _both_ the  
`Loadable` and `Loaded` components or have _neither_ component.

The `CustomGameData` is a big one, which manages multiple _dispatchers_,  
which can be updated individually from within _states_.  
It's supposed to be used as a state's `GameData`.  
<sub>I really need to rename that. Also needs refactoring.</sub>

There's also the `InputManager`, which is an abstraction over  
`amethyst`'s `InputHandler`, which enables you to check for  
_keydown_, _keypress_, and _keyup_ events separately.  
<sub>It's not perfect. Like everything in `deathframe`.</sub>

`SpriteSheetHandles` is another note-worthy resource. I still use it,  
although I think you can definitely do without. Basically, it simplifies  
_loading_ sprite-sheets, and _getting_ handles to them.  
<sub>This was useful when I was learning `amethyst`, and the asset loader was ~~weird~~ new to me.</sub>

### `deathframe_animation`
Frame-by-frame animations for entities. Cycle sprites with delays.  
Animate entities that have a `SpriteRender` with the `Animation` component.  
An entity can hold and switch between multiple animations with  
the `AnimationsContainer` component.

### `deathframe_audio`
Adds abstraction over playing and controlling audio with `amethyst`.  
There are two main resources in this one:  
- `Sounds` (SFX)  
  For loading and playing _sound-effects_.  
  Sounds that you simply play and forget.  
  You can attach the `SoundPlayer` component to entities,  
  to easily allow entities to queue sound-effects.
- `Songs` (BGM)  
  For loading, playing, and controlling playback of background music.  
  The major differences to `Sounds`, is that this can or should only play  
  a single song at once, and that the playback has state,  
  which you can manipulate (play, pause, change song and playback behavior).

### `deathframe_physics`
Adds some common physics-related components, such as `Velocity` and `Gravity`.  

It also adds my implementation of collision detection, for general collision checking,  
and for moving solid entities, without them intersecting.  
Make entities _collidable_ and _solid_ with combinations of the components  
`Collidable`, `Collider`, `Solid`; all collision checking entities  
will also always need a `Hitbox` component.  

The actual collision detection "algorithm" is very naive and simple.  
For now, it only works with rectangles. Surprisingly, it seems efficient  
enough for my game projects <sub>(as long as entities are properly loaded and unloaded)</sub>.  

To make checking for collision events easier, there's the `query` module,  
with which you can build and run _collision queries_ on `Collider` entities.

## Cargo feature flags
| Name | Description | Default? |
|:---- |:----------- |:--------:|
| `vulkan` | For `amethyst`'s  _vulkan_ backend. | |
| `metal` | For `amethyst`'s  _metal_ backend. | |
| `empty` | For `amethyst`'s  _empty_ backend. | |
| `animation` | Use `deathframe_animation` crate.<br />For sprite frame-by-frame animations. | __Yes__ |
| `audio` | Use `deathframe_audio` crate.<br />Simple BGM (`Songs`) and SFX (`Sounds`) systems. | __Yes__ |
| `physics` | Use `deathframe_physics` crate.<br />Adds _velocity_, _gravity_, _friction_, other stuff, and my implementation of _collision detection_ and _solid collision_ for moving entities.<br />You should consider doing your own thing with a proper collision engine. | __Yes__ |
| `debug` | To enable some specific debugging code.<br />Unnecessary, but I always turn this on while developing. | |

## License
Distributed under the terms of the [MIT license][license].

[license]:        ./LICENSE
[`develop`]:      https://github.com/Noah2610/deathframe/tree/develop
[crates.io]:      https://crates.io/crates/deathframe
[`amethyst`]:     https://github.com/amethyst/amethyst
