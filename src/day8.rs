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

fn count_digit(layer: &Vec<u32>, digit: u32) -> usize {
    layer.iter().filter(|d| **d == digit).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day8() {
        assert_eq!(1, day8_1("123456789012", 3, 2));
    }

}
