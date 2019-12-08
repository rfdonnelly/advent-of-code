use std::fs;
use std::io;

use itertools::Chunk;
use itertools::Itertools;

pub(crate) fn main() -> io::Result<()> {
    let (part1, part2) = day8();

    println!("day8::part1: {}", part1);
    println!("day8::part2: {}", part2);

    Ok(())
}

fn day8() -> (usize, usize) {
    let input = fs::read_to_string("input/8").unwrap();

    (part1(&input), part2(&input))
}

type Pixel = char;
type Layer = Vec<Pixel>;
type LayerRef<'a> = &'a [Pixel];

fn part1(s: &str) -> usize {
    let x = 25;
    let y = 6;

    let pixels_per_layer = x * y;

    let layers = chunks(s, pixels_per_layer);
    let layer = layers
        .iter()
        .min_by_key(|layer| count_pixel(layer, '0'))
        .unwrap();

    count_pixel(&layer, '1') * count_pixel(&layer, '2')
}

fn count_pixel(layer: LayerRef, pixel: Pixel) -> usize {
    layer
        .iter()
        .filter(|&&x| x == pixel)
        .count()
}

fn chunks(s: &str, pixels_per_layer: usize) -> Vec<Layer> {
    let mut chunks = Vec::new();
    let mut count = 0;
    let mut layer = Vec::new();

    for c in s.chars() {
        layer.push(c);

        count += 1;
        if count == pixels_per_layer {
            count = 0;
            chunks.push(layer);
            layer = Vec::new();
        }
    }

    chunks
}

fn part2(s: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day8() {
        assert_eq!(day8(), (2159, 0))
    }
}
