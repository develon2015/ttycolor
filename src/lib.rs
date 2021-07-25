mod color;
pub use color::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let str = "Hello, world! 你好！";

        let no_effect = format!("A{}B", str.red().bg_green().bold().italic().underline());
        assert_eq!(no_effect, format!("A{}B", str));

        // just using println!()
        println!("A{}B", str.red().bg_green().bold().italic().underline());
        println!("A{}B", str.to_string().red().bg_green().bold().italic().underline());
    }
}
