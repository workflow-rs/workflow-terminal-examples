use wasm_bindgen::prelude::*;
use workflow_terminal::Result;
use cli::example_terminal;

#[wasm_bindgen(start)]
pub async fn load_terminal() -> Result<()>{
    example_terminal().await?;
    Ok(())
}
