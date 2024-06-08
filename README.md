# @asandmann/font

This wasm wrapping of [fontdue](https://github.com/mooman219/fontdue) provides tools for handling font rendering, layout creation, and text rasterization in bitmap format. 

## Installation

```bash
npm install @asandmann/font
```

## Example

```typescript
import { Font, Layout } from '@asandmann/font';

// Create a font instance
const fontBuffer = new Uint8Array(); // Load your font buffer
const font = new Font(1.0, fontBuffer);

// Check if the font has a glyph for 'A'
if (font.has('A')) {
    // Get metrics for 'A'
    const metrics = font.metrics('A');

    // Rasterize 'A'
    const rasterizedChar = font.rasterize('A');
}

// Create a layout instance
const layout = new Layout();

// Set layout options
layout.reset({
    max_width: 500,
    wrap_style: 'word',
    vertical_align: 'middle',
    horizontal_align: 'center'
});

// Append text to the layout
layout.append(font, 'Hello, World!', { scale: 1, r: 255, g: 255, b: 255 });

// Render the layout
const rasterizedLayout = layout.rasterize(0, 0, 0);

// Clean up
font.free();
layout.clear();
```

This example demonstrates how to create a font instance, check for glyph availability, get character metrics, rasterize a character, create a layout, set layout options, append text, and render the layout to a bitmap.

## Usage

### Font Class

#### Constructor

```typescript
const font = new Font(scale: number, buffer: Uint8Array);
```

Creates a new font instance.

#### Methods

- **free(): void**  
  Frees the font instance.

- **has(char: string): boolean**  
  Checks if the font has a glyph for a specific character.

- **metrics(char: string, scale?: number): metrics**  
  Gets the metrics of a rendered character.

- **rasterize(char: string, scale?: number): font_rasterized**  
  Renders a character to a bitmap.

### Layout Class

#### Constructor

```typescript
const layout = new Layout();
```

Creates a new layout instance.

#### Methods

- **clear(): void**  
  Clears all glyphs from the layout.

- **lines(): number**  
  Returns the number of lines in the layout.

- **reset(options?: layout_options): void**  
  Resets the layout's options.

- **append(font: Font, text: string, init: append_options): void**  
  Appends text to the layout.

- **rasterize(r: number, g: number, b: number): layout_rasterized**  
  Renders the layout to a bitmap.
