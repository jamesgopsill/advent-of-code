fn main() {
    let input = include_str!("../../../puzzle_data/2019/08.txt");
    let out = invoke(input);
    println!("{out}");
    // bench(invoke, input);
}

fn invoke(input: &str) -> String {
    let input = input.trim();

    let pixels: Vec<u32> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let layer_length = 25 * 6;
    let layers = pixels.chunks_exact(layer_length);
    let mut min_zero_count = usize::MAX;
    let mut best_layer_sum = 0;
    for layer in layers {
        let zeros = layer.iter().filter(|v| **v == 0).count();
        if zeros < min_zero_count {
            min_zero_count = zeros;
            let ones = layer.iter().filter(|v| **v == 1).count();
            let twos = layer.iter().filter(|v| **v == 2).count();
            best_layer_sum = ones * twos;
        }
    }

    format!("{}", best_layer_sum)
}
