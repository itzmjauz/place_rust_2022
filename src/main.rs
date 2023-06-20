use csscolorparser;
use ggez::conf;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Image};
use ggez::{Context, ContextBuilder, GameResult};

mod csv;

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("r/place2022", "ggez")
        .window_mode(
            conf::WindowMode::default()
                .resizable(true)
                .dimensions(2000.0, 2000.0),
        )
        .build()
        .unwrap();

    let place = Place::new(&mut ctx);

    event::run(ctx, event_loop, place);
}

struct Place {
    canvas: Vec<u8>,
    iter: Box<dyn Iterator<Item = csv::Record>>,
    frame: i64,
    rate: i32,
}

impl Place {
    pub fn new(_ctx: &mut Context) -> Place {
        Place {
            canvas: vec![255; 2000 * 2000 * 4],
            iter: csv::csv_iter("data/2022_place_canvas_history.csv"),
            frame: 0,
            rate: 40000, //40k pixels per frame seems allright
        }
    }
}

impl EventHandler for Place {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let mut timestamp = String::from("");
        for _ in 0..self.rate {
            let record = self.iter.next();
            if let Some(record) = record {
                let color = record
                    .pixel_color
                    .parse::<csscolorparser::Color>()
                    .unwrap()
                    .to_rgba8();
                let x = record
                    .coordinate
                    .split(',')
                    .nth(0)
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                let y = record
                    .coordinate
                    .split(',')
                    .nth(1)
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                let pixel = (y * 2000 + x) * 4;

                self.canvas[pixel] = color[0];
                self.canvas[pixel + 1] = color[1];
                self.canvas[pixel + 2] = color[2];
                self.canvas[pixel + 3] = color[3];

                timestamp = record.timestamp;
            }
        }

        self.frame += self.rate as i64;
        println!("{}", timestamp);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let image: Image = graphics::Image::from_rgba8(ctx, 2000, 2000, &self.canvas)?;
        graphics::draw(ctx, &image, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
        graphics::present(ctx)
    }
}
