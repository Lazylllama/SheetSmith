use image::RgbaImage;

pub mod guillotiere_alg;

#[derive(Debug, PartialEq)]
pub enum Algorithm {
    Guillotiere,
}

pub fn pack_images_guillotiere(
    args: guillotiere_alg::GuillotiereArgs,
) -> Result<RgbaImage, anyhow::Error> {
    guillotiere_alg::pack_images_guillotiere(args)
}
