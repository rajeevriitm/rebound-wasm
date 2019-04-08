import wasm  from '../pkg/rebound_wasm_bg.wasm';
const exported = wasm({}).exports;
export function getArray() {
  let ptr = exported.get_array_loc();
  return new Float64Array(exported.memory.buffer,ptr,7)
}
export const advance = exported.advance;
