pub struct Hist<T> {
    inner: Vec<T>,
    width: usize,
    _height: usize,
}
impl<T> Hist<T> {
    pub fn from_fn(width: usize, height: usize, f: impl Fn(usize, usize) -> T) -> Self {
        let f = |x: usize, y: usize| -> T { f(x, y) };
        Self {
            inner: (0..height)
                .flat_map(|y| (0..width).map(move |x| f(x, y)))
                .collect(),
            width,
            _height: height,
        }
    }
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.inner.get(y * self.width + x)
    }
}
impl<T: PartialOrd> Hist<T> {
    pub fn max(&self) -> Option<&T> {
        self.inner.iter().max_by(|&x, &y| x.partial_cmp(y).unwrap())
    }
}
