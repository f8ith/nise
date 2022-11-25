fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
        .window("Test", 900, 700)
        .resizable()
        .build()
        .unwrap();
}
