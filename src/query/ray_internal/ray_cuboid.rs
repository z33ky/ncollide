use na::Real;
use bounding_volume::AABB;
use shape::Cuboid;
use query::{Ray, RayCast, RayIntersection};
use math::{Isometry, Point};

impl<N: Real> RayCast<N> for Cuboid<N> {
    #[inline]
    fn toi_with_ray(&self, m: &Isometry<N>, ray: &Ray<N>, solid: bool) -> Option<N> {
        let dl = Point::from_coordinates(-*self.half_extents());
        let ur = Point::from_coordinates(*self.half_extents());
        AABB::new(dl, ur).toi_with_ray(m, ray, solid)
    }

    #[inline]
    fn toi_and_normal_with_ray(
        &self,
        m: &Isometry<N>,
        ray: &Ray<N>,
        solid: bool,
    ) -> Option<RayIntersection<N>> {
        let dl = Point::from_coordinates(-*self.half_extents());
        let ur = Point::from_coordinates(*self.half_extents());
        AABB::new(dl, ur).toi_and_normal_with_ray(m, ray, solid)
    }

    #[cfg(feature = "dim3")]
    #[inline]
    fn toi_and_normal_and_uv_with_ray(
        &self,
        m: &Isometry<N>,
        ray: &Ray<N>,
        solid: bool,
    ) -> Option<RayIntersection<N>> {
        let dl = Point::from_coordinates(-*self.half_extents());
        let ur = Point::from_coordinates(*self.half_extents());
        AABB::new(dl, ur).toi_and_normal_and_uv_with_ray(m, ray, solid)
    }
}
