use riu_rs::img::ImgGen;

fn main() {
    let img_gen = ImgGen::new();

    // let img = img_gen.upscale_times("anime-img1.jpg", 4);
    let img = img_gen.downscale_times("prof.jpg", 1);

    img_gen.save(img, "prof.jpg");
}
