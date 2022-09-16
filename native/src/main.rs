use workflow_terminal::Result;
use cli::example_terminal;

#[async_std::main]
async fn main() -> Result<()>{
    example_terminal().await?;
    Ok(())
}
