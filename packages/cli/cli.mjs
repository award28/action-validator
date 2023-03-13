#!/usr/bin/env node
// @ts-check

import * as actionValidator from "@action-validator/core";

let response = actionValidator.entrypoint(
  JSON.stringify({"args": process.argv.slice(1)})
);

if (response.errors) {
  console.error(response.errors);
}
process.exit(response.exit_code);
