import { clamp_host, data_view, UTF8_DECODER, utf8_encode, UTF8_ENCODED_LEN, Slab, PROMISES, with_current_promise } from './intrinsics.js';
export class Exports {
  constructor() {
    this._resource0_slab = new Slab();
  }
  addToImports(imports) {
    if (!("canonical_abi" in imports)) imports["canonical_abi"] = {};
    
    imports.canonical_abi['resource_drop_Tarball'] = i => {
      this._resource0_slab.remove(i).drop();
    };
    imports.canonical_abi['resource_clone_Tarball'] = i => {
      const obj = this._resource0_slab.get(i);
      return this._resource0_slab.insert(obj.clone())
    };
    imports.canonical_abi['resource_get_Tarball'] = i => {
      return this._resource0_slab.get(i)._wasm_val;
    };
    imports.canonical_abi['resource_new_Tarball'] = i => {
      const registry = this._registry0;
      return this._resource0_slab.insert(new Tarball(i, this));
    };
    
    imports.canonical_abi['async_export_done'] = (ctx, ptr) => {
      PROMISES.remove(ctx)(ptr >>> 0)
    };
  }
  
  async instantiate(module, imports) {
    imports = imports || {};
    this.addToImports(imports);
    
    if (module instanceof WebAssembly.Instance) {
      this.instance = module;
    } else if (module instanceof WebAssembly.Module) {
      this.instance = await WebAssembly.instantiate(module, imports);
    } else if (module instanceof ArrayBuffer || module instanceof Uint8Array) {
      const { instance } = await WebAssembly.instantiate(module, imports);
      this.instance = instance;
    } else {
      const { instance } = await WebAssembly.instantiateStreaming(module, imports);
      this.instance = instance;
    }
    this._exports = this.instance.exports;
    this._registry0 = new FinalizationRegistry(this._exports['canonical_abi_drop_Tarball']);
  }
}

export class Tarball {
  constructor(wasm_val, obj) {
    this._wasm_val = wasm_val;
    this._obj = obj;
    this._refcnt = 1;
    obj._registry0.register(this, wasm_val, this);
  }
  
  clone() {
    this._refcnt += 1;
    return this;
  }
  
  drop() {
    this._refcnt -= 1;
    if (this._refcnt !== 0)
    return;
    this._obj._registry0.unregister(this);
    const dtor = this._obj._exports['canonical_abi_drop_Tarball'];
    const wasm_val = this._wasm_val;
    delete this._obj;
    delete this._refcnt;
    delete this._wasm_val;
    dtor(wasm_val);
  }
  static async fetch(exports, arg0) {
    const memory = exports._exports.memory;
    const realloc = exports._exports["canonical_abi_realloc"];
    const free = exports._exports["canonical_abi_free"];
    const ptr0 = utf8_encode(arg0, realloc, memory);
    const len0 = UTF8_ENCODED_LEN;
    const [ret0, ret1, ret2, ret3] = await new Promise((resolve, reject) => {
      const promise_ctx = PROMISES.insert(val => {
        if (typeof val !== 'number')
        return reject(val);
        resolve([data_view(memory).getInt32(val + 0, true), data_view(memory).getInt32(val + 8, true), data_view(memory).getInt32(val + 16, true), data_view(memory).getInt32(val + 24, true)]);
      });
      with_current_promise(promise_ctx, _prev => {
        exports._exports['Tarball::fetch'](ptr0, len0, promise_ctx);
      });
    });
    let variant5;
    switch (ret0) {
      case 0: {
        variant5 = {
          tag: "ok",
          val: exports._resource0_slab.remove(ret1),
        };
        break;
      }
      case 1: {
        let variant4;
        switch (ret1) {
          case 0: {
            const ptr1 = ret2;
            const len1 = ret3;
            const list1 = UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr1, len1));
            free(ptr1, len1, 1);
            variant4 = {
              tag: "BadStatus",
              val: list1,
            };
            break;
          }
          case 1: {
            const ptr2 = ret2;
            const len2 = ret3;
            const list2 = UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr2, len2));
            free(ptr2, len2, 1);
            variant4 = {
              tag: "NotUtf8",
              val: list2,
            };
            break;
          }
          case 2: {
            const ptr3 = ret2;
            const len3 = ret3;
            const list3 = UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr3, len3));
            free(ptr3, len3, 1);
            variant4 = {
              tag: "Io",
              val: list3,
            };
            break;
          }
          default:
          throw new RangeError("invalid variant discriminant for Error");
        }
        variant5 = {
          tag: "err",
          val: variant4,
        };
        break;
      }
      default:
      throw new RangeError("invalid variant discriminant for expected");
    }
    return variant5;
  }
  files() {
    const memory = this._obj._exports.memory;
    const free = this._obj._exports["canonical_abi_free"];
    const obj0 = this;
    const ret = this._obj._exports['Tarball::files'](this._obj._resource0_slab.insert(obj0.clone()));
    const len2 = data_view(memory).getInt32(ret + 8, true);
    const base2 = data_view(memory).getInt32(ret + 0, true);
    const result2 = [];
    for (let i = 0; i < len2; i++) {
      const base = base2 + i * 8;
      const ptr1 = data_view(memory).getInt32(base + 0, true);
      const len1 = data_view(memory).getInt32(base + 4, true);
      const list1 = UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr1, len1));
      free(ptr1, len1, 1);
      result2.push(list1);
    }
    free(base2, len2 * 8, 4);
    return result2;
  }
  contents(arg1) {
    const memory = this._obj._exports.memory;
    const free = this._obj._exports["canonical_abi_free"];
    const obj0 = this;
    const ret = this._obj._exports['Tarball::contents'](this._obj._resource0_slab.insert(obj0.clone()), clamp_host(arg1, 0, 4294967295));
    const ptr1 = data_view(memory).getInt32(ret + 0, true);
    const len1 = data_view(memory).getInt32(ret + 8, true);
    const list1 = UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr1, len1));
    free(ptr1, len1, 1);
    return list1;
  }
}
