use wasm_bindgen::prelude::*;
use workflow_terminal::Result;
use cli::example_terminal;

#[wasm_bindgen(start)]
pub async fn load_terminal() -> Result<()>{

    console_error_panic_hook::set_once();

    example_terminal().await?;
    Ok(())
}
