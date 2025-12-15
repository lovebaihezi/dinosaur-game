#!/usr/bin/env -S deno run --allow-all
import $ from "@david/dax";
import { Command, EnumType } from "@cliffy/command";

const envEnum = new EnumType(["linux", "windows", "macos", "macos"]);

interface Env {
  binary: string;
}

async function installLinuxDeps() {
  // TODO: check if Ubuntu
  await $`sudo apt-get update`;
  await $`sudo apt-get install --no-install-recommends pkg-config libx11-dev libasound2-dev libudev-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev clang mold libwayland-dev libxkbcommon-dev`;
}

async function installWasmDeps() {
  await Promise.all([
    $`rustup component add rustc-codegen-cranelift-preview --toolchain nightly`,
    $`cargo install wasm-pack`,
  ]);
}

async function buildWasm(env: Env = { binary: "dinosaur-game" }) {
  // We use wasm-pack to build the project
  // We need to set the RUSTFLAGS to use the wasm_js backend for getrandom
  await $`RUSTFLAGS='--cfg getrandom_backend="wasm_js"' wasm-pack build crates/game --target web --release --out-dir ../../wasm --out-name ${env.binary}`;
}

async function prepareWasmPackage(env: Env = { binary: "dinosaur-game" }) {
  // Compress Wasm using brotli
  // wasm-pack output is in wasm/ folder
  await $`brotli wasm/${env.binary}_bg.wasm -o web/${env.binary}_bg.wasm`;
  await $`mv wasm/${env.binary}.js web/`;
  // Copy assets
  await $`cp -r crates/game/assets web/`;
  // Check assets folder exists under web
  if (!(await $`test -d web/assets`)) {
    throw new Error("Assets folder not copied");
  }
}

async function buildRelease() {
  await $`cargo b --release`;
}

// TODO: Migrate All workflow script to this file
await new Command()
  .name("just")
  .description("Command used to build whole project")
  .version("0.1.0")
  .type("env", envEnum)
  .globalOption("--env <level:env>", "Environment to build", {
    default: "linux",
  })
  .description("Script for the dinosaur game").action(async () => {
    await buildRelease();
  })
  .command("install-linux-deps", "Install dependencies").action(async () => {
    await installLinuxDeps();
  })
  .command("install-wasm-deps", "Install wasm dependencies").action(
    async () => {
      await installWasmDeps();
    },
  )
  .command("build-wasm", "Build wasm").action(async () => {
    await buildWasm();
  })
  .command("prepare-wasm-package", "Prepare wasm package").action(async () => {
    await prepareWasmPackage();
  })
  .command("web", "Web build").action(async () => {
    await installWasmDeps();
    await buildWasm();
    await prepareWasmPackage();
  })
  .parse(Deno.args);
