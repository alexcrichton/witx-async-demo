import * as i from './witx/imports.js';
import { Exports, Tarball, Error } from './witx/exports.js';

const input = document.getElementById('input-url') as HTMLInputElement;
const button = document.getElementById('input-button') as HTMLButtonElement;
const form = document.getElementById('form') as HTMLFormElement;
const allFiles = document.getElementById('files');
const editor = ace.edit("editor");
editor.setReadOnly(true);
editor.setOption("useWorker", false);
editor.session.setMode(null);
const modelist = ace.require("ace/ext/modelist");
const successList = document.querySelectorAll('.success');
const errorList = document.querySelectorAll('.error');
const error = document.getElementById('error') as HTMLDivElement;

const importsToWasm: i.Imports = {
  async fetch(url) {
    try {
      const response = await fetch(url);
      const hostResponse: i.Response = new HostResponse(response);
      return { tag: 'ok', val: hostResponse };
    } catch (e) {
      const err: i.ErrorFailure = { tag: 'Failure', val: e.toString() };
      return { tag: 'err', val: err };
    }
  },

  log: msg => console.log(msg),
  logErr: msg => console.error(msg),
};

async function init() {
  const importObject = {};
  let instance: WebAssembly.Instance;
  i.addImportsToImports(importObject, importsToWasm, name => instance.exports[name]);

  const wasm = new Exports();
  await wasm.instantiate(fetch('./witx_async_demo.wasm'), importObject);
  instance = wasm.instance;

  input.disabled = false;
  button.disabled = false;
  form.onsubmit = async function(e) {
    e.preventDefault();

    button.disabled = true;
    input.disabled = true;
    try {
      const tarball = await Tarball.fetch(wasm, input.value);
      if (tarball.tag == 'ok') {
        render(tarball.val);
      } else {
        renderError(tarball.val);
      }
    } finally {
      button.disabled = false;
      input.disabled = false;
    }
  };
}

function render(tarball: Tarball) {
  for (let element of successList)
    (element as HTMLElement).style.display = 'block';
  for (let element of errorList)
    (element as HTMLElement).style.display = 'none';

  const ul = document.createElement('ul') as HTMLUListElement;
  let i = 0;
  for (let file of tarball.files()) {
    const a = document.createElement('a') as HTMLAnchorElement;
    a.href = '#';
    let j = i;
    a.onclick = function() {
      editor.setValue(tarball.contents(j));
      editor.clearSelection();
      editor.resize();
      const selected = ul.querySelector('.selected');
      if (selected)
        selected.classList.remove('selected');
      a.classList.add("selected");
      const mode = modelist.getModeForPath(file).mode;
      editor.session.setMode(mode);
      return false;
    };
    const code = document.createElement('code');
    code.innerText = file;
    a.appendChild(code);
    const li = document.createElement('li');
    li.appendChild(a);
    ul.appendChild(li);
    i += 1;
  }

  while (allFiles.firstChild)
    allFiles.removeChild(allFiles.firstChild);
  allFiles.appendChild(ul);

  ul.querySelector('a').click();
}

function renderError(err: Error) {
  for (let element of errorList)
    (element as HTMLElement).style.display = 'block';
  for (let element of successList)
    (element as HTMLElement).style.display = 'none';

  error.innerText = err.val;
}

class HostResponse {
  response: Response;
  constructor(response) {
    this.response = response;
  }

  headers() {
    return new HostHeaders(this.response.headers);
  }

  async body() {
    const arrayBuffer = await this.response.arrayBuffer();
    return new Uint8Array(arrayBuffer);
  }

  status() {
    return this.response.status;
  }

  statusText() {
    return this.response.statusText;
  }
}

class HostHeaders {
  headers: Headers;
  constructor(headers) {
    this.headers = headers;
  }

  get(name) {
    const ret = this.headers.get(name);
    if (ret === null)
      return [];
    return ret.split(',');
  }

  entries() {
    const ret = [];
    for (let [name, value] of this.headers.entries())
      ret.push([name, value.split(',')]);
    return ret;
  }
}

init()
