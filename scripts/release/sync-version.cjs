#!/usr/bin/env node
"use strict";
const fs = require("node:fs");
const version = process.argv[2];
if (!version) process.exit(1);
fs.writeFileSync("VERSION", `${version}\n`);
const cargoPath = "crates/sanctum-runtime/Cargo.toml";
const cargo = fs.readFileSync(cargoPath, "utf8");
fs.writeFileSync(cargoPath, cargo.replace(/^version = "[^"]+"$/m, `version = "${version}"`));
