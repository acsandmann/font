import { test } from 'uvu';
import * as assert from 'uvu/assert';
import { readFile } from 'fs/promises';
import { Font, Layout } from '../dist/node.mjs';

const raw = await readFile(new URL('./font.ttf', import.meta.url));
const font = new Font(12, raw);

test('constructor', () => {
    const l = new Layout();
    assert.instance(l, Layout);
    assert.type(l.ptr, 'number');
});

test('layout#free', () => {
    const l = new Layout(12, raw);

    l.free();
});

test('layout#clear', () => {
    const l = new Layout();

    l.append(font, 'hi');

    l.clear();
    assert.equal(l.lines(), 0);
});

test('layout#lines', () => {
    const l = new Layout();

    l.append(font, 'hi\n');
    l.append(font, 'bye');

    assert.equal(l.lines(), 2);
});

test('layout#reset', () => {
    const l = new Layout();

    l.reset({ max_width: 10, max_height: 10 });
    // Assuming options are reset correctly
});

test('layout#append', () => {
    const l = new Layout();
    l.append(font, 'Hello', { scale: 1, r: 255, g: 255, b: 255 });
    assert.equal(l.lines(), 1);
});

test('layout#rasterize', () => {
    const l = new Layout();
    l.append(font, 'hello');
    const r = l.rasterize(255, 255, 255);
    
    assert.instance(r.buffer, Uint8Array);
    assert.equal(r.buffer.length, r.height * r.width * 4);
    assert.equal(r.width, 39);
    assert.equal(r.height, 16);
});


test.run();