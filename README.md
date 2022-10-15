# Raycaster in Rust

![banner](https://i.imgur.com/Z3FJBH8.png)

This is a raycaster built with Rust and MacroQuad.

This raycaster has two modes. That being Normal and Debug mode.
The Debug mode is shown the above example.

Here is the Normal mode.
![normal](https://i.imgur.com/D0Y4P6p.png)

## Table of Contents

- [Raycaster in Rust](#raycaster-in-rust)
	- [Table of Contents](#table-of-contents)
	- [Install](#install)
	- [Usage](#usage)

## Install

I will assume you have rust and [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

```bash
git clone this-repo
```

After cloning the repo move into that folder

```bash
cd this-repo
```

## Usage

Now you can run cargo with either debug option or not.

Without debug option:

```bash
cargo run --release
```

With debug option:

```bash
cargo run --release debug
```

You can now move around with the arrow keys and see everything.
You also have the option in the code to edit the map variable inorder to edit the placement of walls.
Finally, you can edit the `GRID_SIZE` constant to get a different sized grid but make sure to switch the map variable or the code won't compile.

To compile the code simply run the command:

```bash
cargo build --release
```

Followed one of the two run commands above.

Small note: If editing the Readme, please conform to the [standard-readme](https://github.com/RichardLitt/standard-readme) specification.
