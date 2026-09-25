#!/usr/bin/env node
"use strict";

const fs = require("node:fs");

const [version, armSha, intelSha] = process.argv.slice(2);
if (!version || !/^[0-9]+\.[0-9]+\.[0-9]+$/.test(version) || !/^[a-f0-9]{64}$/.test(armSha) || !/^[a-f0-9]{64}$/.test(intelSha)) {
  console.error("usage: sync-formula.cjs VERSION ARM_SHA256 INTEL_SHA256");
  process.exit(2);
}

const path = "Formula/silicon-sanctum.rb";
let formula = fs.readFileSync(path, "utf8");
formula = formula.replace(/version "[^"]+"/, `version "${version}"`);
formula = formula.replace(/sha256 "[^"]+"/, `sha256 "${armSha}"`);
formula = formula.replace(/sha256 "[^"]+"/, `sha256 "${intelSha}"`);
fs.writeFileSync(path, formula);
