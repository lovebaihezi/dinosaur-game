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

async function checkShouldRelease() {
    // If explicitly triggered by a tag or push (not schedule), we usually always want to release
    // But the job logic in GHA might rely on this too.
    // However, the requirement is specifically for SCHEDULED builds: "only if main branch got updates".

    const eventName = process.env.GITHUB_EVENT_NAME;
    if (eventName !== "schedule") {
        console.log("true");
        return;
    }

    try {
        // Fetch the latest release's target commit
        // We assume 'gh' is installed and authenticated in the CI environment
        // output format: title, tagName, targetCommitish
        const result = await $`gh release list --limit 1 --json targetCommitish`.text();
        const releases = JSON.parse(result);

        if (releases.length === 0) {
            // No releases yet, so we should release
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
        // If something fails (e.g. gh not found, no releases), default to true to be safe?
        // Or false?
        console.error("Error checking release status:", error);
        // Defaulting to true so we don't miss a release due to error,
        // but arguably if we can't verify, we might fail.
        // Let's print true but log error to stderr.
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

    // Fallback for non-tag, non-schedule (e.g. manual dispatch or push to main without tag)
    // Use short sha
    const sha = (await $`git rev-parse --short HEAD`.text()).trim();
    console.log(`dev-${sha}`);
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

cli.command("install-rust-deps", "Install rust dependencies")
  .option("--target <target>", "Target architecture")
  .action(async (options) => {
    if (options.target) {
        await $`rustup target add ${options.target}`;
    }
    await $`rustup component add rustc-codegen-cranelift-preview --toolchain nightly`;
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

cli.command("build-native", "Build native binary")
  .option("--target <target>", "Target architecture")
  .action(async (options) => {
    if (options.target) {
        await $`cargo build --release --target ${options.target}`;
    } else {
        await $`cargo build --release`;
    }
  });

cli.command("package-native", "Package native binary")
  .option("--target <target>", "Target architecture")
  .option("--binary <binary>", "Binary name")
  .option("--os <os>", "Operating system (linux, windows, macos)")
  .option("--arch <arch>", "Architecture (intel, silicon) for macOS")
  .option("--app-version <version>", "App version for naming")
  .action(async (options) => {
    const binary = options.binary || "dinosaur-game";
    const os = options.os || "linux";
    const target = options.target || "x86_64-unknown-linux-gnu";
    const version = options.appVersion || "latest";

    // Clean previous package dirs if needed, but CI usually is clean.

    if (os === "linux") {
        await $`mkdir -p linux`;
        await $`cp target/${target}/release/${binary} linux/`;
        // Use nothrow() for optional copy, or check existence.
        // Bun shell throws on error by default.
        if ((await $`test -d assets`.nothrow()).exitCode === 0) {
             await $`cp -r assets linux/`;
        }
        await $`rm -f ${binary}.zip`;
        await $`cd linux && zip -r ../${binary}.zip .`;
    } else if (os === "windows") {
        await $`mkdir -p windows`;
        await $`cp target/${target}/release/${binary}.exe windows/`;
        if ((await $`test -d assets`.nothrow()).exitCode === 0) {
            await $`mkdir -p windows/assets`;
             await $`cp -r assets windows/`;
        }
        await $`rm -f ${binary}.zip`;
        // Use PowerShell to zip on Windows
        await $`powershell Compress-Archive -Path windows/* -DestinationPath ${binary}.zip`;
    } else if (os === "macos") {
        const appName = `${binary}.app`;
        const dmgName = `${binary}-${options.arch === "silicon" ? "macOS-apple-silicon" : "macOS-intel"}.dmg`;

        await $`mkdir -p ${appName}/Contents/MacOS`;
        await $`cp target/${target}/release/${binary} ${appName}/Contents/MacOS/`;
         if ((await $`test -d assets`.nothrow()).exitCode === 0) {
             await $`cp -r assets ${appName}/Contents/MacOS/`;
        }
        await $`rm -f ${dmgName}`;
        await $`hdiutil create -fs HFS+ -volname "${binary}" -srcfolder ${appName} ${dmgName}`;
    }
  });

cli.command("check-should-release", "Check if a release is needed")
    .action(async () => {
        await checkShouldRelease();
    });

cli.command("get-version", "Get the version string")
    .action(async () => {
        await getVersion();
    });

cli.help();
cli.version("0.1.0");

cli.parse();
