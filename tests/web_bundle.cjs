// Regression for the upstream strict-mode quad_net registration error.
const fs = require('node:fs');
const vm = require('node:vm');
const assert = require('node:assert/strict');
const bundle = fs.readFileSync('web/mq_js_bundle.js', 'utf8');
new vm.Script(bundle); // Validate the complete staged bundle before isolating the plugin.
const start = bundle.lastIndexOf('function(){function i(){}');
assert.ok(start >= 0, 'Expected pinned Macroquad quad_net plugin');
const plugins = [];
const context = {miniquad_add_plugin: plugin => plugins.push(plugin)};
vm.runInNewContext('"use strict";(' + bundle.slice(start) + ');', context);
assert.equal(plugins.length, 1);
assert.equal(plugins[0].name, 'quad_net');
assert.equal(typeof plugins[0].register_plugin, 'function');
assert.equal('register_plugin' in context, false);
console.log('Staged quad_net plugin registers under strict mode without global leakage');
