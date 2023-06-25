use clap::Parser;

#[derive(Debug, Parser)]
#[clap(
    name = "raytracer",
    version = "0.1.0",
    author = "cstef",
    about = "A raytracer written in Rust."
)]
pub struct Args {
    #[clap(short, long, default_value_t = 2560)]
    pub width: i32,
    #[clap(short, long)]
    pub height: Option<i32>,
    #[clap(short, long, default_value = "16/9")]
    pub aspect_ratio: String,
    #[clap(short, long, default_value_t = 100)]
    pub samples: i32,
    #[clap(short, long, default_value_t = num_cpus::get())]
    pub threads: usize,
    #[clap(short, long, default_value_t = 100)]
    pub jobs: usize,
    #[clap(short, long, default_value = "output.ppm")]
    pub output: String,
    #[clap(short, long, default_value_t = 90.0)]
    pub fov: f32,
    #[clap(long)]
    pub open: bool,
    #[clap(long)]
    pub clear: bool,
}
