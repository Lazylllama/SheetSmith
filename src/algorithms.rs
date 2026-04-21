use image::RgbaImage;

pub mod guillotiere_alg;

pub fn pack_images_guillotiere(
    args: guillotiere_alg::GuillotiereArgs,
) -> Result<RgbaImage, anyhow::Error> {
    guillotiere_alg::pack_images_guillotiere(args)
}
