use std::fs;
use std::path::PathBuf;

use kora_cli::{FrontendSession, format_type_report};

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("workspace root")
        .to_path_buf()
}

#[test]
fn verifies_hello_example_end_to_end() {
    let path = workspace_root().join("examples/01_hello_world/main.kora");
    let src = fs::read_to_string(&path).expect("read 01_hello_world/main.kora");
    let session = FrontendSession::check_source(path.to_str().unwrap(), src);

    assert!(
        !session.diagnostics.has_errors(),
        "01_hello_world/main.kora failed check:\n{}",
        session.diagnostics.render_all(&session.source_map, false)
    );

    let report = format_type_report(&session.types);
    assert!(report.contains("model Coordinates"));
    assert!(report.contains("enum OrderStatus"));
    assert!(report.contains("let developer: String"));
    assert!(report.contains("let finalAmount: Int"));
}

#[test]
fn verifies_users_api_example_end_to_end() {
    let path = workspace_root().join("examples/02_clean_architecture_users/main.kora");
    let src = fs::read_to_string(&path).expect("read 02_clean_architecture_users/main.kora");
    let session = FrontendSession::check_source(path.to_str().unwrap(), src);

    assert!(
        !session.diagnostics.has_errors(),
        "02_clean_architecture_users/main.kora failed check:\n{}",
        session.diagnostics.render_all(&session.source_map, false)
    );

    let report = format_type_report(&session.types);
    assert!(report.contains("model User"));
    assert!(report.contains("interface UserRepository"));
    assert!(report.contains("class UserService implements UserRepository"));
    assert!(report.contains("let app: App"));
}

#[test]
fn reports_rich_diagnostics_for_type_and_di_errors() {
    let bad_src = r#"
struct Item {
    id: Int
    title: String
}

class BadService {
    init(port: Int) {
    }
}

let broken: Int = "not an int"
let partial = Item { id: 1 }
"#;
    let session = FrontendSession::check_source("broken.kora", bad_src);
    assert!(session.diagnostics.has_errors());

    let codes: Vec<&str> = session.diagnostics.items().iter().map(|d| d.code).collect();
    assert!(
        codes.contains(&"E4012"),
        "expected DI check error E4012, got {codes:?}"
    );
    assert!(
        codes.contains(&"E4001"),
        "expected type mismatch E4001, got {codes:?}"
    );
    assert!(
        codes.contains(&"E4004"),
        "expected missing field E4004, got {codes:?}"
    );

    let rendered = session.diagnostics.render_all(&session.source_map, false);
    assert!(rendered.contains("broken.kora:"));
}

#[test]
fn verifies_null_supported_and_undefined_forbidden() {
    let valid_null_src = r#"
class NullCheck {
    fn check(opt: Option<String>) -> Bool {
        if opt == null {
            return false
        }
        return match opt {
            Some(v) => true,
            null => false,
        }
    }
}
"#;
    let valid_session = FrontendSession::check_source("null_check.kora", valid_null_src);
    assert!(
        !valid_session.diagnostics.has_errors(),
        "null_check failed check:\n{}",
        valid_session
            .diagnostics
            .render_all(&valid_session.source_map, false)
    );

    let bad_undef_src = r#"
fn bad() {
    let x = undefined
}
"#;
    let bad_session = FrontendSession::check_source("bad_undef.kora", bad_undef_src);
    assert!(bad_session.diagnostics.has_errors());
    let codes: Vec<&str> = bad_session
        .diagnostics
        .items()
        .iter()
        .map(|d| d.code)
        .collect();
    assert!(
        codes.contains(&"E2005"),
        "expected E2005 for undefined, got {codes:?}"
    );
}

#[test]
fn verifies_all_numbered_catalog_examples_pass_frontend_check() {
    let root = workspace_root();
    let examples_dir = root.join("examples");
    let mut checked_count = 0;

    for entry in fs::read_dir(&examples_dir).expect("read examples dir") {
        let entry = entry.expect("valid entry");
        let path = entry.path();
        if path.is_dir() {
            let main_kora = path.join("main.kora");
            if main_kora.exists() {
                let src = fs::read_to_string(&main_kora).expect("read main.kora");
                let session = FrontendSession::check_source(main_kora.to_str().unwrap(), src);
                assert!(
                    !session.diagnostics.has_errors(),
                    "Example at {} failed check:\n{}",
                    main_kora.display(),
                    session.diagnostics.render_all(&session.source_map, false)
                );
                checked_count += 1;
            }
        }
    }

    assert!(
        checked_count >= 8,
        "Expected at least 8 catalog examples, checked {checked_count}"
    );
}
