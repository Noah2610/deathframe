# `deathframe`
A __rust__ game development framework using the [`amethyst`][amethyst] game engine.  
It's primarily written for my personal game projects,  
I don't think it would be very useful to others.

---

__Note:__ I'm currently working on a major rewrite on the [`develop`][develop] branch.  
I would not recommend using the `master` branch or the latest release on [`crates.io`][crates.io] <small>(it's pretty bad)</small>.

## Usage
For the latest release from [`crates.io`][crates.io], put this in your `Cargo.toml` file ...
```toml
[dependencies]
deathframe = { version = "0.5.1", features = ["vulkan"] }
# Features must include one of:
#     "vulkan", "metal", "empty"
# Optional features:
#     "nightly"
```

For the development version on the [`develop`][develop] branch (major rewrite) ...
```toml
[dependencies.deathframe]
version = "*"
git = "https://github.com/Noah2610/deathframe"
branch = "develop"
features = ["vulkan"]
# Optional features:
#     "nightly", "physics", "serde"
```

## Features
I can't be bothered to write out a full list of features for the latest release,  
as almost everything has been rewritten or has major API changes  
in the [`develop`][develop] branch, which is the main version going forward.

For a _very rough_ list of features, see this [issue comment][issue-features].

## License
Distributed under the terms of the [MIT license][license].

[license]:        ./LICENSE
[develop]:        https://github.com/Noah2610/deathframe/tree/develop
[issue-features]: https://github.com/Noah2610/deathframe/issues/1#issuecomment-510974097
[crates.io]:      https://crates.io/crates/deathframe
[amethyst]:       https://github.com/amethyst/amethyst
