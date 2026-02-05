<div align="center">

![B-Engine.Rustforged](gh_res/BE-LOGO.png)

![GitHub License](https://img.shields.io/github/license/VaccuumDev/B-Engine.Rustforged?style=for-the-badge&color=seagreen)

*Performant procedural worlds game engine.*

</div>

---

### Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Getting Started](#getting-started)
- [Contributing](#contributing)
- [License](#license)
- [Links](#links)

---

### Features

- **Fully Reimplemented**: A complete rewrite of the original B-Engine with enhanced performance.
- **Optimized Physics**: Improved physics engine that leverages Rust's efficiency.
- **Designed for Open-World Games**: Tailored for developing expansive, immersive worlds.
- **Procedural Generation Support**: Facilitate the creation of dynamic, ever-changing environments.
- **Modular Design**: Easily add or remove modules using Bevy ECS + Plugins system.

### Installation

To get started with B-Engine.Rustforged, clone the repository and build the project using the following commands:

```bash
git clone https://github.com/yourusername/B-Engine.Rustforged.git
cd B-Engine.Rustforged
cargo build
```

Ensure you have [Rust](https://www.rust-lang.org/) and [Cargo](https://doc.rust-lang.org/cargo/) installed on your machine.

#### 0R

Clone the repository and directly connect engine files to your project

### Usage

Include B-Engine and Bevy to your Cargo crate and write
```rust
use b_engine::*;
use bevy::prelude::*;

fn main()
{
  App.new().add_plugins(BEngine).run();
}
```
And window will appear on your screen.

---

### Contributing

Contributions to B-Engine.Rustforged are welcome! Here are some ways to contribute:

- Reporting bugs or issues
- Suggesting features or improvements
- Submitting pull requests with outstanding code

Please ensure any contributions align with [contribution guidelines](CONTRIBUTING.md).

---

### License

B-Engine.Rustforged is licensed under the GNU General Public License v3.0.
See [license](LICENSE) for more info.
![GNU GPL](https://upload.wikimedia.org/wikipedia/commons/thumb/9/93/GPLv3_Logo.svg/960px-GPLv3_Logo.svg.png?20210605024701)

---

### Links

- [Bevy Official Website](https://bevy.org/)
- [Bevy GitHub Repository](https://github.com/bevyengine/bevy)
- [Avian3d Github Repository](https://github.com/avianphysics/avian)
- [480 Design Solar Theme Icons](https://github.com/480-Design/Solar-icon-set)

