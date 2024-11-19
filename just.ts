#!/usr/bin/env -S deno run --allow-all
import $ from "@david/dax";
import { Command } from "@cliffy/command";

interface Env {
  binary: string;
}

async function installWasmDeps() {
  await Promise.all([
    $`rustup component add rustc-codegen-cranelift-preview --toolchain nightly`,
    $`cargo install wasm-bindgen-cli`,
    $`cargo install wasm-opt`,
  ]);
}

async function buildWasm() {
  await $`cargo build --release --target wasm32-unknown-unknown`;
}

async function prepareWasmPackage(env: Env) {
  await $`wasm-bindgen --out-name ${env.binary} --out-dir wasm --target web target/wasm32-unknown-unknown/release/${env.binary}.wasm`;
  await $`wasm-opt -O wasm/${env.binary}_bg.wasm -o ${env.binary}.wasm`;
  await $`# Compress Wasm using brotli`;
  await $`brotli wasm/${env.binary}_bg.wasm -o web/${env.binary}_bg.wasm`;
  await $`mv wasm/${env.binary}.js web/`;
  await $`cp -r assets web/ || true # Try to copy, but ignore if it can't copy if source directory does not exist`;
}

async function buildRelease() {
  await $`cargo b --release`;
}

// TODO: Default Run Install Deps and build Release
// TODO: Migrate All workflow script to this file
await new Command()
  .name("just")
  .version("0.1.0")
  .description("Script for the dinosaur game")
  .parse(Deno.args);
