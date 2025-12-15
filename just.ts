#!/usr/bin/env bun
import { $ } from "bun";
import { cac } from "cac";

// If you need env types similar to EnumType in cliffy, you can handle validation manually
const VALID_ENVS = ["linux", "windows", "macos"];

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
    $`cargo install wasm-bindgen-cli --version 0.2.100`,
    $`cargo install wasm-opt`,
  ]);
}

async function buildWasm() {
  await $`cargo build --release --target wasm32-unknown-unknown`;
}

async function prepareWasmPackage(env: Env = { binary: "dinosaur-game" }) {
  // Gen JS
  await $`wasm-bindgen --out-name ${env.binary} --out-dir wasm --target web target/wasm32-unknown-unknown/release/${env.binary}.wasm`;
  // Optimize Wasm
  await $`wasm-opt -O wasm/${env.binary}_bg.wasm -o ${env.binary}.wasm`;
  // Compress Wasm using brotli
  await $`brotli wasm/${env.binary}_bg.wasm -o web/${env.binary}_bg.wasm`;
  await $`mv wasm/${env.binary}.js web/`;
  // Copy assets
  await $`cp -r crates/game/assets web/`;
  // Check assets folder exists under web
  if ((await $`test -d web/assets`.nothrow()).exitCode !== 0) {
    throw new Error("Assets folder not copied");
  }
}

async function buildRelease() {
  await $`cargo b --release`;
}

const cli = cac("just");

cli
  .option("--env <level>", "Environment to build", {
    default: "linux",
  })
  .command("", "Script for the dinosaur game")
  .action(async (options) => {
    if (options.env && !VALID_ENVS.includes(options.env)) {
        console.error(`Invalid env: ${options.env}. Must be one of: ${VALID_ENVS.join(", ")}`);
        process.exit(1);
    }
    await buildRelease();
  });

cli.command("install-linux-deps", "Install dependencies")
  .action(async () => {
    await installLinuxDeps();
  });

cli.command("install-wasm-deps", "Install wasm dependencies")
  .action(async () => {
    await installWasmDeps();
  });

cli.command("build-wasm", "Build wasm")
  .action(async () => {
    await buildWasm();
  });

cli.command("prepare-wasm-package", "Prepare wasm package")
  .action(async () => {
    await prepareWasmPackage();
  });

cli.command("web", "Web build")
  .action(async () => {
    await installWasmDeps();
    await buildWasm();
    await prepareWasmPackage();
  });

cli.help();
cli.version("0.1.0");

cli.parse();
