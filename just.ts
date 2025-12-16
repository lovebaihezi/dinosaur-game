#!/usr/bin/env bun
import { $ } from "bun";
import { cac } from "cac";

// If you need env types similar to EnumType in cliffy, you can handle validation manually
const VALID_ENVS = ["linux", "windows", "macos"];

interface Env {
  binary: string;
}

const DEFAULT_BINARY = "dinosaur-game";

async function installLinuxDeps() {
  await $`sudo apt-get update`;
  await $`sudo apt-get install -y --no-install-recommends pkg-config libx11-dev libasound2-dev libudev-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev clang mold libwayland-dev libxkbcommon-dev`;
}

async function installWasmDeps() {
  // Install trunk for WASM builds
  await $`cargo install trunk`;
}

async function buildWasm(env: Env = { binary: "dinosaur-game" }) {
  // We use trunk to build the project
  // Trunk handles assets and entry points via web/index.html
  // We explicitly output to 'dist' (default, but explicit is good)
  // We specify the manifest path to the game crate
  await $`RUSTFLAGS='--cfg getrandom_backend="wasm_js"' trunk build web/index.html --release --dist dist --manifest-path crates/game/Cargo.toml`;
}

async function buildRelease() {
  await $`cargo b --release`;
}

async function buildNative(target: string) {
    // Ensure target is added
    await $`rustup target add ${target}`;
    // Install cranelift component
    await $`rustup component add rustc-codegen-cranelift-preview --toolchain nightly`;

    // Set env vars specific to targets
    if (target.includes("apple")) {
        if (target.includes("x86_64")) {
             process.env.CFLAGS = "-fno-stack-check";
             process.env.MACOSX_DEPLOYMENT_TARGET = "10.9";
        } else if (target.includes("aarch64")) {
             process.env.MACOSX_DEPLOYMENT_TARGET = "11";
        }
    }

    await $`cargo build --release --target ${target}`;
}

async function packageLinux(target: string, binary: string, version: string) {
    await $`rm -rf linux`;
    await $`mkdir -p linux`;
    await $`cp target/${target}/release/${binary} linux/`;
    if ((await $`test -d assets`.nothrow()).exitCode === 0) {
        await $`cp -r assets linux/`;
    }

    // Zip naming convention: binary-linux-version.zip
    // The CI currently names it ${binary}.zip inside the artifact, and renames it for release.
    // We will create the properly named file.
    const zipName = `${binary}-linux-${version}.zip`;
    // zip -r relative path
    await $`cd linux && zip -r ../${zipName} .`;
    console.log(`Created ${zipName}`);
}

async function packageWindows(target: string, binary: string, version: string) {
    await $`rm -rf windows`;
    await $`mkdir -p windows`;
    await $`cp target/${target}/release/${binary}.exe windows/`;

    // Check if assets directory exists before copying
    if ((await $`test -d assets`.nothrow()).exitCode === 0) {
        await $`cp -r assets windows/`;
    }

    const zipName = `${binary}-windows-${version}.zip`;
    // Using PowerShell to zip because zip might not be available on Windows environment,
    // and Bun shell doesn't have a built-in zip command (it delegates to system).
    await $`powershell -Command "Compress-Archive -Path windows/* -DestinationPath ${zipName} -Force"`;
    console.log(`Created ${zipName}`);
}

async function packageMac(target: string, binary: string, version: string) {
    const arch = target.includes("aarch64") ? "apple-silicon" : "intel";
    const appName = `${binary}.app`;

    await $`mkdir -p ${appName}/Contents/MacOS`;
    await $`cp target/${target}/release/${binary} ${appName}/Contents/MacOS/`;

    if ((await $`test -d assets`.nothrow()).exitCode === 0) {
        await $`cp -r assets ${appName}/Contents/MacOS/`;
    }

    const dmgName = `${binary}-macOS-${arch}-${version}.dmg`;
    await $`rm -f ${dmgName}`;
    await $`hdiutil create -fs HFS+ -volname "${binary}-${arch}" -srcfolder ${appName} ${dmgName}`;
    console.log(`Created ${dmgName}`);
}

async function packageNative(target: string, binary: string, version?: string) {
    if (!version) {
        version = (await $`git rev-parse --short HEAD`.text()).trim();
    }

    if (target.includes("linux")) {
        await packageLinux(target, binary, version);
    } else if (target.includes("windows")) {
        await packageWindows(target, binary, version);
    } else if (target.includes("apple") || target.includes("darwin")) {
        await packageMac(target, binary, version);
    } else {
        throw new Error(`Unsupported target for packaging: ${target}`);
    }
}

async function checkShouldRelease() {
    const eventName = process.env.GITHUB_EVENT_NAME;
    if (eventName !== "schedule") {
        console.log("true");
        return;
    }

    try {
        const result = await $`gh release list --limit 1 --json targetCommitish`.text();
        const releases = JSON.parse(result);

        if (releases.length === 0) {
            console.log("true");
            return;
        }

        const lastReleaseCommit = releases[0].targetCommitish;
        const currentCommit = (await $`git rev-parse HEAD`.text()).trim();

        if (lastReleaseCommit === currentCommit) {
            console.log("false");
        } else {
            console.log("true");
        }
    } catch (error) {
        console.error("Error checking release status:", error);
        console.log("true");
    }
}

async function getVersion() {
    const ref = process.env.GITHUB_REF || "";
    const eventName = process.env.GITHUB_EVENT_NAME;

    if (ref.startsWith("refs/tags/")) {
        console.log(ref.replace("refs/tags/", ""));
        return;
    }

    if (eventName === "schedule") {
        const date = new Date().toISOString().split("T")[0];
        console.log(`nightly-${date}`);
        return;
    }

    const sha = (await $`git rev-parse --short HEAD`.text()).trim();
    console.log(`dev-${sha}`);
}

async function test() {
    await $`cargo test --workspace`;
}

async function clippy() {
    await $`cargo clippy --workspace --all-targets --all-features -- -D warnings`;
}

async function fmt() {
    await $`cargo fmt --all -- --check`;
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

cli.command("web", "Web build")
  .action(async () => {
    await installWasmDeps();
    await buildWasm();
  });

cli.command("check-should-release", "Check if a release is needed")
    .action(async () => {
        await checkShouldRelease();
    });

cli.command("get-version", "Get the version string")
    .action(async () => {
        await getVersion();
    });

cli.command("test", "Run tests")
    .action(async () => {
        await test();
    });

cli.command("clippy", "Run clippy")
    .action(async () => {
        await clippy();
    });

cli.command("fmt", "Run fmt")
    .action(async () => {
        await fmt();
    });

cli.command("build-native <target>", "Build native binary for target")
    .action(async (target) => {
        await buildNative(target);
    });

cli.command("package-native <target>", "Package native binary for target")
    .option("--app-version <version>", "Version string")
    .action(async (target, options) => {
        await packageNative(target, DEFAULT_BINARY, options.appVersion);
    });

cli.help();
cli.version("0.1.0");

cli.parse();
