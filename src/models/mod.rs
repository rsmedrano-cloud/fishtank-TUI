pub mod fish;
pub mod decoration;
pub mod achievement;

pub use fish::{Fish, FishState, Species, GrowthStage, Gender};
pub use decoration::{Decoration, DecorationType};
pub use achievement::{Achievement, create_achievements};
