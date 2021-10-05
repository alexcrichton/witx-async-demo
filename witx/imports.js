import { clamp_host, data_view, UTF8_DECODER, utf8_encode, UTF8_ENCODED_LEN, Slab, PROMISES, with_current_promise } from './intrinsics.js';
export function addImportsToImports(imports, obj, get_export) {
  if (!("imports" in imports)) imports["imports"] = {};
  imports["imports"]["fetch"] = function(arg0, arg1, arg2, arg3) {
    const memory = get_export("memory");
    const realloc = get_export("canonical_abi_realloc");
    const ptr0 = arg0;
    const len0 = arg1;
    with_current_promise(null, cur_promise => {
      const catch_closure = e => PROMISES.remove(cur_promise)(e);
      obj.fetch(UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr0, len0))).then(e => {
        const ret = e;
        const variant3 = ret;
        let variant3_0;
        let variant3_1;
        let variant3_2;
        let variant3_3;
        switch (variant3.tag) {
          case "ok": {
            const e = variant3.val;
            variant3_0 = 0;
            variant3_1 = resources0.insert(e);
            variant3_2 = 0;
            variant3_3 = 0;
            break;
          }
          case "err": {
            const e = variant3.val;
            const variant2 = e;
            let variant2_0;
            let variant2_1;
            let variant2_2;
            switch (variant2.tag) {
              case "Failure": {
                const e = variant2.val;
                const ptr1 = utf8_encode(e, realloc, memory);
                const len1 = UTF8_ENCODED_LEN;
                variant2_0 = 0;
                variant2_1 = ptr1;
                variant2_2 = len1;
                break;
              }
              case "Aborted": {
                variant2_0 = 1;
                variant2_1 = 0;
                variant2_2 = 0;
                break;
              }
              default:
              throw new RangeError("invalid variant specified for Error");
            }
            variant3_0 = 1;
            variant3_1 = variant2_0;
            variant3_2 = variant2_1;
            variant3_3 = variant2_2;
            break;
          }
          default:
          throw new RangeError("invalid variant specified for expected");
        }
        with_current_promise(cur_promise, _prev => {
          get_export("__indirect_function_table").get(arg2)(arg3, variant3_0, variant3_1, variant3_2, variant3_3);
        });
      }, catch_closure);
    });
  };
  imports["imports"]["log"] = function(arg0, arg1) {
    const memory = get_export("memory");
    const ptr0 = arg0;
    const len0 = arg1;
    obj.log(UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr0, len0)));
  };
  imports["imports"]["log_err"] = function(arg0, arg1) {
    const memory = get_export("memory");
    const ptr0 = arg0;
    const len0 = arg1;
    obj.logErr(UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr0, len0)));
  };
  imports["imports"]["Response::headers"] = function(arg0) {
    const ret = resources0.get(arg0).headers();
    return resources1.insert(ret);
  };
  imports["imports"]["Response::body"] = function(arg0, arg1, arg2) {
    const memory = get_export("memory");
    const realloc = get_export("canonical_abi_realloc");
    with_current_promise(null, cur_promise => {
      const catch_closure = e => PROMISES.remove(cur_promise)(e);
      resources0.get(arg0).body().then(e => {
        const ret = e;
        const val0 = ret;
        const len0 = val0.length;
        const ptr0 = realloc(0, 0, 1, len0 * 1);
        (new Uint8Array(memory.buffer, ptr0, len0 * 1)).set(new Uint8Array(val0.buffer));
        with_current_promise(cur_promise, _prev => {
          get_export("__indirect_function_table").get(arg1)(arg2, ptr0, len0);
        });
      }, catch_closure);
    });
  };
  imports["imports"]["Response::status"] = function(arg0) {
    const ret = resources0.get(arg0).status();
    return clamp_host(ret, 0, 4294967295);
  };
  imports["imports"]["Response::status_text"] = function(arg0, arg1) {
    const memory = get_export("memory");
    const realloc = get_export("canonical_abi_realloc");
    const ret = resources0.get(arg0).statusText();
    const ptr0 = utf8_encode(ret, realloc, memory);
    const len0 = UTF8_ENCODED_LEN;
    data_view(memory).setInt32(arg1 + 8, len0, true);
    data_view(memory).setInt32(arg1 + 0, ptr0, true);
  };
  imports["imports"]["Headers::get"] = function(arg0, arg1, arg2, arg3) {
    const memory = get_export("memory");
    const realloc = get_export("canonical_abi_realloc");
    const ptr0 = arg1;
    const len0 = arg2;
    const ret = resources1.get(arg0).get(UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr0, len0)));
    const vec2 = ret;
    const len2 = vec2.length;
    const result2 = realloc(0, 0, 4, len2 * 8);
    for (let i = 0; i < vec2.length; i++) {
      const e = vec2[i];
      const base = result2 + i * 8;
      const ptr1 = utf8_encode(e, realloc, memory);
      const len1 = UTF8_ENCODED_LEN;
      data_view(memory).setInt32(base + 4, len1, true);
      data_view(memory).setInt32(base + 0, ptr1, true);
    }
    data_view(memory).setInt32(arg3 + 8, len2, true);
    data_view(memory).setInt32(arg3 + 0, result2, true);
  };
  imports["imports"]["Headers::entries"] = function(arg0, arg1) {
    const memory = get_export("memory");
    const realloc = get_export("canonical_abi_realloc");
    const ret = resources1.get(arg0).entries();
    const vec4 = ret;
    const len4 = vec4.length;
    const result4 = realloc(0, 0, 4, len4 * 16);
    for (let i = 0; i < vec4.length; i++) {
      const e = vec4[i];
      const base = result4 + i * 16;
      const [tuple0_0, tuple0_1] = e;
      const ptr1 = utf8_encode(tuple0_0, realloc, memory);
      const len1 = UTF8_ENCODED_LEN;
      data_view(memory).setInt32(base + 4, len1, true);
      data_view(memory).setInt32(base + 0, ptr1, true);
      const vec3 = tuple0_1;
      const len3 = vec3.length;
      const result3 = realloc(0, 0, 4, len3 * 8);
      for (let i = 0; i < vec3.length; i++) {
        const e = vec3[i];
        const base = result3 + i * 8;
        const ptr2 = utf8_encode(e, realloc, memory);
        const len2 = UTF8_ENCODED_LEN;
        data_view(memory).setInt32(base + 4, len2, true);
        data_view(memory).setInt32(base + 0, ptr2, true);
      }
      data_view(memory).setInt32(base + 12, len3, true);
      data_view(memory).setInt32(base + 8, result3, true);
    }
    data_view(memory).setInt32(arg1 + 8, len4, true);
    data_view(memory).setInt32(arg1 + 0, result4, true);
  };
  if (!("canonical_abi" in imports)) imports["canonical_abi"] = {};
  
  const resources0 = new Slab();
  imports.canonical_abi["resource_drop_Response"] = (i) => {
    const val = resources0.remove(i);
    if (obj.dropResponse)
    obj.dropResponse(val);
  };
  
  const resources1 = new Slab();
  imports.canonical_abi["resource_drop_Headers"] = (i) => {
    const val = resources1.remove(i);
    if (obj.dropHeaders)
    obj.dropHeaders(val);
  };
}