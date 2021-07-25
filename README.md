# ttycolor

Easy way to use termcolor.

# usage

```
extern crate ttycolor;
use ttycolor::ColorTrait;

fn main() {
    let str = "Warning";

    println!("{}: xxxxxx", str.red());
    println!("{}: xxxxxx", str.fg(ttycolor::rgb(0, 255, 0)));
    println!("{}: xxxxxx", str.cyan().bold());
    println!("{}: xxxxxx", str.white().bg_magenta().bold().italic().underline());
    
    println!("{}: xxxxxx", String::from(str).red());
    println!("{}: {}", str.green(), format!("{:018p}", str).red());
}
```
