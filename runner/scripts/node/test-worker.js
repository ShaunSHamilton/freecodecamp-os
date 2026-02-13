import { parentPort, workerData } from "node:worker_threads";
// These are used in the local scope of the `eval` in `runTests`
import { assert, AssertionError, expect, config as chaiConfig } from "chai";

const { before_each = "", project } = workerData;

parentPort.on("message", async ({ code, id }) => {
  let passed = false;
  let error = null;
  try {
    const _eval_out = await eval(`(async () => {
      ${before_each}
      ${code}
})();`);
    passed = true;
  } catch (e) {
    error = {};
    Object.getOwnPropertyNames(e).forEach((key) => {
      error[key] = e[key];
    });
    // Cannot pass `e` "as is", because classes cannot be passed between threads
    error.type = e instanceof AssertionError ? "AssertionError" : "Error";
  }
  parentPort.postMessage({ passed, id, error });
});
