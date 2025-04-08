fn main() {
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
    println!("Message: {}", message_string);
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