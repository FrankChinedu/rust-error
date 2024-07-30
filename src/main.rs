pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

mod fs;
use fs::list_files;

fn main() -> Result<()> {
    let files = list_files(".")?;
    println!("{files:#?}");
    Ok(())
}
