use termion::{color, style, clear, async_stdin, terminal_size};

use std::{io, thread, time, fs};
use std::time::Duration;
use std::io::{stdout, Write, stdin, Stdout, Read};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::event::{Key, Event, MouseEvent};
use termion::input::{TermRead, MouseTerminal};
use termion::event::*;
use termion::cursor;
use termion::cursor::DetectCursorPos;
use termion::screen::{AlternateScreen, ToMainScreen, ToAlternateScreen};
use termion::color::{DetectColors, Bg, AnsiValue};

// ----------------------------------------------------------------------------- styles

// fn main() {
// //     println!("{}Red", color::Fg(color::Red));
// //     println!("{}Blue", color::Fg(color::Blue));
// //     println!("{}Blue'n'Bold{}", style::Bold, style::Reset);
// //     println!("{}Just plain italic", style::Italic);
// //     println!("{}", clear::All);
//     print!("{}{}Stuff", termion::clear::All, termion::cursor::Goto(5, 10));
// }

// fn main() {
//     println!("{red}more red than any comrade{reset}",
//              red   = color::Fg(color::Red),
//              reset = color::Fg(color::Reset));
//     // Sleep for a short period of time.
//     thread::sleep(Duration::from_millis(300));
//     // Go back;
//     println!("\r");
//     // Clear the line and print some new stuff
//     print!("{clear}{red}g{blue}a{green}y{red} space communism{reset}",
//             clear = clear::CurrentLine,
//             red   = color::Fg(color::Red),
//             blue  = color::Fg(color::Blue),
//             green = color::Fg(color::Green),
//             reset = color::Fg(color::Reset));
// }

// fn main() {
//     println!("{bold}{red}g{blue}a{green}y{red} space communism{reset}",
//             bold  = style::Bold,
//             red   = color::Fg(color::Red),
//             blue  = color::Fg(color::Blue),
//             green = color::Fg(color::Green),
//             reset = style::Reset);
// }

// const COMMUNISM: &'static str = r#"
//               !#########       #
//             !########!          ##!
//          !########!               ###
//       !##########                  ####
//     ######### #####                ######
//      !###!      !####!              ######
//        !           #####            ######!
//                      !####!         #######
//                         #####       #######
//                           !####!   #######!
//                              ####!########
//           ##                   ##########
//         ,######!          !#############
//       ,#### ########################!####!
//     ,####'     ##################!'    #####
//   ,####'            #######              !####!
//  ####'                                      #####
//  ~##                                          ##~
// "#;
//
// fn main() {
//     let mut state = 0;
//
//     println!("\n{}{}{}{}{}{}",
//              cursor::Hide,
//              clear::All,
//              cursor::Goto(1, 1),
//              color::Fg(color::Black),
//              color::Bg(color::Red),
//              COMMUNISM);
//     loop {
//         println!("{}{}           ☭ GAY ☭ SPACE ☭ COMMUNISM ☭           ",
//                  cursor::Goto(1, 1),
//                  color::Bg(color::AnsiValue(state)));
//         println!("{}{}             WILL PREVAIL, COMRADES!             ",
//                  cursor::Goto(1, 20),
//                  color::Bg(color::AnsiValue(state)));
//
//         state += 1;
//         state %= 8;
//
//         thread::sleep(time::Duration::from_millis(90));
//     }
// }

// fn main() {
//     let count;
//     {
//         let mut term = stdout().into_raw_mode().unwrap();
//         count = term.available_colors().unwrap();
//     }
//
//     println!("This terminal supports {} colors.", count);
//     for i in 0..count {
//         print!("{} {}", Bg(AnsiValue(i as u8)), Bg(AnsiValue(0)));
//     }
//     println!();
// }

// fn rainbow<W: Write>(stdout: &mut W, blue: u8) {
//     write!(stdout,
//            "{}{}",
//            termion::cursor::Goto(1, 1),
//            termion::clear::All)
//             .unwrap();
//
//     for red in 0..32 {
//         let red = red * 8;
//         for green in 0..64 {
//             let green = green * 4;
//             write!(stdout,
//                    "{} ",
//                    termion::color::Bg(termion::color::Rgb(red, green, blue)))
//                     .unwrap();
//         }
//         write!(stdout, "\n\r").unwrap();
//     }
//
//     writeln!(stdout, "{}b = {}", termion::style::Reset, blue).unwrap();
// }
//
// fn main() {
//     let stdin = stdin();
//     let mut stdout = stdout().into_raw_mode().unwrap();
//
//     writeln!(stdout,
//              "{}{}{}Use the up/down arrow keys to change the blue in the rainbow.",
//              termion::clear::All,
//              termion::cursor::Goto(1, 1),
//              termion::cursor::Hide)
//             .unwrap();
//
//     let mut blue = 172u8;
//
//     for c in stdin.keys() {
//         match c.unwrap() {
//             Key::Up => {
//                 blue = blue.saturating_add(4);
//                 rainbow(&mut stdout, blue);
//             }
//             Key::Down => {
//                 blue = blue.saturating_sub(4);
//                 rainbow(&mut stdout, blue);
//             }
//             Key::Char('q') => break,
//             _ => {}
//         }
//         stdout.flush().unwrap();
//     }
//
//     write!(stdout, "{}", termion::cursor::Show).unwrap();
// }

