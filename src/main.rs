use loading_bar::{text::TextLoadingBar, Color, LoadingBar};
use std::{io::stdout, thread, time};

use crossterm::{
    cursor::{self, MoveTo, RestorePosition, SavePosition},
    execute,
    terminal::{Clear, ClearType},
};

fn main() {
    let mut cursors: (u16, u16) = (0, 0);
    // the following for loop puts the line number of each line for the first 100 lines (if 100 lines are available)
    for line in 0..100 {
        execute!(
            stdout(),
            MoveTo(0, line as u16),
            Clear(ClearType::CurrentLine),
            SavePosition
        )
        .expect("\x07failed to clear line\x07");
        cursors = cursor::position().expect("Could not get cursor position");
        println!("{}", cursors.1);
    }
    execute!(stdout(), RestorePosition).expect("\x07failed to restore position\x07");

    TextLoadingBar::auto_run(
        "Loading".to_string(),
        "Loading".to_string(),
        10,
        50,
        10,
        (Some(Color::Green), Some(Color::Green), Some(Color::Green)),
        (7, 4),
    );

    let mut bar = LoadingBar::new(50, Some(Color::Green), (7, 13));

    LoadingBar::auto_run(10, 50, 11, Some(Color::Green), (7, 10));
    for i in 0..6 {
        if bar.done {
            // we dont want to advance the bar if it is done
            break;
        }

        let ten_millis = time::Duration::from_secs_f32(2.0); // wow what a descriptive name
        thread::sleep(ten_millis); // sleep for 1 seconds

        // bar.advance_by_percent(600.0); // this will panic because were going over a 100%
        bar.advance_by_print(8);

        match i {
            0 => {
                bar.change_color_print(Some(Color::Black));
            }
            1 => {
                bar.change_color_print(Some(Color::BrightGreen));
            }
            2 => {
                bar.change_color_print(Some(Color::Red));
            }
            3 => {
                bar.change_color_print(Some(Color::Yellow));
            }
            _ => {
                bar.change_color_print(None);
            }
        }
    }

    let (x, y) = cursors;
    execute!(stdout(), MoveTo(x, y)).expect("\x07failed to restore cursor\x07"); // restore cursor to the end of the text 
}
