use rand::random;
use std::mem::MaybeUninit;

pub fn uninit<T>() -> T {
    unsafe { MaybeUninit::<T>::uninit().assume_init() }
}

pub fn map(
    value: f32, 
    min1: f32, 
    max1: f32, 
    min2: f32, 
    max2: f32,
) -> f32 {
    (value - min1) / (max1 - min1) * (max2 - min2) + min2
}

pub fn randomf(min: f32, max: f32) -> f32 {
    let mut r = random::<f32>();
    r *= max - min;
    r += min;
    r
}

pub fn randomuint(min: usize, max: usize) -> usize {
    let mut r = random::<f32>();
    r *= (max as f32) - (min as f32);
    r += min as f32;
    r as usize
}