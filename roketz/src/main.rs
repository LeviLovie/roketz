mod logging;

use utils::prelude::*;

#[main_pretty_error]
fn main() -> Result<()> {
    logging::init();
    logging::debug();

    Ok(())
}
