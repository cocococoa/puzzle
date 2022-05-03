/* tslint:disable */
/* eslint-disable */
/**
*/
export class Array64 {
  free(): void;
}
/**
*/
export class Latin {
  free(): void;
/**
* @returns {Latin}
*/
  static mynew(): Latin;
/**
* @returns {TransversalList}
*/
  transversal(): TransversalList;
/**
* @returns {LatinList}
*/
  orthogonal(): LatinList;
/**
* @returns {number}
*/
  size(): number;
/**
* @param {number} i
* @param {number} j
* @param {number} v
*/
  set(i: number, j: number, v: number): void;
/**
* @param {number} i
* @param {number} j
* @returns {number}
*/
  get(i: number, j: number): number;
/**
* @returns {string}
*/
  render(): string;
}
/**
*/
export class LatinList {
  free(): void;
/**
* @returns {number}
*/
  size(): number;
/**
* @param {number} idx
* @returns {Latin}
*/
  get(idx: number): Latin;
}
/**
*/
export class Transversal {
  free(): void;
/**
* @param {number} j
* @param {number} v
*/
  set(j: number, v: number): void;
/**
* @param {number} j
* @returns {number}
*/
  get(j: number): number;
}
/**
*/
export class TransversalList {
  free(): void;
/**
* @returns {number}
*/
  size(): number;
/**
* @param {number} idx
* @returns {Transversal}
*/
  get(idx: number): Transversal;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_latin_free: (a: number) => void;
  readonly latin_mynew: () => number;
  readonly latin_transversal: (a: number) => number;
  readonly latin_orthogonal: (a: number) => number;
  readonly latin_size: (a: number) => number;
  readonly latin_set: (a: number, b: number, c: number, d: number) => void;
  readonly latin_get: (a: number, b: number, c: number) => number;
  readonly latin_render: (a: number, b: number) => void;
  readonly __wbg_latinlist_free: (a: number) => void;
  readonly latinlist_size: (a: number) => number;
  readonly latinlist_get: (a: number, b: number) => number;
  readonly __wbg_transversal_free: (a: number) => void;
  readonly transversal_set: (a: number, b: number, c: number) => void;
  readonly transversal_get: (a: number, b: number) => number;
  readonly __wbg_transversallist_free: (a: number) => void;
  readonly transversallist_size: (a: number) => number;
  readonly transversallist_get: (a: number, b: number) => number;
  readonly __wbg_array64_free: (a: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
