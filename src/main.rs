mod camera;
mod materials;
mod shapes;
mod utils;
use std::{fs::File, io::Write, sync::mpsc::channel, thread};

use crate::{
    camera::Camera,
    materials::{lambertian::Lambertian, metal::Metal},
    shapes::list::HittableList,
    utils::{
        helpers::{random_float, random_float_range, split_evenly},
        hittable,
        vec::Color,
    },
};
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};
use shapes::sphere::Sphere;
use utils::vec::{Point3, Vec3};

struct TxData {
    buffer: String,
    thread_id: usize,
}

fn main() {
    // print!("\x1B[2J\x1B[1;1H");
    let cpus = num_cpus::get();
    // let cpus = 8;
    let aspect_ratio = 16.0 / 9.0; // =~ 1.7
    let image_width = 2560;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;

    let camera = Box::new(Camera::new(
        Vec3::new(-5.0, 5.0, 5.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        aspect_ratio as f32,
    ));
    let random: Vec<Box<dyn hittable::Hittable>> = vec![0; 10]
        .iter()
        .map(|_| {
            let color = Color::new(
                random_float() * random_float(),
                random_float() * random_float(),
                random_float() * random_float(),
            );
            let material = if random_float() < 0.7 {
                Box::new(Lambertian::new(color)) as Box<dyn materials::Material + Sync + Send>
            } else {
                Box::new(Metal::new(color, 0.0)) as Box<dyn materials::Material + Sync + Send>
            };
            Box::new(Sphere::new(
                Point3::new(
                    random_float_range(-7.0, 7.0),
                    // random_float_range(-5.0, 5.0),
                    0.0,
                    random_float_range(-7.0, 7.0),
                ),
                random_float_range(0.1, 2.0),
                material,
            )) as Box<dyn hittable::Hittable>
        })
        .collect();
    let world = Box::new(HittableList::new(
        vec![Box::new(Sphere::new(
            Point3::new(0.0, -1010.0, 0.0),
            1000.0,
            Box::new(Lambertian::new(Color::new(0.5, 0.5, 0.5))),
        )) as Box<dyn hittable::Hittable>]
        .into_iter()
        .chain(random.into_iter())
        .collect(),
    ));

    let mut image_buffer = vec![String::new(); cpus];

    let start = std::time::Instant::now();

    let mut image_rows = (0..image_height).collect::<Vec<i32>>();
    image_rows.reverse();

    let (buffer_tx, buffer_rx) = channel::<TxData>();

    let mp = MultiProgress::new();

    thread::scope(|s| {
        let (rows_chunks, rows_per_chunk) = split_evenly(image_rows, cpus);

        mp.set_move_cursor(true);
        mp.println(format!(
            "Rendering {}x{} ({} spp) | {} threads ({} rows/cpu)",
            image_width, image_height, samples_per_pixel, cpus, rows_per_chunk[0]
        ))
        .unwrap();

        for i in 0..cpus {
            let world = world.clone();
            let camera = camera.clone();
            let rows_chunks = rows_chunks.clone();
            let buffer_tx = buffer_tx.clone();

            let p = mp.add(ProgressBar::new(rows_per_chunk[i] as u64));

            s.spawn(move || {
                let mut average_speed = 0.0;
                let mut image_buffer = String::new();
                // println!("Thread {} started", i);
                let chunk = rows_chunks
                    .get(i)
                    .expect("Failed to get chunk from rows_chunks");
                p.set_style(
                    ProgressStyle::default_bar()
                        .template(&format!(
                            "[{}] {{elapsed}} ({{eta:3}}) {{wide_bar}} [{{pos:>2}}/{{len:2}}] {{msg}}",
                            i + 1
                        ))
                        .unwrap()
                        .progress_chars("█▉▊▋▌▍▎▏  "),
                );
                // p.set_message(format!("{}->{}", chunk[0], chunk[chunk.len() - 1]));
                // let row_start_time = std::time::Instant::now();
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
                // let duration = row_start_time.elapsed();
                // p.finish_with_message(format!("✅ in {}", HumanDuration(duration)));
            });
        }
    });

    buffer_rx.iter().take(cpus).for_each(|data| {
        image_buffer[data.thread_id] = data.buffer;
    });

    let mut file_stream = File::create("image.ppm").unwrap();

    file_stream
        .write_all(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes())
        .unwrap();
    file_stream
        .write_all(image_buffer.join("").as_bytes())
        .unwrap();

    let duration = start.elapsed();
    println!(
        "\n⏱️  Time elapsed: {} (avg. {:.2}ms/line)\n",
        HumanDuration(duration),
        duration.as_millis() as f32 / image_height as f32
    );
}