// fn main() {
//     println!("{lighgreen}-- src/test/ui/borrow-errors.rs at 82:18 --\n\
//               {red}error: {reset}{bold}two closures require unique access to `vec` at the same time {reset}{bold}{magenta}[E0524]{reset}\n\
//               {line_num_fg}{line_num_bg}79 {reset}     let append = |e| {{\n\
//               {line_num_fg}{line_num_bg}{info_line}{reset}                  {red}^^^{reset} {error_fg}first closure is constructed here\n\
//               {line_num_fg}{line_num_bg}80 {reset}         vec.push(e)\n\
//               {line_num_fg}{line_num_bg}{info_line}{reset}                 {red}^^^{reset} {error_fg}previous borrow occurs due to use of `vec` in closure\n\
//               {line_num_fg}{line_num_bg}84 {reset}     }};\n\
//               {line_num_fg}{line_num_bg}85 {reset} }}\n\
//               {line_num_fg}{line_num_bg}{info_line}{reset} {red}^{reset} {error_fg}borrow from first closure ends here",
//              lighgreen = color::Fg(color::LightGreen),
//              red = color::Fg(color::Red),
//              bold = style::Bold,
//              reset = style::Reset,
//              magenta = color::Fg(color::Magenta),
//              line_num_bg = color::Bg(color::AnsiValue::grayscale(3)),
//              line_num_fg = color::Fg(color::AnsiValue::grayscale(18)),
//              info_line = "|  ",
//              error_fg = color::Fg(color::AnsiValue::grayscale(17)))
// }

// fn main() {
//     for r in 0..255 {
//         let c = color::Rgb(r, !r, 2 * ((r % 128) as i8 - 64).abs() as u8);
//         println!("{}{}{}wow", cursor::Goto(1, 1), color::Bg(c), clear::All);
//         thread::sleep(time::Duration::from_millis(100));
//     }
// }

// ----------------------------------------------------------------------------- raw mode

// fn main() {
//     let mut stdout = stdout().into_raw_mode().unwrap();
//     writeln!(stdout, "hey there").unwrap();
// }

// ----------------------------------------------------------------------------- keys

// fn main() {
//     // Get the standard input stream.
//     let stdin = stdin();
//     // Get the standard output stream and go to raw mode.
//     let mut stdout = stdout().into_raw_mode().unwrap();
//
//     write!(stdout, "{}{}q to exit. Type stuff, use alt, and so on.{}",
//            // Clear the screen.
//            termion::clear::All,
//            // Goto (1,1).
//            termion::cursor::Goto(1, 1),
//            // Hide the cursor.
//            termion::cursor::Hide).unwrap();
//     // Flush stdout (i.e. make the output appear).
//     stdout.flush().unwrap();
//
//     for c in stdin.keys() {
//         // Clear the current line.
//         write!(stdout, "{}{}", termion::cursor::Goto(1, 1), termion::clear::CurrentLine).unwrap();
//
//         // Print the key we type...
//         match c.unwrap() {
//             // Exit.
//             Key::Char('q') => break,
//             Key::Char(c)   => println!("{}", c),
//             Key::Alt(c)    => println!("Alt-{}", c),
//             Key::Ctrl(c)   => println!("Ctrl-{}", c),
//             Key::Left      => println!("<left>"),
//             Key::Right     => println!("<right>"),
//             Key::Up        => println!("<up>"),
//             Key::Down      => println!("<down>"),
//             _              => println!("Other"),
//         }
//
//         // Flush again.
//         stdout.flush().unwrap();
//     }
//
//     // Show the cursor again before we exit.
//     write!(stdout, "{}", termion::cursor::Show).unwrap();
// }

