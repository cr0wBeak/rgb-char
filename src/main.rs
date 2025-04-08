fn main() {
    let mut choice: String = String::new();
    loop {
        println!("Please choose an option: ");
        println!("1. Read message from image");
        println!("2. Create image from message");
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line!");
        choice = choice.trim().to_string();
        if choice == "1" || choice == "2" {
            break;
        } else {
            println!("Invalid choice!");
        }
    }
    match choice.parse::<u8>() {
        Ok(1) => read_message(),
        Ok(2) => create_image(),
        _ => println!("Invalid choice!"),
    }
}

fn read_message() {
    let mut image_path: String = String::new();
    println!("Enter the name of the image: ");
    std::io::stdin()
        .read_line(&mut image_path)
        .expect("Failed to read line!");
    // read values from image
    println!("Reading RGB values of image...");
    let rgb: Vec<u8> = rgb_from_image(&image_path);
    let mut message: Vec<char> = Vec::new();
    println!("Interpreting message...");
    for i in 0..rgb.len() {
        message.push(rgb[i] as char);
    }
    println!("Message read successfully!");
    // convert vector to string
    let message_string: String = message.into_iter().collect();
    let lines: Vec<&str> = message_string.split("\\n").collect();
    for line in lines {
        println!("{}", line.trim_end());
    }
}

fn create_image() {
    let mut msg: String = String::new();
    println!("Enter the message you want to encode. Please use \\n to create new lines.");
    std::io::stdin()
        .read_line(&mut msg)
        .expect("Failed to read line!");
    msg = msg.trim_end().to_string();
    // break message apart into lines
    println!("Calculating image dimensions...");
    let lines: Vec<&str> = msg.split("\\n").collect();
    let height: u32 = lines.len() as u32; // get height of image
    let width: u32 = calculate_image_width(&lines);
    // break lines into characters
    draw_image(height, width, lines);
    // draw_image() also saves the image.
}

fn draw_image(height: u32, width: u32, lines: Vec<&str>) {
    let mut img = image::RgbImage::new(width, height);
    for y in 0..height {
        let mut rgb_values: Vec<char> = lines[y as usize].chars().collect();
        for x in 0..width {
            // check number of rgb values remaining
            match rgb_values.len() {
                0 => break,
                1 => {
                    img.put_pixel(x, y, image::Rgb([rgb_values[0] as u8, 0, 0]));
                    rgb_values.remove(0);
                }
                2 => {
                    img.put_pixel(x, y, image::Rgb([rgb_values[0] as u8, rgb_values[1] as u8, 0]));
                    rgb_values.remove(0);
                    rgb_values.remove(0);
                }
                _ => {
                    img.put_pixel(x, y, image::Rgb([rgb_values[0] as u8, rgb_values[1] as u8, rgb_values[2] as u8]));
                    rgb_values.remove(0);
                    rgb_values.remove(0);
                    rgb_values.remove(0);
                }
            }
        }
    }
    let mut path: String = String::new();
    println!("Enter the name of the image. Do not forget the file extension!");
    std::io::stdin()
        .read_line(&mut path)
        .expect("Failed to read line!");
    path = path.trim_end().to_string();
    img.save("img/".to_owned() + &path).expect("Failed to save image!");
    println!("Image saved successfully!");
}

fn calculate_image_width(lines: &Vec<&str>) -> u32 {
    let mut max_width = 0;
    for line in lines {
        if line.len() > max_width {
            max_width = line.len() / 3;
            if line.len() % 3 != 0 {
                max_width += 1; // add an extra pixel to the width for safety
            }
        }
    }
    max_width as u32
}

fn rgb_from_image(path: &str) -> Vec<u8> {
    let img = image::open("img/".to_owned() + path.trim_end()).expect("Failed to open image");
    // convert to RGB format
    let rgb_image = img.to_rgb8();
    // get height and width of image
    //
    // if height or width is too large, process will not run
    let height: u32 = if rgb_image.height() <= std::u32::MAX {
        rgb_image.height()
    } else {
        println!("Height is too large!");
        return Vec::new();
    };
    let width: u32 = if rgb_image.width() <= std::u32::MAX {
        rgb_image.width()
    } else {
        println!("Width is too large!");
        return Vec::new();
    };
    let mut rgb_values: Vec<u8> = Vec::new();
    // add rgb values to vector
    for i in 0..height {
        for j in 0..width {
            let pixel = rgb_image.get_pixel(j, i);
            rgb_values.push(pixel[0]);
            rgb_values.push(pixel[1]);
            rgb_values.push(pixel[2]);
        }
        rgb_values.push(10u8); // newline character
    }
    rgb_values
}