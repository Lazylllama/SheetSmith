pub mod algorithms;
pub mod utils;

pub use algorithms::guillotiere_alg::GuillotiereArgs;
pub use algorithms::pack_images_guillotiere;
pub use utils::{find_optimal_size, trim_image};
