/** For internal Diplomat use when constructing opaques or structs. */
export const internalConstructor = Symbol("constructor");

export function readString8(wasm, ptr, len) {
    const buf = new Uint8Array(wasm.memory.buffer, ptr, len);
    return (new TextDecoder("utf-8")).decode(buf)
}

export function readString16(wasm, ptr, len) {
    const buf = new Uint16Array(wasm.memory.buffer, ptr, len);
    return String.fromCharCode.apply(null, buf)
}

export function withDiplomatWrite(wasm, callback) {
    const write = wasm.diplomat_buffer_write_create(0);
    try {
    callback(write);
    const outStringPtr = wasm.diplomat_buffer_write_get_bytes(write);
    if (outStringPtr === null) {
        throw Error("Out of memory");
    }
    const outStringLen = wasm.diplomat_buffer_write_len(write);
    return readString8(wasm, outStringPtr, outStringLen);
    } finally {
    wasm.diplomat_buffer_write_destroy(write);
    }
}

/**
 * Get the pointer returned by an FFI function.
 * 
 * It's tempting to call `(new Uint32Array(wasm.memory.buffer, FFI_func(), 1))[0]`.
 * However, there's a chance that `wasm.memory.buffer` will be resized between
 * the time it's accessed and the time it's used, invalidating the view.
 * This function ensures that the view into wasm memory is fresh.
 * 
 * This is used for methods that return multiple types into a wasm buffer, where
 * one of those types is another ptr. Call this method to get access to the returned
 * ptr, so the return buffer can be freed.
 * @param {WebAssembly.Exports} wasm Provided by diplomat generated files. 
 * @param {number} ptr Pointer of a pointer, to be read.
 * @returns {number} The underlying pointer.
 */
export function ptrRead(wasm, ptr) {
    return (new Uint32Array(wasm.memory.buffer, ptr, 1))[0];
}

/** 
 * Get the flag of a result type.
 */
export function resultFlag(wasm, ptr, offset) {
    return (new Uint8Array(wasm.memory.buffer, ptr + offset, 1))[0];
}

/** 
 * Get the discriminant of a Rust enum.
*/
export function enumDiscriminant(wasm, ptr) {
    return (new Int32Array(wasm.memory.buffer, ptr, 1))[0]
}

/** 
 * A wrapper around a slice of WASM memory that can be freed manually or
 * automatically by the garbage collector.
 *
 * This type is necessary for Rust functions that take a `&str` or `&[T]`, since
 * they can create an edge to this object if they borrow from the str/slice,
 * or we can manually free the WASM memory if they don't.
 */
export class DiplomatBuf {
    static str8 = (wasm, string) => {
    var utf8Length = 0;
    for (const codepointString of string) {
        let codepoint = codepointString.codePointAt(0);
        if (codepoint < 0x80) {
        utf8Length += 1
        } else if (codepoint < 0x800) {
        utf8Length += 2
        } else if (codepoint < 0x10000) {
        utf8Length += 3
        } else {
        utf8Length += 4
        }
    }

    const ptr = wasm.diplomat_alloc(utf8Length, 1);

    const result = (new TextEncoder()).encodeInto(string, new Uint8Array(wasm.memory.buffer, ptr, utf8Length));
    console.assert(string.length === result.read && utf8Length === result.written, "UTF-8 write error");

    return new DiplomatBuf(ptr, utf8Length, () => wasm.diplomat_free(ptr, utf8Length, 1));
    }

    static str16 = (wasm, string) => {
    const byteLength = string.length * 2;
    const ptr = wasm.diplomat_alloc(byteLength, 2);

    const destination = new Uint16Array(wasm.memory.buffer, ptr, string.length);
    for (let i = 0; i < string.length; i++) {
        destination[i] = string.charCodeAt(i);
    }

    return new DiplomatBuf(ptr, string.length, () => wasm.diplomat_free(ptr, byteLength, 2));
    }

