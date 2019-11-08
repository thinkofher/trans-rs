extern crate image;
use image::{Pixel, Rgb, Rgba};

use std::iter::FromIterator;
use std::ops::{Add, Div, Rem, Sub};

#[derive(Copy, Clone)]
pub struct RgbaDiff(i32, i32, i32, i32);

impl RgbaDiff {
    pub fn new(dr: i32, dg: i32, db: i32, da: i32) -> RgbaDiff {
        RgbaDiff(dr, dg, db, da)
    }

    pub fn from_pixel(p: Rgba<u8>) -> RgbaDiff {
        let channels = pixel_to_vec(p);
        RgbaDiff::from_vec([channels[0], channels[1], channels[2], channels[3]])
    }

    pub fn from_vec(slice: [i32; 4]) -> RgbaDiff {
        RgbaDiff(slice[0], slice[1], slice[2], slice[3])
    }

    pub fn channels(&self) -> Vec<i32> {
        vec![self.0, self.1, self.2, self.3]
    }

    pub fn apply_to_pixel(&self, p: Rgba<u8>) -> Rgba<u8> {
        let pixel_vec = pixel_to_vec(p);
        let result_vec = add_vecs(self.channels(), pixel_vec);
        Rgba([
            result_vec[0] as u8,
            result_vec[1] as u8,
            result_vec[2] as u8,
            result_vec[3] as u8,
        ])
    }
}

pub struct Recipe {
    recipe: Vec<RgbaDiff>,
    step: usize,
}

impl Recipe {
    pub fn new() -> Recipe {
        Recipe {
            recipe: Vec::new(),
            step: 0,
        }
    }

    pub fn from_vec(v: Vec<RgbaDiff>) -> Recipe {
        Recipe { recipe: v, step: 0 }
    }

    pub fn max_step(&self) -> usize {
        self.recipe.len()
    }
}

impl Iterator for Recipe {
    type Item = RgbaDiff;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ans) = self.recipe.get(self.step) {
            self.step += 1;
            Some(*ans)
        } else {
            None
        }
    }
}

pub struct RecipeBuilder {
    minuend: Option<Rgba<u8>>,
    subtrahend: Option<Rgba<u8>>,
    steps: usize,
}

impl RecipeBuilder {
    pub fn new() -> RecipeBuilder {
        RecipeBuilder {
            minuend: None,
            subtrahend: None,
            steps: 0,
        }
    }

    pub fn build(&mut self) -> Recipe {
        if let Some(vec) = self.calc_recipe_vec() {
            Recipe::from_vec(vec)
        } else {
            panic!(
                "You have to provide number of steps,
 minuend and subtrahend in order to build Recipe."
            );
        }
    }

    pub fn steps(mut self, s: usize) -> RecipeBuilder {
        self.steps = s;
        self
    }

    pub fn minuend(mut self, pixel: Rgba<u8>) -> RecipeBuilder {
        self.minuend = Some(pixel);
        self
    }

    pub fn subtrahend(mut self, pixel: Rgba<u8>) -> RecipeBuilder {
        self.subtrahend = Some(pixel);
        self
    }

    fn calc_recipe_vec(&mut self) -> Option<Vec<RgbaDiff>> {
        if self.steps == 0 || self.minuend == None || self.subtrahend == None {
            return None;
        }

        let mut result = Vec::with_capacity(self.steps);

        let minuend_vec = pixel_to_vec(self.minuend.unwrap());
        let subtrahend_vec = pixel_to_vec(self.subtrahend.unwrap());

        let vec_to_push = divide_vec(sub_vecs(minuend_vec, subtrahend_vec), self.steps as i32);

        for _ in 0..self.steps {
            let diff_to_push = RgbaDiff::from_vec([
                vec_to_push[0],
                vec_to_push[1],
                vec_to_push[2],
                vec_to_push[3],
            ]);
            result.push(diff_to_push);
        }

        // TODO: this code generates some bad pixels at last step
        // let last_element = result.last_mut().unwrap();
        // let vec_to_add = add_vecs(
        //     vec_to_push.clone(),
        //     mod_vec(vec_to_push, self.steps as i32),
        // );
        // *last_element =
        //     RgbaDiff::from_vec([vec_to_add[0], vec_to_add[1], vec_to_add[2], vec_to_add[3]]);
        Some(result)
    }
}

