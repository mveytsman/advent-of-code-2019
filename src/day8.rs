fn get_layers(input: &str, width: usize, height: usize) -> Vec<Vec<u32>> {
    let digits: Vec<u32> = input
        .chars()
        .map(|char| char.to_digit(10).unwrap())
        .collect();

    digits
        .chunks(width * height)
        .map(|layer| layer.to_vec())
        .collect()
}

pub fn day8_1(input: &str, width: usize, height: usize) -> usize {
    let mut layers = get_layers(input, width, height);
    layers.sort_by(|a, b| count_digit(a, 0).cmp(&count_digit(b, 0)));

    count_digit(&layers[0], 1) * count_digit(&layers[0], 2)
}

pub fn collapse_layers(input: &str, width: usize, height: usize) -> Vec<u32> {
    let layers = get_layers(input, width, height);
    let mut visible_layer = vec![0; width * height];
    for i in 0..(width * height) {
        for layer in &layers {
            if layer[i] == 2 {
                continue;
            } else {
                visible_layer[i] = layer[i];
                break;
            }
        }
    }
    print_layer(&visible_layer, width, height);
    visible_layer
}

fn print_layer(layer: &Vec<u32>, width: usize, height: usize) {
    for row in layer.chunks(width) {
        for i in row {
            if i == &0 {
                print!(" ");
            } else {
                print!("x");
            }
        }
        println!();
    }
}

fn count_digit(layer: &Vec<u32>, digit: u32) -> usize {
    layer.iter().filter(|d| **d == digit).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day8() {
        assert_eq!(1, day8_1("123456789012", 3, 2));
        assert_eq!(vec![0, 1, 1, 0], collapse_layers("0222112222120000", 2, 2));
    }

}