    static slice = (wasm, list, rustType) => {
    const elementSize = rustType === "u8" || rustType === "i8" || rustType === "boolean" ? 1 :
        rustType === "u16" || rustType === "i16" ? 2 :
        rustType === "u64" || rustType === "i64" || rustType === "f64" ? 8 :
            4;

    const byteLength = list.length * elementSize;
    const ptr = wasm.diplomat_alloc(byteLength, elementSize);

    /** 
     * Create an array view of the buffer. This gives us the `set` method which correctly handles untyped values
     */
    const destination =
        rustType === "u8" || rustType === "boolean" ? new Uint8Array(wasm.memory.buffer, ptr, byteLength) :
        rustType === "i8" ? new Int8Array(wasm.memory.buffer, ptr, byteLength) :
            rustType === "u16" ? new Uint16Array(wasm.memory.buffer, ptr, byteLength) :
            rustType === "i16" ? new Int16Array(wasm.memory.buffer, ptr, byteLength) :
                rustType === "i32" ? new Int32Array(wasm.memory.buffer, ptr, byteLength) :
                rustType === "u64" ? new BigUint64Array(wasm.memory.buffer, ptr, byteLength) :
                    rustType === "i64" ? new BigInt64Array(wasm.memory.buffer, ptr, byteLength) :
                    rustType === "f32" ? new Float32Array(wasm.memory.buffer, ptr, byteLength) :
                        rustType === "f64" ? new Float64Array(wasm.memory.buffer, ptr, byteLength) :
                        new Uint32Array(wasm.memory.buffer, ptr, byteLength);
    destination.set(list);

    return new DiplomatBuf(ptr, list.length, () => wasm.diplomat_free(ptr, byteLength, elementSize));
    }

    
    static strs = (wasm, strings, encoding) => {
        let encodeStr = (encoding === "string16") ? DiplomatBuf.str16 : DiplomatBuf.str8;

        const byteLength = strings.length * 4 * 2;

        const ptr = wasm.diplomat_alloc(byteLength, 4);

        const destination = new Uint32Array(wasm.memory.buffer, ptr, byteLength);

        const stringsAlloc = [];

        for (let i = 0; i < strings.length; i++) {
            stringsAlloc.push(encodeStr(wasm, strings[i]));

            destination[2 * i] = stringsAlloc[i].ptr;
            destination[(2 * i) + 1] = stringsAlloc[i].size;
        }

        return new DiplomatBuf(ptr, strings.length, () => {
            wasm.diplomat_free(ptr, byteLength, 4);
            for (let i = 0; i < stringsAlloc.length; i++) {
                stringsAlloc[i].free();
            }
        });
    }

    /**
     * Generated code calls one of methods these for each allocation, to either
     * free directly after the FFI call, to leak (to create a &'static), or to
     * register the buffer with the garbage collector (to create a &'a).
     */
    free;

    constructor(ptr, size, free) {
        this.ptr = ptr;
        this.size = size;
        this.free = free;
        this.leak = () => { };
        this.garbageCollect = () => DiplomatBufferFinalizer.register(this, this.free);
    }

    splat() {
        return [this.ptr, this.size];
    }
}

/** 
 * Helper class for creating and managing `diplomat_buffer_write`.
 * Meant to minimize direct calls to `wasm`.
 */
export class DiplomatWriteBuf {
    leak;

    #wasm;
    #buffer;

    constructor(wasm) {
        this.#wasm = wasm;
        this.#buffer = this.#wasm.diplomat_buffer_write_create(0);

        this.leak = () => { };
    }
    
    free() {
        this.#wasm.diplomat_buffer_write_destroy(this.#buffer);
    }

    garbageCollect() {
        DiplomatBufferFinalizer.register(this, this.free);
    }

    readString8() {
        return readString8(this.#wasm, this.ptr, this.size);
    }

    get buffer() {
        return this.#buffer;
    }

    get ptr() {
        return this.#wasm.diplomat_buffer_write_get_bytes(this.#buffer);
    }

    get size() {
        return this.#wasm.diplomat_buffer_write_len(this.#buffer);
    }
}

/**
 * Represents an underlying slice that we've grabbed from WebAssembly.
 * You can treat this in JS as a regular slice of primitives, but it handles additional data for you behind the scenes.
 */
export class DiplomatSlice {
    #wasm;

    #bufferType;
    get bufferType() {
        return this.#bufferType;
    }

    #buffer;
    get buffer() {
        return this.#buffer;
    }

    #lifetimeEdges;

