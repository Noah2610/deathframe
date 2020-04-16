# `deathframe`
An opinionated __rust__ game development framework using the [`amethyst`][amethyst] game engine.  
It's like a _"goodie-bag"_ of features and boilerplate code for `amethyst` projects.  
`deathframe` abstracts many of `amethyst`'s features, the way I find it easier to use,  
with less boilerplate, setup, copy/paste. Very opinionated.

## Usage
For the latest release from [`crates.io`][crates.io], put this in your `Cargo.toml` file ...
```toml
[dependencies]
deathframe = { version = "0.5.1", features = ["vulkan"] }
# Features must include one of:
#     "vulkan", "metal", "empty"
```

For the development version on the [`develop`][develop] branch ...
```toml
[dependencies.deathframe]
version = "*"
git = "https://github.com/Noah2610/deathframe"
branch = "develop"
features = ["vulkan"]
```

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
[develop]:        https://github.com/Noah2610/deathframe/tree/develop
[issue-features]: https://github.com/Noah2610/deathframe/issues/1#issuecomment-510974097
[crates.io]:      https://crates.io/crates/deathframe
[amethyst]:       https://github.com/amethyst/amethyst
