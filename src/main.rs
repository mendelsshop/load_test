use loading_bar::{Color, LoadingBar};
use std::{io::Write, thread, time};

fn main() {
    let mut bar = LoadingBar::new(4, Some(Color::Green));
    // bar.display();

    // flush stdout or else wont work properly

    for i in 0..9 {
        if bar.done {
            // we dont want to advance the bar if it is done
            return;
        }
        let ten_millis = time::Duration::from_secs_f32(1.0); // wow what a descriptive name
        thread::sleep(ten_millis); // sleep for 1 seconds

        // bar.advance_by_percent(600.0); this will panic because were going over a 100%
        bar.advance_by(2);

        match i {
            0 => {
                bar.change_color(Some(Color::Black));
            }
            1 => {
                bar.change_color(Some(Color::BrightGreen));
            }
            2 => {
                bar.change_color(Some(Color::Red));
            }
            3 => {
                bar.change_color(Some(Color::Yellow));
            }
            _ => {
                bar.change_color(None);
            }
        };

        print!("{}", bar);
        // bar.display();

        // flush stdout or else wont work properly
        std::io::stdout().flush().unwrap();
    }
    println!();
}
