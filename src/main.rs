fn main() -> Result<(), std::io::Error> {
    std::fs::remove_file(std::env::current_exe()?)?;
    Ok(())
}