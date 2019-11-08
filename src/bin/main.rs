extern crate image;

use image::GenericImageView;
use image::ImageBuffer;
use image::Pixel;
use image::Rgba;
use image::Nearest;

use std::collections::HashMap;
use std::env;
use std::process;

use trans_rs::{Recipe, RecipeBuilder, RgbaDiff};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    // get first filename
    let base_filename = args.get(0).unwrap_or_else(|| {
        eprintln!("You have to provide first filename.");
        process::exit(1);
    });

    // get second filename
    let trans_filename = args.get(1).unwrap_or_else(|| {
        eprintln!("You have to provide second filename.");
        process::exit(1);
    });

    // if user provides filename for result, save it
    // else use result.jpg
    let result_filename = args.get(2).cloned().unwrap_or(String::from("result"));
    let extension = args.get(3).cloned().unwrap_or(String::from("jpg"));

    // Opening first image
    let img1 = image::open(base_filename).unwrap_or_else(|_| {
        eprintln!("Cannot open '{}' file.", base_filename);
        process::exit(1);
    });

    // Opening second image
    let mut img2 = image::open(trans_filename).unwrap_or_else(|_| {
        eprintln!("Cannot open '{}' file.", base_filename);
        process::exit(1);
    });

    let (imgx, imgy) = img1.dimensions();
    img2 = img2.resize(imgx, imgy, Nearest);

    // TODO: Convert first or second image's dimensions
    assert_eq!(img1.dimensions(), img2.dimensions());

    let steps = 10;
    let mut recipes = HashMap::new();
    // Calculate recipes
    println!("Calculating recipes...");
    for ((x, y, p1), (_, _, p2)) in img1.pixels().zip(img2.pixels()) {
        let recipe = RecipeBuilder::new()
            .steps(steps)
            .minuend(p2)
            .subtrahend(p1)
            .build();
        recipes.insert((x, y), recipe);
    }
    println!("Done calculating recipes.");

    println!("Calculating dimensions.");
    let (imgx, imgy) = img1.dimensions();
    println!("Start buffering new image.");

    // TODO: iterate over raw pixels to test it
    let mut imgbuff: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(imgx, imgy);
    for (x, y, p) in imgbuff.enumerate_pixels_mut() {
        *p = img1.get_pixel(x, y);
    }
    println!("Done buffering new image.");

    for i in 0..steps {
        for (x, y, p) in imgbuff.enumerate_pixels_mut() {
            if let Some(recipe) = recipes.get_mut(&(x, y)) {
                if let Some(diff) = recipe.next() {
                    let new_pixel = diff.apply_to_pixel(*p);
                    *p = new_pixel;
                }
            }
        }
        let filename = format!("{}-{}.{}", result_filename, i, extension);
        println!("Saving to {} ...", &filename);
        imgbuff.save(filename).unwrap_or_else(|_| {
            eprintln!("Cannot save result.");
            process::exit(1);
        });
    }
    img2.save(format!("{}-{}.{}", result_filename, steps, extension)).unwrap_or_else(|_| {
        eprintln!("Cannot save result.");
        process::exit(1);
    });
    println!("Completed.");
}
