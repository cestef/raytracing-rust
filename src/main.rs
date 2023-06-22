mod camera;
mod materials;
mod shapes;
mod utils;
use std::{
    fs::File,
    io::Write,
    rc::Rc,
    sync::{mpsc::channel, Arc, Mutex},
    thread,
};

use crate::{
    camera::Camera,
    materials::{lambertian::Lambertian, metal::Metal},
    shapes::list::HittableList,
    utils::{helpers::random_float, vec::Color},
};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use shapes::sphere::Sphere;
use utils::vec::{Point3, Vec3};

struct TxData {
    buffer: String,
    thread_id: usize,
}

fn main() {
    // print!("\x1B[2J\x1B[1;1H");
    // let cpus = num_cpus::get();
    let cpus = 8;
    let aspect_ratio = 16.0 / 9.0; // =~ 1.7
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 500;

    let camera = Box::new(Camera::new(
        Vec3::new(-2.0, 2.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        aspect_ratio as f32,
    ));

    let world = Box::new(HittableList::new(vec![
        Box::new(Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            Box::new(Lambertian::new(Color::new(0.9, 0.9, 0.0))),
        )),
        Box::new(Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            0.5,
            Box::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.0)),
        )),
        Box::new(Sphere::new(
            Point3::new(0.0, 0.0, -1.0),
            0.5,
            Box::new(Lambertian::new(Color::new(0.7, 0.3, 0.3))),
        )),
        Box::new(Sphere::new(
            Point3::new(1.0, 0.0, -1.0),
            0.5,
            Box::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.3)),
        )),
    ]));

    let mut image_buffer = String::new();

    let start = std::time::Instant::now();
    let rows_per_chunk = (image_height as f32 / cpus as f32).floor() as i32;
    let image_rows = (0..image_height).collect::<Vec<i32>>();
    let rows_chunks = image_rows.chunks(rows_per_chunk as usize);
    let (buffer_tx, buffer_rx) = channel::<TxData>();
    let mp = MultiProgress::new();
    thread::scope(|s| {
        mp.set_move_cursor(true);
        mp.println(format!(
            "Rendering {}x{} ({} spp) | {} threads ({} rows/cpu)",
            image_width, image_height, samples_per_pixel, cpus, rows_per_chunk
        ))
        .unwrap();
        for i in 0..cpus {
            let world = world.clone();
            let camera = camera.clone();
            let mut rows_chunks = rows_chunks.clone();
            let buffer_tx = buffer_tx.clone();
            let p = mp.add(ProgressBar::new(rows_per_chunk as u64));

            s.spawn(move || {
                let mut average_speed = 0.0;
                let mut image_buffer = String::new();
                // println!("Thread {} started", i);
                let chunk = rows_chunks
                    .nth(i)
                    .expect("Failed to get chunk from rows_chunks");
                p.set_style(
                    ProgressStyle::default_bar()
                        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
                        .unwrap()
                        .progress_chars("##-"),
                );
                p.set_message(format!(
                    "Thread {} ({}->{})",
                    i,
                    chunk[0],
                    chunk[chunk.len() - 1]
                ));
                let row_start_time = std::time::Instant::now();
                // \x1B[2K\r
                for j in chunk.iter() {
                    let start_line = std::time::Instant::now();
                    for i in 0..image_width {
                        let mut average_color = Color::default();

                        for _ in 0..samples_per_pixel {
                            let u = (i as f32 + random_float()) / (image_width - 1) as f32; // 0.0 <= u <= 1.0 | u is the horizontal component of the pixel
                            let v = (*j as f32 + random_float()) / (image_height - 1) as f32; // 0.0 <= v <= 1.0 | v is the vertical component of the pixel
                            let r = camera.get_ray(u, v);
                            average_color += r.color(&world, 1000);
                        }

                        average_color /= samples_per_pixel as f32;
                        average_color = average_color.clamp(0.0, 0.999);
                        average_color = average_color.gamma_correct(2.0);
                        let res = format!("{}\n", average_color.to_string_color());
                        // print!("{}", res);
                        image_buffer.push_str(&res);
                    }
                    let duration = start_line.elapsed();
                    average_speed = (average_speed + duration.as_secs_f32()) / 2.0;
                    p.inc(1);
                }
                buffer_tx
                    .send(TxData {
                        buffer: image_buffer,
                        thread_id: i,
                    })
                    .expect("Failed to send buffer to buffer_tx");
                let duration = row_start_time.elapsed();
                p.finish_with_message(format!(
                    "Thread {} finished in {:.2}s",
                    i,
                    duration.as_secs_f32()
                ));
            });
        }
    });

    buffer_rx.iter().take(cpus).for_each(|data| {
        image_buffer.push_str(&data.buffer);
    });

    let mut file_stream = File::create("image.ppm").unwrap();

    file_stream
        .write_all(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes())
        .unwrap();
    file_stream.write_all(image_buffer.as_bytes()).unwrap();

    let duration = start.elapsed();
    println!(
        "\nTime elapsed: {:?} ({}ms/line)",
        duration,
        duration.as_millis() as f32 / image_height as f32
    );
}
