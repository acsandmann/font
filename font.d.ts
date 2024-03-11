interface metrics {
    xmin: number;
    ymin: number;
    width: number;
    height: number;
    advance_width: number;
    advance_height: number;
    bounds: bounds;
}

interface bounds {
    xmin: number;
    ymin: number;
    width: number;
    height: number;
}

interface font_rasterized {
    buffer: Uint8Array;
    metrics: metrics;
}

interface layout_options {
    max_width?: number;
    max_height?: number;
    wrap_style?: 'word' | 'letter';
    vertical_align?: 'top' | 'middle' | 'bottom';
    wrap_hard_breaks?: boolean;
    horizontal_align?: 'left' | 'right' | 'center';
}

interface append_options {
    scale?: number;
    r?: number;
    g?: number;
    b?: number;
}

interface layout_rasterized {
    buffer: Uint8Array;
    width: number;
    height: number;
}

export class Font {
    scale: number;

    /** create a new font instance **/
    constructor(scale: number, buffer: Uint8Array);

    /** free font instance **/
    free(): void;

    /** check if the font has a glyph for a specific character **/
    has(char: string): boolean;

    /** see the metrics of a rendered character **/
    metrics(char: string, scale?: number): metrics;

    /** render a character to a bitmap **/
    rasterize(char: string, scale?: number): font_rasterized;
}

export class Layout {
    /** create a new layout instance **/
    constructor();

    /** clear all glyph's off of a layout **/
    clear(): void;

    /** how many lines this layout has **/
    lines(): number;

    /** reset the layout's options **/
    reset(options?: layout_options): void;

    /** append text to a layout **/
    append(font: Font, text: string, init: append_options): void;

    /** render a layout to a bitmap **/
    rasterize(r: number, g: number, b: number): layout_rasterized;
}