fn pixel_to_vec(pixel: Rgba<u8>) -> Vec<i32> {
    pixel.channels().iter().map(|c| *c as i32).collect()
}

fn substract_pixels_to_vec(a: image::Rgb<u8>, b: image::Rgb<u8>) -> Vec<i32> {
    let vec_a: Vec<i32> = a.channels().iter().map(|c| *c as i32).collect();
    let vec_b = b.channels().iter().map(|c| *c as i32).collect();
    sub_vecs(vec_a, vec_b)
}

fn divide_vec<T>(v: Vec<T>, n: T) -> Vec<T>
where
    T: Div + Copy + Clone,
    Vec<T>: FromIterator<<T as Div>::Output>,
{
    v.iter().map(|value| *value / n).collect()
}

fn add_vecs<T>(a: Vec<T>, b: Vec<T>) -> Vec<T>
where
    T: Add + Copy + Clone,
    Vec<T>: FromIterator<<T as Add>::Output>,
{
    a.iter().zip(b.iter()).map(|(a, b)| *a + *b).collect()
}

fn mod_vec<T>(v: Vec<T>, n: T) -> Vec<T>
where
    T: Rem + Copy + Clone,
    Vec<T>: FromIterator<<T as Rem>::Output>,
{
    v.iter().map(|value| *value % n).collect()
}

fn sub_vecs<T>(a: Vec<T>, b: Vec<T>) -> Vec<T>
where
    T: Sub + Copy + Clone,
    Vec<T>: FromIterator<<T as Sub>::Output>,
{
    a.iter().zip(b.iter()).map(|(a, b)| *a - *b).collect()
}

fn divide_pixel(p: image::Rgb<u8>, n: u8) -> Vec<i32> {
    let vec = p.channels().iter().map(|c| *c as i32).collect();
    divide_vec(vec, n as i32)
}

fn mod_pixel(p: image::Rgb<u8>, n: u8) -> Vec<i32> {
    let vec = p.channels().iter().map(|c| *c as i32).collect();
    mod_vec(vec, n as i32)
}

// TODO: Create tests for Rgba pixels
#[cfg(test)]
mod tests {
    use super::*;
    use image::Rgb;

    #[test]
    fn mod_pixel_five() {
        assert_eq!(mod_pixel(Rgb([25, 40, 17]), 5), vec![0, 0, 2]);
    }

    #[test]
    fn mod_pixel_three() {
        assert_eq!(mod_pixel(Rgb([33, 20, 83]), 3), vec![0, 2, 2]);
    }

    #[test]
    fn sub_vecs_test() {
        let vec1 = vec![23, 34, 12];
        let vec2 = vec![15, 15, 2];
        assert_eq!(sub_vecs(vec1, vec2), vec![8, 19, 10]);
    }

    #[test]
    fn sub_vec_test_negative() {
        let vec1 = vec![33, 15, 3];
        let vec2 = vec![-10, 20, 10];
        assert_eq!(sub_vecs(vec1, vec2), vec![43, -5, -7]);
    }

    #[test]
    fn divide_vec_by_two() {
        let test_vec = vec![13, 10, 15];
        assert_eq!(divide_vec(test_vec, 2), vec![6, 5, 7]);
    }

    #[test]
    fn divide_vec_by_five() {
        let test_vec = vec![25, 100, 19];
        assert_eq!(divide_vec(test_vec, 5), vec![5, 20, 3]);
    }

    #[test]
    fn substract_pixel_one() {
        let p1 = Rgb([255, 50, 100]);
        let p2 = Rgb([244, 90, 75]);
        assert_eq!(substract_pixels_to_vec(p1, p2), vec![11, -40, 25]);
    }
}
