import init, { wasm_main } from "./pkg/daydream_druid.js";

async function run() {
  await init();
  wasm_main();
}

run();