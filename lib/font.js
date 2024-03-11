import { StringDecoder } from 'string_decoder';  // !node

let wasm;
let registry = null;
let ref = { deref() { } };

{
    const module = new WebAssembly.Module(WASM_BYTES);
    const instance = ref.deref() ?? (ref = new WeakRef(new WebAssembly.Instance(module))).deref();

    wasm = instance.exports;
}

class mem {
    static length() { return wasm.wlen(); }
    static token() { return wasm.wtoken(); }
    static alloc(size) { return wasm.walloc(size); }
    static free(ptr, size) { return wasm.wfree(ptr, size); }
    static u8(ptr, size) { return new Uint8Array(wasm.memory.buffer, ptr, size); }
    static u32(ptr, size) { return new Uint32Array(wasm.memory.buffer, ptr, size); }
    static gc(f) { return !('FinalizationRegistry' in globalThis) ? { delete(_) { }, add(_, __) { } } : { r: new FinalizationRegistry(f), delete(k) { this.r.unregister(k); }, add(k, v) { this.r.register(k, v, k); } }; }

    static copy_and_free(ptr, size) {
        let slice = mem.u8(ptr, size).slice();
        return (wasm.wfree(ptr, size), slice);
    }
}

const decode_utf8 = globalThis.Deno?.core?.decode ?? StringDecoder.prototype.end.bind(new StringDecoder);
const encode_utf8 = globalThis.Deno?.core?.encode ?? globalThis.Buffer?.from.bind(globalThis.Buffer) ?? TextEncoder.prototype.encode.bind(new TextEncoder);

if ('FinalizationRegistry' in globalThis) {
    registry = new FinalizationRegistry(([t, ptr]) => {
        if (t === 0) wasm.font_free(ptr);
        if (t === 1) wasm.layout_free(ptr);
    });
}

export class Font {
    constructor(scale, buffer) {
        this.scale = scale;
        const ptr = mem.alloc(buffer.length);
        mem.u8(ptr, buffer.length).set(buffer);
        this.ptr = wasm.font_new(ptr, buffer.length, scale);

        if (!this.ptr) throw new Error('invalid font');
        if (registry) registry.register(this, [0, this.ptr], this);
    }

    free() {
        this.ptr = wasm.font_free(this.ptr);
        if (registry) registry.unregister(this);
    }

    has(char) {
        return wasm.font_has(this.ptr, String.prototype.charCodeAt.call(char, 0));
    }

    metrics(char, scale = this.scale) {
        const ptr = wasm.font_metrics(this.ptr, String.prototype.charCodeAt.call(char, 0), scale);
        const metrics = JSON.parse(decode_utf8(mem.copy_and_free(wasm.font_metrics_buffer(ptr), mem.length())));

        return (wasm.font_metrics_free(ptr), metrics);
    }

    rasterize(char, scale = this.scale) {
        const ptr = wasm.font_rasterize(this.ptr, String.prototype.charCodeAt.call(char, 0), scale);

        const glyph = {
            buffer: mem.u8(wasm.font_rasterize_buffer(ptr), mem.length()).slice(),
            metrics: JSON.parse(decode_utf8(mem.copy_and_free(wasm.font_rasterize_metrics(ptr), mem.length()))),
        }

        return (wasm.font_rasterize_free(ptr), glyph);
    }
}

export class Layout {
    constructor() {
        this.ptr = wasm.layout_new();
        if (registry) this.refs = [];
        if (registry) registry.register(this, [1, this.ptr], this);
    }

    clear() {
        wasm.layout_clear(this.ptr);
        if (registry) this.refs.length = 0;
    }

    lines() {
        return wasm.layout_lines(this.ptr);
    }

    free() {
        if (registry) this.refs.length = 0;
        this.ptr = wasm.layout_free(this.ptr);
        if (registry) registry.unregister(this);
    }

    reset(options = {}) {
        options = encode_utf8(JSON.stringify(options));

        if (registry) this.refs.length = 0;
        const ptr = mem.alloc(options.length);
        mem.u8(ptr, options.length).set(options);
        wasm.layout_reset(this.ptr, ptr, options.length);
    }

    append(font, text, init) {
        text = encode_utf8(text);
        const options = init || {};
        if (registry) this.refs.push(font);
        const ptr = mem.alloc(text.length);
        mem.u8(ptr, text.length).set(text);
        const has_color = ('r' in options) || ('g' in options) || ('b' in options) || ('a' in options);
        wasm.layout_append(this.ptr, font.ptr, ptr, text.length, options.scale ?? font.scale, has_color, options.r, options.g, options.b);
    }

    rasterize(r, g, b, a = 255) {
        const ptr = wasm.layout_rasterize(this.ptr, r, g, b, a);
        
        const framebuffer = {
            width: wasm.layout_rasterize_width(ptr),
            height: wasm.layout_rasterize_height(ptr),
            buffer: mem.u8(wasm.layout_rasterize_buffer(ptr), mem.length()).slice(),
        }

        return (wasm.layout_rasterize_free(ptr), framebuffer);
    }
}