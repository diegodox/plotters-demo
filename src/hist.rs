use std::usize;

use plotters::{prelude::Polygon, style::RGBAColor};

pub struct Hist<T> {
    inner: Vec<T>,
    width: usize,
    height: usize,
}
impl<T> Hist<T> {
    pub fn from_fn(width: usize, height: usize, f: impl Fn(usize, usize) -> T) -> Self {
        let f = |x: usize, y: usize| -> T { f(x, y) };
        Self {
            inner: (0..height)
                .flat_map(|y| (0..width).map(move |x| f(x, y)))
                .collect(),
            width,
            height,
        }
    }
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.inner.get(y * self.width + x)
    }
}

impl Hist<f64> {
    pub fn into_polygons(self, color: RGBAColor) -> impl Iterator<Item = Polygon<(f64, f64, f64)>> {
        let width = self.width;
        let height = self.height;
        let v = move |x: usize, z: usize| *self.get(x, z).unwrap();
        (0..width)
            .flat_map(move |x| std::iter::repeat(x).zip(0..height))
            .map(move |(x, z)| {
                Polygon::new(
                    vec![
                        (x as f64, v(x, z), z as f64),
                        (x as f64 + 1.0, v(x, z), z as f64),
                        (x as f64, v(x, z), z as f64 + 1.0),
                        (x as f64 + 1.0, v(x, z), z as f64 + 1.0),
                    ],
                    &color,
                )
            })
    }
}
impl Hist<usize> {
    pub fn into_polygons(
        self,
        color: RGBAColor,
    ) -> impl Iterator<Item = Polygon<(usize, usize, usize)>> {
        let width = self.width;
        let height = self.height;
        let v = move |x: usize, z: usize| *self.get(x, z).unwrap();
        (0..width)
            .flat_map(move |x| std::iter::repeat(x).zip(0..height))
            .map(move |(x, z)| {
                Polygon::new(
                    vec![
                        (x, v(x, z), z),
                        (x + 1, v(x, z), z),
                        (x, v(x, z), z + 1),
                        (x + 1, v(x, z), z + 1),
                    ],
                    &color,
                )
            })
    }
}
impl<T: PartialOrd> Hist<T> {
    pub fn max(&self) -> Option<&T> {
        self.inner.iter().max_by(|&x, &y| x.partial_cmp(y).unwrap())
    }
}
