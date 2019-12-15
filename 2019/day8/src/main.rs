use std::{fs};

fn main()
{
    let filecontents = fs::read_to_string("input.txt").unwrap();

    let mut min_zeroes= 150;
    let mut min_layer = 0;

    for layer_start in (0..filecontents.len()).step_by(150) {
        let layer = String::from(&filecontents[layer_start..layer_start+150]);
        let zeroes = layer.chars().fold(0,|acc, c| if c == '0' { acc + 1 } else { acc } );

        if zeroes < min_zeroes {
            min_zeroes=zeroes;
            min_layer=layer_start;
        }
    }

    let layer = String::from(&filecontents[min_layer..min_layer+150]);
    let ones = layer.chars().fold(0,|acc, c| if c == '1' { acc + 1 } else { acc } );
    let twos = layer.chars().fold(0,|acc, c| if c == '2' { acc + 1 } else { acc } );

    println!("{} * {} = {} \n", ones, twos, ones*twos);

    let mut image =  Vec::from(&filecontents[filecontents.len()-150..filecontents.len()]);
    for layer_start in (0..filecontents.len()).step_by(150) {
        let layer = Vec::from(&filecontents[filecontents.len()-layer_start-150..filecontents.len()-layer_start]);
        for i in 0..150 {
            match layer[i] as char {
                '1' => image[i] = '1' as u8,
                '0' => image[i] = ' ' as u8,
                _ => {},
            }
        }
    }

    let image = String::from_utf8(image).unwrap();
    for image_line in (0..image.len()).step_by(25) {
        println!("{}", &image[image_line..image_line+25]);
    }
}
