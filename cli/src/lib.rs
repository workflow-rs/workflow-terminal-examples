use std::sync::{Arc,Mutex};
use std::time::Duration;
use async_trait::async_trait;
use workflow_terminal::Terminal;
// use workflow_terminal::Options;
use workflow_terminal::Result;
use workflow_terminal::CliResult;
use workflow_terminal::Cli;
use workflow_terminal::parse;
use workflow_log::*;


struct ExampleCli {
    // term : Arc<Mutex<Option<Arc<Terminal>>>>
}

impl ExampleCli {

    fn new() -> Self {
        ExampleCli {
            // term : Arc::new(Mutex::new(None))
        }
    }

    // fn term(&self) -> Option<Arc<Terminal>> {
    //     if let Some(term) = self.term.lock().unwrap().as_ref() { //; //.unwrap().clone()
    //         Some(term.clone())
    //     } else {
    //         None
    //     }
    // }

}

// impl workflow_log::Sink for ExampleCli {
//     fn write(&self, _level:Level, args : &std::fmt::Arguments<'_>) -> bool {

//         // note, the terminal may not be initialized
//         // if workflow_log::pipe() is bound before the
//         // Terminal::init() is complete.
//         if let Some(term) = self.term() {
//             term.writeln(args.to_string());
//             // true to disable further processing (no further output is made)
//             true
//         } else {
//             // false for default log output handling (print to stdout or web console)
//             false
//         }
//     }
// }

#[async_trait]
impl Cli for ExampleCli {

    fn init(&self, term : &Arc<Terminal>) -> Result<()> {
        // *self.term.lock().unwrap() = Some(term.clone());
        Ok(())
    }

    async fn digest(&self, term : Arc<Terminal>, cmd: String) -> CliResult<()> {
        let argv = parse(&cmd);
        match argv[0].as_str() {
            // "help" => {
            //     term.writeln("\n\rCommands:\n\r\thelp - this list\n\r\thello - simple terminal text output\n\r\ttest - workflow_log!() macro test\n\r\texit - exits example (native only)\n\r");
            // },
            // "hello" => {
            //     term.writeln("hello back to you!");
            // },
            // "test" => {
            //     log_trace!("log_trace!() macro test");
            // },
            "sleep" => {
                workflow_core::task::sleep(Duration::from_secs(10)).await; 
            },
            // "ask" => {
            //     let text = term.ask(false,"Enter something:").await?;
            //     log_info!("You have entered something: {}", text);
            // },
            // "exit" => {
            //     term.writeln("bye!");
            //     term.exit();
            // },
            _ => {
                return Err(format!("command not found: {}", cmd).into())
            }
        }

        Ok(())
    }

    async fn complete(&self, _term : Arc<Terminal>, cmd : String) -> CliResult<Vec<String>> {
        let argv = parse(&cmd);
        if argv.len() == 0 { return Ok(vec![]) }
        let last = argv.last().unwrap();
        if last.starts_with('a') {
            Ok(vec!["alpha".to_string(), "aloha".to_string(), "albatross".to_string()])
        } else {
            Ok(vec![])
        }
    }
}

pub async fn example_terminal() -> Result<()> {

    let cli = Arc::new(ExampleCli::new());
    let term = Arc::new(Terminal::try_new(cli.clone(),"$ ")?);
    term.init().await?;

    // IMPORTANT: if redirecting workflow_log, using pipe()
    // the handler must be installed after Terminal::init()
    // is invoked.
    // workflow_log::pipe(Some(cli.clone()));

    term.writeln("Terminal example (type 'help' for list of commands)");
    term.run().await?;

    Ok(())
}
