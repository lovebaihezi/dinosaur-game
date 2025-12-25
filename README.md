[![Build Status](https://img.shields.io/github/actions/workflow/status/lovebaihezi/dinosaur-game/build.yml?branch=main&label=build)](https://github.com/lovebaihezi/dinosaur-game/actions/workflows/build.yml)

# Dinosaur Game

A Rust implementation of the classic Chrome Dinosaur Game. This project is built with the Bevy game engine and runs on native platforms (Windows, macOS, Linux) and the web (via WebAssembly).

## How to Play

You can play the web version of the game [here](https://dino.lqxclqxc.com).

## Downloads

You can download the latest native versions of the game from the [releases page](https://github.com/lovebaihezi/dinosaur-game/releases).

## Release Process

This project uses an automated release system:

- **Stable Releases**: Created automatically when the version in `crates/game/Cargo.toml` is updated and pushed to the main branch. The workflow creates a version tag (e.g., `0.1.0`) which triggers builds for all platforms (Linux, Windows, macOS x86_64, macOS ARM64).

- **Nightly Releases**: Created automatically on weekdays (Monday-Friday) via scheduled workflow. Tagged with the format `nightly-YYYY-MM-DD`.

To create a new stable release:
1. Update the version number in `crates/game/Cargo.toml`
2. Commit and push to the main branch
3. The automated workflow will create a tag and trigger the build process
4. Binaries will be available on the releases page once builds complete
