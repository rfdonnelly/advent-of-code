use std::fs;
use std::io;

use itertools::Itertools;

pub(crate) fn main() -> io::Result<()> {
    let (part1, part2) = day8();

    println!("day8::part1: {}", part1);
    println!("day8::part2:\n{}", part2);

    Ok(())
}

fn day8() -> (usize, String) {
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

fn part2(s: &str) -> String {
    let width = 25;
    let height = 6;

    render_input(s, width, height)
}

fn count_pixel(layer: LayerRef, pixel: Pixel) -> usize {
    layer
        .iter()
        .filter(|&&x| x == pixel)
        .count()
}

fn chunks(s: &str, chunk_size: usize) -> Vec<Layer> {
    let mut chunks = Vec::new();
    let mut count = 0;
    let mut layer = Vec::new();

    for c in s.chars() {
        layer.push(c);

        count += 1;
        if count == chunk_size {
            count = 0;
            chunks.push(layer);
            layer = Vec::new();
        }
    }

    chunks
}

fn render_input(s: &str, width: usize, height: usize) -> String {
    let pixels_per_layer = width * height;

    let layers = chunks(s, pixels_per_layer);
    let image = flatten(layers);

    render(&image, width)
}

fn flatten(layers: Vec<Layer>) -> Layer {
    let layers = {
        let mut layers = layers;
        layers.reverse();
        layers
    };

    let mut image = vec![' '; layers[0].len()];

    for layer in layers {
        for (i, &pixel) in layer.iter().enumerate() {
            if pixel != '2' {
                image[i] = pixel;
            }
        }
    }

    image
}

fn render(image: LayerRef, length: usize) -> String {
    let image: String = image.iter().collect();
    let rows = chunks(&image, length);

    rows
        .iter()
        .map(|row| row.iter().map(render_pixel).collect::<String>())
        .join("\n")
}

fn render_pixel(pixel: &Pixel) -> char {
    if pixel == &'1' {
        '#'
    } else {
        ' '
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    #[test]
    fn test_part2_example1() {
        let input = "0222112222120000";
        let expected =
            " #\n\
             # ";

        assert_eq!(render_input(input, 2, 2), expected.to_string());
    }

    #[test]
    fn test_part2() {
        let input = "0022112222120000";
        let expected =
            "  \n\
             # ";

        assert_eq!(render_input(input, 2, 2), expected.to_string());
    }

    #[test]
    fn test_day8() {
        let part2 = indoc!("
             ##    ## #### #  # ###  
            #  #    #    # #  # #  # 
            #       #   #  #### #  # 
            #       #  #   #  # ###  
            #  # #  # #    #  # # #  
             ##   ##  #### #  # #  # ");
        assert_eq!(day8(), (2159, part2.into()))
    }
}
