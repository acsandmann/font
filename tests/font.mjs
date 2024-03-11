import { test } from 'uvu';
import * as assert from 'uvu/assert';
import { readFile } from 'fs/promises';
import { Font } from '../dist/node.mjs';

const raw = await readFile(new URL('./font.ttf', import.meta.url));
const alphabet = Array.from({ length: 26 }, (_, i) => String.fromCharCode(97 + i));

test('constructor', () => {
    const f = new Font(12, raw);
    assert.equal(f.scale, 12);
    assert.type(f.ptr, 'number')
});

test('font#free', () => {
    const f = new Font(12, raw);

    f.free();
});

test('font#has', () => {
    const f = new Font(12, raw);
    for (let i = 0; i < alphabet.length; i++) assert.is.not(f.has(alphabet[i]), 0)
});

test('font#metrics', () => {
    const f = new Font(12, raw);
    const metrics = f.metrics('A');
    
    assert.instance(metrics, Object);
    assert.equal(metrics.xmin, 0);
    assert.equal(metrics.ymin, 0);
    assert.equal(metrics.width, 7);
    assert.equal(metrics.height, 9);
    assert.equal(metrics.advance_width, 7.4179688);
    assert.equal(metrics.advance_height, 0);

    assert.instance(metrics.bounds, Object);
    assert.equal(metrics.bounds.xmin, 0.43359375);
    assert.equal(metrics.bounds.ymin, 0);
    assert.equal(metrics.bounds.width, 6.5507813);
    assert.equal(metrics.bounds.height, 8.455078);
});

test('font#rasterize', () => {
    const f = new Font(12, raw);
    const r = f.rasterize('A');
    
    assert.equal(r.buffer.length, 63);

    const metrics = r.metrics;

    assert.instance(metrics, Object);
    assert.equal(metrics.xmin, 0);
    assert.equal(metrics.ymin, 0);
    assert.equal(metrics.width, 7);
    assert.equal(metrics.height, 9);
    assert.equal(metrics.advance_width, 7.4179688);
    assert.equal(metrics.advance_height, 0);

    assert.instance(metrics.bounds, Object);
    assert.equal(metrics.bounds.xmin, 0.43359375);
    assert.equal(metrics.bounds.ymin, 0);
    assert.equal(metrics.bounds.width, 6.5507813);
    assert.equal(metrics.bounds.height, 8.455078);
});


test.run();