use std::{env, fs, process::ExitCode};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let sub = args.next().ok_or("missing subcommand: use `tokens`, `parse`, `check`, `hir`, or `mir`")?;
    let path = args.next().ok_or("missing input path")?;
    let input = fs::read_to_string(path)?;

    match sub.as_str() {
        "tokens" => {
            let tokens = ails_lexer::lex(&input)?;
            for token in tokens {
                println!("{:?}", token);
            }
        }
        "parse" => {
            let module = ails_parser::parse_module(&input)?;
            println!("{:#?}", module);
        }
        "check" => {
            let module = ails_parser::parse_module(&input)?;
            ails_typecheck::check_module(&module)?;
            println!("OK: type check passed");
        }
        "hir" => {
            let module = ails_parser::parse_module(&input)?;
            ails_typecheck::check_module(&module)?;
            let hir = ails_hir::lower_module(&module);
            println!("{:#?}", hir);
        }
        "mir" => {
            let module = ails_parser::parse_module(&input)?;
            ails_typecheck::check_module(&module)?;
            let hir = ails_hir::lower_module(&module);
            let mir = ails_mir::lower_module(&hir);
            println!("{:#?}", mir);
        }
        _ => return Err(format!("unknown subcommand `{sub}`").into()),
    }

    Ok(())
}


// NOTE(v0.23 draft):
// A future `validate-mir` subcommand should:
// 1. parse source
// 2. typecheck
// 3. lower to HIR
// 4. lower to MIR
// 5. run ails_mir::validate_module_structure(&mir)
// This comment is kept until the validated command is wired in.