    constructor(wasm, buffer, bufferType, lifetimeEdges) {
        this.#wasm = wasm;
        
        const [ptr, size] = new Uint32Array(this.#wasm.memory.buffer, buffer, 2);

        this.#buffer = new bufferType(this.#wasm.memory.buffer, ptr, size);
        this.#bufferType = bufferType;

        this.#lifetimeEdges = lifetimeEdges;
    }

    getValue() {
        return this.#buffer;
    }

    [Symbol.toPrimitive]() {
        return this.getValue();
    }

    valueOf() {
        return this.getValue();
    }
}

export class DiplomatSlicePrimitive extends DiplomatSlice {
    constructor(wasm, buffer, sliceType, lifetimeEdges) {
        const [ptr, size] = new Uint32Array(wasm.memory.buffer, buffer, 2);

        let arrayType;
        switch (sliceType) {
            case "u8":
            case "boolean":
                arrayType = Uint8Array;
                break;
            case "i8":
                arrayType = Int8Array;
                break;
            case "u16":
                arrayType = Uint16Array;
                break;
            case "i16":
                arrayType = Int16Array;
                break;
            case "i32":
                arrayType = Int32Array;
                break;
            case "u32":
                arrayType = Uint32Array;
                break;
            case "i64":
                arrayType = BigInt64Array;
                break;
            case "u64":
                arrayType = BigUint64Array;
                break;
            case "f32":
                arrayType = Float32Array;
                break;
            case "f64":
                arrayType = Float64Array;
                break;
            default:
                console.error("Unrecognized bufferType ", bufferType);
        }

        super(wasm, buffer, arrayType, lifetimeEdges);
    }
}

export class DiplomatSliceStr extends DiplomatSlice {
    #decoder;

    constructor(wasm, buffer, stringEncoding, lifetimeEdges) {
        let encoding;
        switch (stringEncoding) {
            case "string8":
                encoding = Uint8Array;
                break;
            case "string16":
                encoding = Uint16Array;
                break;
            default:
                console.error("Unrecognized stringEncoding ", stringEncoding);
                break;
        }
        super(wasm, buffer, encoding, lifetimeEdges);

        if (stringEncoding === "string8") {
            this.#decoder = new TextDecoder('utf-8');
        }
    }

    getValue() {
        switch (this.bufferType) {
            case Uint8Array:
                return this.#decoder.decode(super.getValue());
            case Uint16Array:
                return String.fromCharCode.apply(null, super.getValue());
            default:
                return null;
        }
    }

    toString() {
        return this.getValue();
    }
}

export class DiplomatSliceStrings extends DiplomatSlice {
    #strings = [];
    constructor(wasm, buffer, stringEncoding, lifetimeEdges) {
        super(wasm, buffer, Uint32Array, lifetimeEdges);

        for (let i = this.buffer.byteOffset; i < this.buffer.byteLength; i += this.buffer.BYTES_PER_ELEMENT * 2) {
            this.#strings.push(new DiplomatSliceStr(wasm, i, stringEncoding, lifetimeEdges));
        }
    }

    getValue() {
        return this.#strings;
    }
}

/**
 * A number of Rust functions in WebAssembly require a buffer to populate struct, slice, Option<> or Result<> types with information.
 * {@link DiplomatReceiveBuf} allocates a buffer in WebAssembly, which can then be passed into functions with the {@link DiplomatReceiveBuf.buffer}
 * property.
 */
export class DiplomatReceiveBuf {
    #wasm;

    #size;
    #align;

    #hasResult;

    #buffer;

    constructor(wasm, size, align, hasResult) {
        this.#wasm = wasm;

        this.#size = size;
        this.#align = align;

        this.#hasResult = hasResult;

        this.#buffer = this.#wasm.diplomat_alloc(this.#size, this.#align);

        this.leak = () => { };
    }
    
    free() {
        this.#wasm.diplomat_free(this.#buffer, this.#size, this.#align);
    }
    
    get buffer() {
        return this.#buffer;
    }

    /**
     * Only for when a DiplomatReceiveBuf is allocating a buffer for an `Option<>` or a `Result<>` type.
     * 
     * This just checks the last byte for a successful result (assuming that Rust's compiler does not change).
     */
    get resultFlag() {
        if (this.#hasResult) {
            return resultFlag(this.#wasm, this.#buffer, this.#size - 1);
        } else {
            return true;
        }
    }
}

/**
 * For cleaning up slices inside struct _intoFFI functions.
 * Based somewhat on how the Dart backend handles slice cleanup.
 * 
 * We want to ensure a slice only lasts as long as its struct, so we have a `functionCleanupArena` CleanupArena that we use in each method for any slice that needs to be cleaned up. It lasts only as long as the function is called for.
 * 
 * Then we have `createWith`, which is meant for longer lasting slices. It takes an array of edges and will last as long as those edges do. Cleanup is only called later.
 */
export class CleanupArena {
    #items = [];

    #edgeArray = [];
    
    constructor() {
    }
    
    alloc(item) {
        this.#items.push(item);
        return item;
    }

    createWith(edgeArray) {
        this.#edgeArray = edgeArray;
        DiplomatBufferFinalizer.register(this, this.free);
    }

    free() {
        this.#items.forEach((i) => {
            i.free();
        });
    }
}

const DiplomatBufferFinalizer = new FinalizationRegistry(free => free());