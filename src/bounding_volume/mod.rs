//! Bounding volumes.

#[doc(inline)]
pub use bounding_volume::aabb::{aabb, AABB};
pub use bounding_volume::aabb_ball::ball_aabb;
pub use bounding_volume::aabb_utils::{point_cloud_aabb, support_map_aabb};
#[doc(inline)]
pub use bounding_volume::bounding_sphere::{bounding_sphere, BoundingSphere};
pub use bounding_volume::bounding_sphere_utils::{point_cloud_bounding_sphere,
                                                 point_cloud_bounding_sphere_with_center};
#[doc(inline)]
pub use bounding_volume::bounding_volume::{BoundingVolume, HasBoundingVolume};
pub use self::circular_cone::CircularCone;
pub use self::conical_approximation::ConicalApproximation;
pub use self::spatialized_normal_cone::SpatializedNormalCone;

#[doc(hidden)]
pub mod bounding_volume;

#[doc(hidden)]
pub mod aabb;
mod aabb_cuboid;
mod aabb_support_map;
mod aabb_ball;
mod aabb_plane;
#[cfg(feature = "dim3")]
mod aabb_convex;
#[cfg(feature = "dim2")]
mod aabb_convex_polygon;
mod aabb_compound;
mod aabb_polyline;
#[cfg(feature = "dim3")]
mod aabb_trimesh;
mod aabb_utils;
mod aabb_shape;

#[doc(hidden)]
pub mod bounding_sphere;
mod bounding_sphere_cuboid;
#[cfg(feature = "dim3")]
mod bounding_sphere_cone;
mod bounding_sphere_ball;
#[cfg(feature = "dim3")]
mod bounding_sphere_cylinder;
mod bounding_sphere_capsule;
mod bounding_sphere_plane;
#[cfg(feature = "dim3")]
mod bounding_sphere_convex;
#[cfg(feature = "dim2")]
mod bounding_sphere_convex_polygon;
mod bounding_sphere_compound;
#[cfg(feature = "dim3")]
mod bounding_sphere_triangle;
mod bounding_sphere_segment;
mod bounding_sphere_polyline;
#[cfg(feature = "dim3")]
mod bounding_sphere_trimesh;
mod bounding_sphere_utils;
mod bounding_sphere_shape;

mod conical_approximation;
pub(crate) mod circular_cone;
mod spatialized_normal_cone;