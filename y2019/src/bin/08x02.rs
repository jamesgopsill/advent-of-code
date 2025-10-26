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
    let mut image: Vec<u32> = vec![2; layer_length];

    for layer in layers {
        for (i, val) in layer.iter().enumerate() {
            if image[i] == 2 {
                image[i] = *val;
            }
        }
    }

    // Print image
    let rows = image.chunks_exact(25);
    for row in rows {
        println!("{:?}", row);
    }

    "".into()
}
