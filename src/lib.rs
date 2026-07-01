pub fn is_setup_done() -> Result<(), &'static str> {
    //TODO: add real logic
    let done_setup = true;
    if !done_setup {
        return Err("CRAW setup not done. Please complete setup first. Run `craw setup`");
    }
    Ok(())
}
pub fn run_setup() {
    println!("running craw setup now");
}
