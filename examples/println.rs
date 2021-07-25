extern crate ttycolor;
use ttycolor::*;

fn main() {
    let str = "System";
    println!(
        "{} {} {tag}: {target} error! ({location})",
        "1".cyan(),
        "->".bg_magenta(),
        tag = "Warning:".red().italic().bold(),
        target = str.bg_black().fg(rgb(255, 0, 0)).bold(),
        location = format!("{:018p}", Box::new("abc")).underline(),
    );
    println!(
        "{} {} {tag}: {target} recovery!",
        "2".fg(rgb(0, 255, 255)),
        "->".bg_black().bold(),
        tag = "Finished:".green().italic().bold(),
        target = str.clone().to_string().bg_black().fg(rgb(0, 0, 255)).bold(),
    );

    // format! is no color
    let a = "ABC";
    let b = format!("x{}y", a.red()); // The Display trait has been called, tty color reseted
    println!("{}", b); //: xABCy
    println!("{}", a.red().eq(a)); //: true
}
