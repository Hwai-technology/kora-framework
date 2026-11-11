# Fish completions for Kora compiler & CLI

# Disable file completions by default for the first token
complete -c kora -f

# Global flags
complete -c kora -s h -l help -d "Display help message"
complete -c kora -s v -l version -d "Display Kora version"

# Top-level commands
complete -c kora -n "__fish_use_subcommand" -a run -d "Compile and execute a Kora program directly or project entrypoint"
complete -c kora -n "__fish_use_subcommand" -a watch -d "Watch for file modifications and re-compile instantaneously"
complete -c kora -n "__fish_use_subcommand" -a w -d "Short alias for watch"
complete -c kora -n "__fish_use_subcommand" -a check -d "Parse and verify architectural purity without codegen"
complete -c kora -n "__fish_use_subcommand" -a build -d "Compile Kora source to an optimized native binary"
complete -c kora -n "__fish_use_subcommand" -a types -d "Run static type inference and interface verification"
complete -c kora -n "__fish_use_subcommand" -a codegen -d "Generate intermediate Rust code into .kora-build"
complete -c kora -n "__fish_use_subcommand" -a fmt -d "Format Kora source files according to language standards"
complete -c kora -n "__fish_use_subcommand" -a init -d "Initialize a new Kora project in the current directory"
complete -c kora -n "__fish_use_subcommand" -a create -d "Create a new Kora project with Clean Architecture"
complete -c kora -n "__fish_use_subcommand" -a new -d "Alias for create"
complete -c kora -n "__fish_use_subcommand" -a generate -d "Generate a domain feature or component"
complete -c kora -n "__fish_use_subcommand" -a g -d "Short alias for generate"
complete -c kora -n "__fish_use_subcommand" -a bench -d "Run high-performance async concurrency benchmark"
complete -c kora -n "__fish_use_subcommand" -a lsp -d "Start Language Server Protocol (LSP) daemon"
complete -c kora -n "__fish_use_subcommand" -a version -d "Display Kora compiler version"

# File suggestions for commands that accept source files
complete -c kora -n "__fish_seen_subcommand_from run check build types codegen fmt" -k -a "(__fish_complete_suffix .kora .kr)"

# Subcommands for generate / g
complete -c kora -n "__fish_seen_subcommand_from g generate" -a feature -d "Scaffold a Clean Architecture domain feature"

# Flags for fmt
complete -c kora -n "__fish_seen_subcommand_from fmt" -l check -d "Check formatting without modifying files"