// fn main() {
//     // Initialize 'em all.
//     let stdout = stdout();
//     let mut stdout = stdout.lock().into_raw_mode().unwrap();
//     let stdin = stdin();
//     let stdin = stdin.lock();
//
//     write!(stdout,
//            "{}{}{}yo, 'q' will exit.{}{}",
//            termion::clear::All,
//            termion::cursor::Goto(5, 5),
//            termion::style::Bold,
//            termion::style::Reset,
//            termion::cursor::Goto(20, 10))
//             .unwrap();
//     stdout.flush().unwrap();
//
//     let mut bytes = stdin.bytes();
//     loop {
//         let b = bytes.next().unwrap().unwrap();
//
//         match b {
//                 // Quit
//                 b'q' => return,
//                 // Clear the screen
//                 b'c' => write!(stdout, "{}", termion::clear::All),
//                 // Set red color
//                 b'r' => write!(stdout, "{}", color::Fg(color::Rgb(5, 0, 0))),
//                 // Write it to stdout.
//                 a => write!(stdout, "{}", a),
//             }
//             .unwrap();
//
//         stdout.flush().unwrap();
//     }
// }

// ----------------------------------------------------------------------------- mouse

// fn main() {
//     // get the standard input stream
//     let stdin = io::stdin();
//     // set standard output to a terminal with added mouse support
//     let mut stdout = MouseTerminal::from(io::stdout().into_raw_mode().unwrap());
//     writeln!(stdout,
//              "{}{}q to exit. Type stuff, use alt, click around...",
//              termion::clear::All,
//              termion::cursor::Goto(1, 1))
//         .unwrap();
//     for c in stdin.events() {
//         let evt = c.unwrap();
//         //if event is a press of 'q' button = exit
//         //otherwise get coords of mouse and move the cursor to those coords
//         match evt {
//             Event::Key(Key::Char('q')) => break,
//             Event::Mouse(me) => {
//                 match me {
//                     MouseEvent::Press(_, a, b) |
//                     MouseEvent::Release(a, b) |
//                     MouseEvent::Hold(a, b) => {
//                         write!(stdout, "{}", cursor::Goto(a, b)).unwrap();
//                     }
//                 }
//             }
//             _ => {}
//         }
//         stdout.flush().unwrap();
//     }
// }

// ----------------------------------------------------------------------------- my own - text editor

// // move cursor around
// fn main() {
//     let stdin = io::stdin();
//     let mut stdout = stdout().into_raw_mode().unwrap();
//
//     let mut x = 1;
//     let mut y = 1;
//
//     write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(x, y));
//     stdout.flush().unwrap();
//
//     for c in stdin.keys() {
//         match c.unwrap() {
//             Key::Char('q') => break,
//             Key::Left => x -= 1,
//             Key::Right => x += 1,
//             Key::Up => y -= 1,
//             Key::Down => y += 1,
//             _ => ()
//         }
//         write!(stdout, "{}", termion::cursor::Goto(x, y));
//         stdout.flush().unwrap();
//     }
// }

fn main() {
    let stdin = io::stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "{}{}{}",
        //interesting, in order to color the entire background of the terminal all you have to do is set the BG color FIRST, and only then clear the terminal
        termion::cursor::Goto(1, 1),
        termion::color::Bg(termion::color::Red),
        termion::clear::All,
    );
    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Esc => break,
            Key::Backspace => {
                let (x, y) = stdout.cursor_pos().unwrap();
                write!(stdout, "{}{}", " ", termion::cursor::Goto(x-1, y));
            },
            Key::Char('\n') => { write!(stdout, "\r\n"); },
            Key::Char(x) => { write!(stdout, "{}", x); },
                        Key::Left => {
                let (x, y) = stdout.cursor_pos().unwrap();
                write!(stdout, "{}", termion::cursor::Goto(x-1, y));
            },
            Key::Right => {
                let (x, y) = stdout.cursor_pos().unwrap();
                write!(stdout, "{}", termion::cursor::Goto(x+1, y));
            },
            Key::Up => {
                let (x, y) = stdout.cursor_pos().unwrap();
                write!(stdout, "{}", termion::cursor::Goto(x, y-1));
            },
            Key::Down => {
                let (x, y) = stdout.cursor_pos().unwrap();
                write!(stdout, "{}", termion::cursor::Goto(x, y+1));
            },
            _ => ()
        }
        stdout.flush().unwrap();
    }
}

// -----------------------------------------------------------------------------

