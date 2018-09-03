extern crate stb_image;

use std::path::Path;
use std::env;

use stb_image::image;
use stb_image::image::LoadResult;
use stb_image::image::LoadResult::*;

fn main()
{
    let args: Vec<String> = env::args().collect();

    if args.len() != 2
    {
        println!("Please only enter the file path");
        return;
    }

    let path = Path::new(&args[0]).parent().unwrap().join(&args[1]);
    if !&path.exists()
    {
        println!("The image doesn't exist");
        return;
    }

	let image = match stb_image::image::load(&path)
    {
		LoadResult::ImageU8(data) => data,
		LoadResult::ImageF32(..) => panic!("HDR images are not supported"),
		LoadResult::Error(string) => panic!(string)
    };

    let mut result: String = String::new();

    // Hard-coded atm
    let name = "raven";

    let image_width = &format!("#define {}_WIDTH: {}\n", &name, &image.width.to_string());
    let image_height = &format!("#define {}_HEIGHT: {}\n", &name, &image.height.to_string());
    result.push_str(image_width);
    result.push_str(image_height);

    let mut image_raw_data: String = String::new();

    // Convert to (R, G, B) to GBA format, not working atm
    for pixel in image.data
    {
        let pixel_data = &format!("{}, ", &pixel.to_string());
        image_raw_data.push_str(pixel_data);
    }

    // Fix formatting: it's hideous
    let image_data = &format!("\nconst u16 {}_DATA[] = 
{{
    {}
}};", &name, &image_raw_data);
    result.push_str(image_data);

    // Save with namefile.h

    println!("{}", result);
}
