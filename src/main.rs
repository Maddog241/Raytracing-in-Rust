mod aabb;
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod objects;
mod ray;
mod run;
mod texture;
mod utilities;
mod vec3;

extern crate libc;

mod ffi {
    extern "C" {
        pub fn clock() -> ::libc::clock_t;
    }
}

fn main() {
    let start = unsafe { ffi::clock() };
    let mode = 7;
    run::run(mode);

    let end = unsafe { ffi::clock() };

    eprintln!("cost: {} secs", (end - start) as f64 / 1000.0);
}
