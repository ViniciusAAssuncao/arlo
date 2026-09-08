pub(crate) mod scalar_macro;
pub(crate) mod vector_macro;

pub mod angle;
pub mod closing;
pub mod collision;
pub mod constants;
pub mod duration;
pub mod length;
pub mod operators;
pub mod position;
pub mod speed;
pub mod vector3;
pub mod velocity;

pub use angle::Angle;
pub use closing::time_to_close;
pub use collision::compute_swept_sphere_intersection;
pub use constants::*;
pub use duration::Duration;
pub use length::Length;
pub use position::Position;
pub use speed::Speed;
pub use vector3::Vector3;
pub use velocity::Velocity;
