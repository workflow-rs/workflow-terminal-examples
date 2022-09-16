// use workflow_terminal::{Result, cli::Terminal as TerminalTrait};
use std::sync::{Arc,Mutex};
use async_trait::async_trait;
use workflow_terminal::Terminal;
// use workflow_terminal::Options;
use workflow_terminal::Result;
use workflow_terminal::Cli;
use workflow_terminal::parse;
use workflow_log::*;


struct ExampleCli {
    term : Arc<Mutex<Option<Arc<Terminal>>>>
}

impl ExampleCli {

    fn new() -> Self {
        ExampleCli {
            term : Arc::new(Mutex::new(None))
        }
    }

    fn term(&self) -> Arc<Terminal> {
        self.term.lock().unwrap().as_ref().unwrap().clone()
    }

}

impl workflow_log::Sink for ExampleCli {
    fn write(&self, _level:Level, args : &std::fmt::Arguments<'_>) -> bool {
        
        self.term().writeln(args.to_string());
        // self.term().writeln("HELLO WORLD".to_string());
        // self.term().writeln(args);
        // return: 
        // - false for default log output handling (print to stdout or web console)
        // - true, to disable further processing (no further output is made)
        true
    }
}

#[async_trait]
impl Cli for ExampleCli {

    fn init(&self, term : &Arc<Terminal>) -> Result<()> {
        *self.term.lock().unwrap() = Some(term.clone());
        Ok(())
    }

    async fn digest(&self, term : &Arc<Terminal>, cmd: String) -> Result<()> {
        //println!("cmd:: {}", cmd);
        let argv = parse(&cmd);
        //println!("argv[0]:: {}", argv[0]);
        match argv[0].as_str() {
            "hello" => {
                term.writeln("hello back to you!");
            },
            "test" => {
                log_trace!("log_trace!() macro test");
            },
            "exit" => {
                term.writeln("bye!");
                term.exit();
            },
            _ => {
                return Err(format!("command not found: {}", cmd).into())
            }
        }

        Ok(())
    }

    async fn complete(&self, _term : &Arc<Terminal>, substring : String) -> Result<Vec<String>> {
        if substring.starts_with('a') {
            Ok(vec!["alpha".to_string(), "aloha".to_string(), "albatross".to_string()])
        } else {
            Ok(vec![])
        }
    }
}

pub async fn example_terminal() -> Result<()> {

    let cli = Arc::new(ExampleCli::new());

    workflow_log::pipe(Some(cli.clone()));

    let term = Arc::new(Terminal::try_new(cli,"$ ")?);
    term.init().await?;
    term.writeln("Terminal example");
    term.run().await?;

    Ok(())
}
