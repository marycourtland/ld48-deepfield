import * as deepfield from "deepfield";
import {Pixel, writePixel} from "./pixels";

import { memory } from "deepfield/deepfield_bg";

window.getWasmMemory = function(start, length) {
  return new Uint8Array(memory.buffer, start, length)
}

// Rust is encoding the pixels in the order (i32, i32, u8, u8, u8, u8)
function getPixel(loc) {
  let xy = new Int32Array(memory.buffer, loc, 2);
  // let xy2 = new Uint32Array(memory.buffer, loc, 2);
  let rgba = new Uint8Array(memory.buffer, loc + 8, 4)
  return Pixel(xy, rgba)
}

function getPixels(loc, n) {
  let pixels = [];
  for (let i = 0; i < n; i++) {
    pixels.push(getPixel(loc + 12 * i))
  }
  return pixels
}

window.refresh = function(numbers) {
  console.log("refreshing", numbers)
}

deepfield.init();