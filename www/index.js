import * as deepfield from "deepfield";

import { memory } from "deepfield/deepfield_bg";

window.getWasmMemory = function(start, length) {
  return new Uint8Array(memory.buffer, start, length)
}