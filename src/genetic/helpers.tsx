export const POPULATION_SIZE = 15

export const COLORS: string[] = ["#3E2A35", "#6B8F71", "#F0386B", "#7B8CDE", "#FFBF81"]
export const COLOR_TARGET  = COLORS[random()]
export const HEIGHT_TARGET = 500
export const HEIGHT_MAX    = 500
export const MUTATION_PROBABILITY = 0.5

export function random() {
    return Math.floor(Math.random() * COLORS.length)
}

// https://stackoverflow.com/a/49434653
export function boxmuller(): number {
    let u = 0, v = 0;
    while(u === 0) u = Math.random(); //Converting [0,1) to (0,1)
    while(v === 0) v = Math.random();
    let num = Math.sqrt( -2.0 * Math.log( u ) ) * Math.cos( 2.0 * Math.PI * v );
    num = num / 10.0 + 0.5; // Translate to 0 -> 1
    if (num > 1 || num < 0) return boxmuller() // resample between 0 and 1
    return num
}

// From: https://stackoverflow.com/a/5624139
export function rgbToHex(r: number, g: number, b: number): string {
    return "#" + (1 << 24 | r << 16 | g << 8 | b).toString(16).slice(1);
}

// From: https://stackoverflow.com/a/5624139
export function hexToRgb(hex: string): [number, number, number] {
    var result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
    return result ? [
      parseInt(result[1], 16),
      parseInt(result[2], 16),
      parseInt(result[3], 16)
     ] : [0, 0, 0];
}

export function randomColor(): string {
    const hex = COLORS[random()]
    const [r, g, b] = hexToRgb(hex)
    let rd = r + Math.ceil(Math.random() * 256)
    let gd = g + Math.ceil(Math.random() * 256)
    let bd = b + Math.ceil(Math.random() * 256)
    return rgbToHex(rd, gd, bd)
}

// ? TYPES

export type IGenotype = {
    color:    string,
    height:   number,
    fitness?: number
}