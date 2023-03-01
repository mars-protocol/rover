/* tslint:disable */
/* eslint-disable */
/**
 * @param {any} val
 * @returns {any}
 */
export function compute_health_js(val: any): any

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module

export interface InitOutput {
  readonly memory: WebAssembly.Memory
  readonly compute_health_js: (a: number) => number
  readonly allocate: (a: number) => number
  readonly deallocate: (a: number) => void
  readonly requires_iterator: () => void
  readonly interface_version_8: () => void
  readonly __wbindgen_malloc: (a: number) => number
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number
  readonly __wbindgen_free: (a: number, b: number) => void
  readonly __wbindgen_exn_store: (a: number) => void
}

export type SyncInitInput = BufferSource | WebAssembly.Module
/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {SyncInitInput} module
 *
 * @returns {InitOutput}
 */
export function initSync(module: SyncInitInput): InitOutput

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {InitInput | Promise<InitInput>} module_or_path
 *
 * @returns {Promise<InitOutput>}
 */
export default function init(module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>