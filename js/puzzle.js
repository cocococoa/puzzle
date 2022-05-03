
let wasm;

const cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}
/**
*/
export class Array64 {

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_array64_free(ptr);
    }
}
/**
*/
export class Latin {

    static __wrap(ptr) {
        const obj = Object.create(Latin.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_latin_free(ptr);
    }
    /**
    * @returns {Latin}
    */
    static mynew() {
        const ret = wasm.latin_mynew();
        return Latin.__wrap(ret);
    }
    /**
    * @returns {TransversalList}
    */
    transversal() {
        const ret = wasm.latin_transversal(this.ptr);
        return TransversalList.__wrap(ret);
    }
    /**
    * @returns {LatinList}
    */
    orthogonal() {
        const ret = wasm.latin_orthogonal(this.ptr);
        return LatinList.__wrap(ret);
    }
    /**
    * @returns {number}
    */
    size() {
        const ret = wasm.latin_size(this.ptr);
        return ret;
    }
    /**
    * @param {number} i
    * @param {number} j
    * @param {number} v
    */
    set(i, j, v) {
        wasm.latin_set(this.ptr, i, j, v);
    }
    /**
    * @param {number} i
    * @param {number} j
    * @returns {number}
    */
    get(i, j) {
        const ret = wasm.latin_get(this.ptr, i, j);
        return ret;
    }
    /**
    * @returns {string}
    */
    render() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.latin_render(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
}
/**
*/
export class LatinList {

    static __wrap(ptr) {
        const obj = Object.create(LatinList.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_latinlist_free(ptr);
    }
    /**
    * @returns {number}
    */
    size() {
        const ret = wasm.latinlist_size(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} idx
    * @returns {Latin}
    */
    get(idx) {
        const ret = wasm.latinlist_get(this.ptr, idx);
        return Latin.__wrap(ret);
    }
}
/**
*/
export class Transversal {

    static __wrap(ptr) {
        const obj = Object.create(Transversal.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_transversal_free(ptr);
    }
    /**
    * @param {number} j
    * @param {number} v
    */
    set(j, v) {
        wasm.transversal_set(this.ptr, j, v);
    }
    /**
    * @param {number} j
    * @returns {number}
    */
    get(j) {
        const ret = wasm.transversal_get(this.ptr, j);
        return ret;
    }
}
/**
*/
export class TransversalList {

    static __wrap(ptr) {
        const obj = Object.create(TransversalList.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_transversallist_free(ptr);
    }
    /**
    * @returns {number}
    */
    size() {
        const ret = wasm.transversallist_size(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} idx
    * @returns {Transversal}
    */
    get(idx) {
        const ret = wasm.transversallist_get(this.ptr, idx);
        return Transversal.__wrap(ret);
    }
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = new URL('puzzle_bg.wasm', import.meta.url);
    }
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }



    const { instance, module } = await load(await input, imports);

    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;

    return wasm;
}

export default init;

