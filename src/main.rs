#![allow(dead_code)]

mod materials;
mod shapes;
mod utils;

use std::sync::mpsc::Receiver;

use crate::{
    materials::metal::Metal,
    shapes::{list::HittableList, plane::Plane},
    utils::{
        args::Args,
        camera::Camera,
        helpers::{
            clear, compute_chunk, parse_aspect_ratio, random_spheres, split_evenly, write_to_file,
        },
        hittable,
        result::Res,
        threads::{job::Job, pool::ThreadPool},
        vec::{Color, Point3, Vec3},
    },
};
use clap::Parser;
use indicatif::{HumanBytes, HumanDuration, ProgressBar, ProgressStyle};

fn main() {
    let args = Args::parse();
    if args.clear {
        clear();
    }
    let cpus = args.threads;
    let image_width = args.width;
    let aspect_ratio = parse_aspect_ratio(&args.aspect_ratio).unwrap_or({
        if args.width > 0 && args.height.is_some() {
            args.width as f32 / args.height.unwrap() as f32
        } else {
            16.0 / 9.0
        }
    });
    let image_height = if let Some(height) = args.height {
        height
    } else {
        (image_width as f32 / aspect_ratio) as i32
    };
    let samples_per_pixel = args.samples;

    let camera = Box::new(Camera::new(
        Vec3::new(-5.0, 5.0, 5.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        args.fov,
        aspect_ratio,
    ));
    let random: Vec<Box<dyn hittable::Hittable>> = random_spheres(10);
    let world = Box::new(HittableList::new(
        vec![
            Box::new(Plane::new(
                Point3::new(0.0, -5.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Box::new(Metal::new(Color::new(0.5, 0.5, 0.5), 0.0)),
            )) as Box<dyn hittable::Hittable>,
            // Box::new(Triangle::new(
            //     Point3::new(-5.0, 0.0, -5.0),
            //     Point3::new(5.0, 0.0, -5.0),
            //     Point3::new(0.0, 0.0, 5.0),
            //     Vec3::new(0.0, 1.0, 0.0),
            //     Box::new(Metal::new(Color::new(0.5, 0.5, 0.5), 0.0)),
            // )) as Box<dyn hittable::Hittable>,
        ]
        .into_iter()
        .chain(random.into_iter())
        .collect(),
    ));

    let mut image_buffer: Vec<Vec<Color>> =
        vec![vec![Color::new(0.0, 0.0, 0.0); image_width as usize]; image_height as usize];

    let start = std::time::Instant::now();

    let thread_pool =
        ThreadPool::<(Vec<i32>, Box<Camera>, Box<HittableList>, i32, i32, i32), Res>::new(cpus);

    let (rows_chunks, rows_per_chunk) =
        split_evenly((0..image_height).collect::<Vec<i32>>(), args.jobs);

    let mut rxs: Vec<Receiver<Res>> = Vec::with_capacity(cpus);
    let progress_bar = ProgressBar::new(image_height as u64).with_style(
        ProgressStyle::default_bar()
            .template("{spinner:.blue} [{elapsed_precise}] {wide_bar:40.cyan/blue} {percent}% {eta_precise}")
            .unwrap()
            .progress_chars("█▉▊▋▌▍▎▏ "),
    );
    progress_bar.println(format!(
        "ℹ️  Rendering {}x{} ({} spp) | {} jobs for {} threads ({} rows/chunk)",
        image_width, image_height, samples_per_pixel, args.jobs, cpus, rows_per_chunk[0]
    ));

    for chunk in rows_chunks {
        let camera = camera.clone();
        let world = world.clone();
        let (job, rx) = Job::with_result_sink(
            move |args| compute_chunk(args),
            (
                chunk,
                camera,
                world,
                image_width,
                image_height,
                samples_per_pixel,
            ),
        );
        thread_pool.schedule(job).expect("Failed to schedule job");
        rxs.push(rx);
    }
    progress_bar.println("🚚 Dispatched jobs to threads");

    for rx in rxs {
        let result = rx.recv().unwrap();
        for (i, buffer) in result.buffers.iter().enumerate() {
            let index = (result.start + i as i32) as usize;
            image_buffer[index] = buffer.clone();
        }
        progress_bar.inc(result.buffers.len() as u64);
    }

    progress_bar.finish_and_clear();
    println!("🔨 Finished rendering");
    let duration = start.elapsed();

    image_buffer.reverse();
    write_to_file(&args.output, &image_buffer, image_width, image_height);
    let file_size = std::fs::metadata(&args.output)
        .expect(&format!("Failed to get metadata for {}", args.output))
        .len();
    println!("📝 Wrote to {} ({})", args.output, HumanBytes(file_size));
    println!(
        "⏱️  Time elapsed: {} (avg. {:.2}µs/pixel)",
        HumanDuration(duration),
        duration.as_micros() as f32 / (image_height as f32 * image_width as f32)
    );
    if args.open {
        open::that(args.output.clone()).expect(&format!("Failed to open {}", args.output));
    }
}
