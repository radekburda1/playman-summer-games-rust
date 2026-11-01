const {readFileSync} = require('node:fs');
const vm = require('node:vm');
const assert = require('node:assert/strict');
let plugin;
const data = new Map();
const context = {
    miniquad_add_plugin: p => plugin = p,
    localStorage: {getItem: k => data.get(k) ?? null, setItem: (k,v) => data.set(k,v)},
    TextDecoder, Uint8Array,
    wasm_memory: {buffer: new ArrayBuffer(256)},
};
vm.runInNewContext(readFileSync('web/records.js','utf8'),context);
function connect() { const imports = {env:{}}; plugin.register_plugin(imports); return imports.env; }
let api = connect();
assert.equal(api.playman_record_load(0),-1);
function save(text) {const bytes = new TextEncoder().encode(text);new Uint8Array(context.wasm_memory.buffer).set(bytes);return api.playman_record_save(0,bytes.length);}
assert.equal(save('1237\n-\n1450\n500\n7000'),1);
api = connect(); // A newly initialized game reads persisted slots.
assert.deepEqual(Array.from({length:5},(_,i)=>api.playman_record_load(i)),[1237,-1,1450,500,7000]);
assert.equal(save('-\n-\n-\n-\n-'),1);
assert.equal(api.playman_record_load(0),-1);
data.set('playman-records-v1','garbage\n-3\n2147483648\n0');
assert.deepEqual(Array.from({length:5},(_,i)=>api.playman_record_load(i)),[-1,-1,-1,0,-1]);
context.localStorage.setItem = () => {throw Error('storage blocked');};
assert.equal(save('1000'),0);
console.log('Web record persistence, reload, reset, malformed data and storage failures passed');
