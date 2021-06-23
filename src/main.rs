use std::borrow::BorrowMut;

use iced::Sandbox;
use plotters::{
    prelude::{ChartBuilder, IntoDrawingArea, Polygon},
    style::{Color, BLUE, WHITE},
};
use plotters_bitmap::{bitmap_pixel::BGRXPixel, BitMapBackend};
use plotters_iced_demo::{buffer::BufferWrapper, hist::Hist};

// pub mod buffer;
// pub mod hist;

const WIN_WIDTH: usize = 1024;
const WIN_HEIGHT: usize = 720;

fn main() {
    <App as Sandbox>::run(iced::Settings::default()).unwrap();
}

#[derive(Clone, Debug)]
enum Message {}
struct App {
    buf: BufferWrapper,
}
impl iced::Sandbox for App {
    type Message = Message;
    fn new() -> Self {
        const WIDTH: usize = 256;
        const HEIGHT: usize = 256;

        let hist = {
            const X0: f64 = 128.0;
            const Y0: f64 = 128.0;
            fn f(x: usize, y: usize) -> f64 {
                (x as f64 - X0).powi(2) + (y as f64 - Y0).powi(2)
            }
            Hist::from_fn(WIDTH, HEIGHT, f)
        };

        let mut buf = BufferWrapper {
            data: vec![0u32; WIN_WIDTH * WIN_HEIGHT],
            height: WIN_HEIGHT as u32,
            width: WIN_WIDTH as u32,
        };

        let root = {
            let root = BitMapBackend::<BGRXPixel>::with_buffer_and_format(
                buf.borrow_mut(),
                (WIN_WIDTH as u32, WIN_HEIGHT as u32),
            )
            .unwrap()
            .into_drawing_area();
            let _ = root.fill(&WHITE);
            root
        };
        let chart = {
            let mut chart = ChartBuilder::on(&root)
                .margin(10)
                .set_all_label_area_size(30)
                .build_cartesian_3d(
                    0.0..WIDTH as f64,
                    0.0..*hist.max().unwrap(),
                    0.0..HEIGHT as f64,
                )
                .unwrap();
            chart.configure_axes().draw().unwrap();
            let _ = chart.draw_series(hist.into_polygons(BLUE.mix(0.3)));
            chart
        };
        let _chart_state = chart.into_chart_state();
        drop(root);

        Self { buf }
    }
    fn title(&self) -> String {
        "3d chart".to_string()
    }
    fn update(&mut self, _message: Self::Message) {
        // do nothing
    }
    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        iced::Image::new(self.buf.clone()).into()
    }
}