// fn run_alt_screen() {
//     {
//         let mut screen = AlternateScreen::from(stdout());
//         write!(screen, "Welcome to the alternate screen.\n\nPlease wait patiently until we arrive back at the main screen in a about three seconds.").unwrap();
//         screen.flush().unwrap();
//
//         thread::sleep(time::Duration::from_secs(3));
//     }
// }
//
// fn main() {
//     let stdin = io::stdin();
//     let mut stdout = stdout().into_raw_mode().unwrap();
//
//     write!(stdout, "{}{} this is the normal screen. Press x to switch", termion::clear::All, termion::cursor::Goto(1, 1));
//     stdout.flush().unwrap();
//
//     for c in stdin.keys() {
//         match c.unwrap() {
//             Key::Char('x') => {
//                 run_alt_screen();
//                 write!(stdout, "{}{} this is the normal screen. Press x to switch", termion::clear::All, termion::cursor::Goto(1, 1));
//                 println!("Phew! We are back.");
//                 stdout.flush().unwrap();
//             },
//             _ => (),
//         }
//     }
//
// }

// fn write_alt_screen_msg<W: Write>(screen: &mut W) {
//     write!(screen, "{}{}Welcome to the alternate screen.{}Press '1' to switch to the main screen or '2' to switch to the alternate screen.{}Press 'q' to exit (and switch back to the main screen).",
//            termion::clear::All,
//            termion::cursor::Goto(1, 1),
//            termion::cursor::Goto(1, 3),
//            termion::cursor::Goto(1, 4)).unwrap();
// }
//
// fn main() {
//     let stdin = stdin();
//     let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
//     write!(screen, "{}", termion::cursor::Hide).unwrap();
//     write_alt_screen_msg(&mut screen);
//
//     screen.flush().unwrap();
//
//     for c in stdin.keys() {
//         match c.unwrap() {
//             Key::Char('q') => break,
//             Key::Char('1') => {
//                 write!(screen, "{}", ToMainScreen).unwrap();
//             }
//             Key::Char('2') => {
//                 write!(screen, "{}", ToAlternateScreen).unwrap();
//                 write_alt_screen_msg(&mut screen);
//             }
//             _ => {}
//         }
//         screen.flush().unwrap();
//     }
//     write!(screen, "{}", termion::cursor::Show).unwrap();
// }

// ----------------------------------------------------------------------------- async loop

// fn main() {
//     let stdout = stdout();
//     let mut stdout = stdout.lock().into_raw_mode().unwrap();
//
//     // this is where we call async
//     let mut stdin = async_stdin().bytes();
//
//     // simpley clear the terminal once in the beginning, before the loop
//     write!(stdout,
//            "{}{}",
//            termion::clear::All,
//            termion::cursor::Goto(1, 1))
//             .unwrap();
//
//     loop {
//         write!(stdout, "{}", termion::clear::CurrentLine).unwrap();
//
//         // this is where we take user input and print it to the screen on the first line. This happens async.
//         let b = stdin.next();
//         write!(stdout, "\r{:?}    <- This demonstrates the async read input char. Between each update a 100 ms. is waited, simply to demonstrate the async fashion. \n\r", b).unwrap();
//         if let Some(Ok(b'q')) = b {
//             break;
//         }
//         stdout.flush().unwrap();
//
//         //in the meantime the screen does its own thing on the main thread
//         thread::sleep(Duration::from_millis(50));
//         stdout.write_all(b"# ").unwrap();
//         stdout.flush().unwrap();
//
//         thread::sleep(Duration::from_millis(50));
//         stdout.write_all(b"\r #").unwrap();
//         write!(stdout, "{}", termion::cursor::Goto(1, 1)).unwrap();
//         stdout.flush().unwrap();
//     }
// }

// ----------------------------------------------------------------------------- tty

// fn main() {
//     if termion::is_tty(&fs::File::create("/dev/stdout").unwrap()) {
//         println!("This is a TTY!");
//     } else {
//         println!("This is not a TTY :(");
//     }
// }

// ----------------------------------------------------------------------------- pw

// fn main() {
//     let stdout = stdout();
//     let mut stdout = stdout.lock();
//     let stdin = stdin();
//     let mut stdin = stdin.lock();
//
//     stdout.write_all(b"password: ").unwrap();
//     stdout.flush().unwrap();
//
//     // a special function they have for this purpose
//     let pass = stdin.read_passwd(&mut stdout);
//
//     if let Ok(Some(pass)) = pass {
//         stdout.write_all(pass.as_bytes()).unwrap();
//         stdout.write_all(b"\n").unwrap();
//     } else {
//         stdout.write_all(b"Error\n").unwrap();
//     }
// }

// ----------------------------------------------------------------------------- terminal size

// fn main() {
//     println!("Size is {:?}", terminal_size().unwrap())
// }
