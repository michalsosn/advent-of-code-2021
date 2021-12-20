use std::io;
use std::io::BufRead;


fn main() {
    let Input { replacements, image } = Input::read_stdin();
    println!("{}", render_image(&[replacements.to_vec()]));
    println!("{}", render_image(image.as_slice()));

    let enhanced_2_image = enhance_image(&image, &replacements, 2);
    println!("{}", render_image(enhanced_2_image.as_slice()));
    println!("ones_count = {}", count_ones(&enhanced_2_image));

    let enhanced_50_image = enhance_image(&image, &replacements, 50);
    println!("{}", render_image(enhanced_50_image.as_slice()));
    println!("ones_count = {}", count_ones(&enhanced_50_image));
}

#[derive(Debug)]
struct Input {
    replacements: [u8; 512],
    image: Vec<Vec<u8>>,
}

impl Input {
    fn read_stdin() -> Self {
        let stdin = io::stdin();
        let mut lines = stdin.lock().lines()
            .map(|l| l.expect("error: unable to read line").trim().to_string());

        let mut replacement_lines: Vec<Vec<u8>> = lines.by_ref()
            .take_while(|l| !l.is_empty())
            .map(|l| l.chars().map(|c| if c == '#' { 1 } else { 0 }).collect())
            .collect();
        let replacements: [u8; 512] = replacement_lines.pop().unwrap().try_into().unwrap();

        let image: Vec<Vec<u8>> = lines
            .map(|l| l.chars().map(|c| if c == '#' { 1 } else { 0 }).collect())
            .collect();

        Input {
            replacements,
            image
        }
    }
}

fn enhance_image(image: &Vec<Vec<u8>>, replacements: &[u8; 512], steps: u32) -> Vec<Vec<u8>> {
    let mut image = image.clone();
    let mut filler: u8 = 0;
    let mut neighbors: [[u8; 3]; 3] = [[0; 3]; 3];

    for _ in 0..steps {
        let height = image.len();
        let width = image[0].len();
        let mut new_image: Vec<Vec<u8>> = vec![vec![0; width + 2]; height + 2];

        for ty in 0..height+2 {
            for tx in 0..width+2 {
                for y in 0..=2 {
                    for x in 0..=2 {
                        let ny = (ty as i32) + (y as i32) - 2;
                        let nx = (tx as i32) + (x as i32) - 2;
                        neighbors[y][x] = if ny < 0 || ny >= height as i32 || nx < 0 || nx >= width as i32 {
                            filler
                        } else {
                            image[ny as usize][nx as usize]
                        };
                    }
                }
                let index = to_number(&neighbors);
                new_image[ty][tx] = replacements[index];
            }
        }

        filler = replacements[filler as usize * 9];
        image = new_image;
    }
    image
}

fn to_number(neighbors: &[[u8; 3]; 3]) -> usize {
    neighbors[0][0] as usize * 256 + neighbors[0][1] as usize * 128 + neighbors[0][2] as usize * 64
        + neighbors[1][0] as usize * 32 + neighbors[1][1] as usize * 16 + neighbors[1][2] as usize * 8
        + neighbors[2][0] as usize * 4 + neighbors[2][1] as usize * 2 + neighbors[2][2] as usize
}

fn render_image(image: &[Vec<u8>]) -> String {
    image.iter()
        .map(|r| r.into_iter().map(|&n| if n == 1 { '#' } else { '.' }).collect())
        .collect::<Vec<String>>()
        .join("\n")
}

fn count_ones(image: &[Vec<u8>]) -> u32 {
    image.into_iter().map(|row| row.into_iter().map(|&n| n as u32).sum::<u32>()).sum()
}
