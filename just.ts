#!/usr/bin/env -S deno run --allow-all
import $ from "@david/dax";
import { Command } from "@cliffy/command";

// TODO: Default Run Install Deps and build Release
// TODO: Migrate All workflow script to this file
await new Command()
  .name("just")
  .version("0.1.0")
  .description("Script for the dinosaur game")
  .parse(Deno.args);
