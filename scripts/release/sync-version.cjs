#!/usr/bin/env node
"use strict";
const fs = require("node:fs");
const version = process.argv[2];
if (!version) process.exit(1);
fs.writeFileSync("VERSION", `${version}\n`);
