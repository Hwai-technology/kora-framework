use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

use kora_cli::{FrontendSession, format_type_report, scaffold_project};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() || args[0] == "--help" || args[0] == "-h" || args[0] == "help" {
        print_usage();
        return ExitCode::SUCCESS;
    }

    if args[0] == "--version" || args[0] == "-V" {
        println!("kora {}", env!("CARGO_PKG_VERSION"));
        return ExitCode::SUCCESS;
    }

    let subcommand = args[0].as_str();
    match subcommand {
        "check" => {
            let Some(path) = args.get(1) else {
                eprintln!("error: missing `<file.kora>` argument for `kora check`");
                return ExitCode::FAILURE;
            };
            cmd_check(path)
        }
        "types" => {
            let Some(path) = args.get(1) else {
                eprintln!("error: missing `<file.kora>` argument for `kora types`");
                return ExitCode::FAILURE;
            };
            cmd_types(path)
        }
        "ast" => {
            let Some(path) = args.get(1) else {
                eprintln!("error: missing `<file.kora>` argument for `kora ast`");
                return ExitCode::FAILURE;
            };
            cmd_ast(path)
        }
        "tokens" => {
            let Some(path) = args.get(1) else {
                eprintln!("error: missing `<file.kora>` argument for `kora tokens`");
                return ExitCode::FAILURE;
            };
            cmd_tokens(path)
        }
        "init" => {
            let Some(dir) = args.get(1) else {
                eprintln!("error: missing `<directory>` argument for `kora init`");
                return ExitCode::FAILURE;
            };
            match scaffold_project(Path::new(dir)) {
                Ok(()) => {
                    println!("✓ Initialized new Kora project in `{dir}`");
                    println!("  Next step: kora check {dir}/src/main.kora");
                    ExitCode::SUCCESS
                }
                Err(err) => {
                    eprintln!("error: {err}");
                    ExitCode::FAILURE
                }
            }
        }
        unknown => {
            eprintln!("error: unknown command `{unknown}`\n");
            print_usage();
            ExitCode::FAILURE
        }
    }
}

fn read_source_or_exit(path: &str) -> Result<String, ExitCode> {
    match fs::read_to_string(path) {
        Ok(s) => Ok(s),
        Err(err) => {
            eprintln!("error: failed to read `{path}`: {err}");
            Err(ExitCode::FAILURE)
        }
    }
}

fn cmd_check(path: &str) -> ExitCode {
    let Ok(src) = read_source_or_exit(path) else {
        return ExitCode::FAILURE;
    };
    let session = FrontendSession::check_source(path, src);
    if session.diagnostics.has_errors() {
        eprint!(
            "{}",
            session.diagnostics.render_all(&session.source_map, true)
        );
        eprintln!(
            "\n✗ Check failed for `{path}` with {} error(s).",
            session.diagnostics.error_count()
        );
        ExitCode::FAILURE
    } else {
        println!(
            "✓ Checked `{path}` — 0 errors ({} item(s), {} inferred binding(s)).",
            session.ast.items.len(),
            session.types.bindings.len()
        );
        ExitCode::SUCCESS
    }
}

fn cmd_types(path: &str) -> ExitCode {
    let Ok(src) = read_source_or_exit(path) else {
        return ExitCode::FAILURE;
    };
    let session = FrontendSession::check_source(path, src);
    if session.diagnostics.has_errors() {
        eprint!(
            "{}",
            session.diagnostics.render_all(&session.source_map, true)
        );
        return ExitCode::FAILURE;
    }
    print!("{}", format_type_report(&session.types));
    ExitCode::SUCCESS
}

fn cmd_ast(path: &str) -> ExitCode {
    let Ok(src) = read_source_or_exit(path) else {
        return ExitCode::FAILURE;
    };
    let session = FrontendSession::check_source(path, src);
    if session.diagnostics.has_errors() {
        eprint!(
            "{}",
            session.diagnostics.render_all(&session.source_map, true)
        );
        return ExitCode::FAILURE;
    }
    println!("{:#?}", session.ast);
    ExitCode::SUCCESS
}

fn cmd_tokens(path: &str) -> ExitCode {
    let Ok(src) = read_source_or_exit(path) else {
        return ExitCode::FAILURE;
    };
    let (sm, tokens, diagnostics) = FrontendSession::lex_only(path, src);
    if diagnostics.has_errors() {
        eprint!("{}", diagnostics.render_all(&sm, true));
        return ExitCode::FAILURE;
    }
    let file = sm.get_file(kora_span::FileId(0)).expect("file loaded");
    for tok in &tokens {
        let lc = file.line_col(tok.span.start);
        let nl = if tok.had_newline_before { " [nl]" } else { "" };
        println!("{:>3}:{:<3} {:?}{}", lc.line, lc.col, tok.kind, nl);
    }
    ExitCode::SUCCESS
}

fn print_usage() {
    println!(
        "Kora Compiler & Toolchain ({})\n\
         Write Less. Build Faster. Run Smarter.\n\n\
         USAGE:\n\
         \x20 kora <COMMAND> [ARGS]\n\n\
         COMMANDS:\n\
         \x20 check <file.kora>    Parse, resolve symbols, and statically type-check a Kora file\n\
         \x20 types <file.kora>    Inspect verified models, DI classes, functions, and inferred types\n\
         \x20 ast <file.kora>      Dump the parsed Abstract Syntax Tree\n\
         \x20 tokens <file.kora>   Inspect the lexical token stream and line/column spans\n\
         \x20 init <dir>           Scaffold a new Kora project with kora.toml and src/main.kora",
        env!("CARGO_PKG_VERSION")
    );
}
