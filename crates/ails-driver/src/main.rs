use std::{env, fs, process::ExitCode};

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<ExitCode, Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let sub = args.next().ok_or(
        "missing subcommand: use `tokens`, `parse`, `check`, `hir`, `mir`, or `validate-mir-structure`",
    )?;
    let path = args.next().ok_or("missing input path")?;
    let input = fs::read_to_string(path)?;

    match sub.as_str() {
        "tokens" => {
            let tokens = ails_lexer::lex(&input)?;
            for token in tokens {
                println!("{:?}", token);
            }
            Ok(ExitCode::SUCCESS)
        }
        "parse" => {
            let module = ails_parser::parse_module(&input)?;
            println!("{:#?}", module);
            Ok(ExitCode::SUCCESS)
        }
        "check" => {
            let module = ails_parser::parse_module(&input)?;
            ails_typecheck::check_module(&module)?;
            println!("OK: type check passed");
            Ok(ExitCode::SUCCESS)
        }
        "hir" => {
            let module = ails_parser::parse_module(&input)?;
            ails_typecheck::check_module(&module)?;
            let hir = ails_hir::lower_module(&module);
            println!("{:#?}", hir);
            Ok(ExitCode::SUCCESS)
        }
        "mir" => {
            let module = ails_parser::parse_module(&input)?;
            ails_typecheck::check_module(&module)?;
            let hir = ails_hir::lower_module(&module);
            let mir = ails_mir::lower_module(&hir);
            println!("{:#?}", mir);
            Ok(ExitCode::SUCCESS)
        }
        "validate-mir-structure" => {
            let module = ails_parser::parse_module(&input)?;
            ails_typecheck::check_module(&module)?;
            let hir = ails_hir::lower_module(&module);
            let mir = ails_mir::lower_module(&hir);
            let reports = ails_mir::validate_module_structure(&mir);

            let mut any_failure = false;
            for report in &reports {
                println!("function: {}", report.function_name);
                println!("  success: {}", report.success);
                println!("  block_count: {}", report.block_count);
                println!("  unreachable_blocks: {:?}", report.unreachable_blocks);
                if report.errors.is_empty() {
                    println!("  errors: []");
                } else {
                    println!("  errors:");
                    for err in &report.errors {
                        println!("    - {:?}", err.kind);
                    }
                }
                if !report.success {
                    any_failure = true;
                }
            }

            Ok(if any_failure {
                ExitCode::FAILURE
            } else {
                ExitCode::SUCCESS
            })
        }
        _ => Err(format!("unknown subcommand `{sub}`").into()),
    }
}
