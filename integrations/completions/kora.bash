# Bash completion for Kora Backend Programming Language CLI (`kora`)

_kora_completions() {
    local cur prev words cword
    _init_completion || return

    local commands="check types ast tokens codegen build run bench db pkg create new init g generate help version --version --help"
    local db_actions="migrate sync seed truncate drop refresh"
    local pkg_actions="install add remove list publish"
    local gen_actions="feature f"

    if [[ $cword -eq 1 ]]; then
        COMPREPLY=($(compgen -W "$commands" -- "$cur"))
        return 0
    fi

    case "${words[1]}" in
        check|types|ast|tokens|codegen|build|run)
            # Complete .kora and .kr source files
            COMPREPLY=($(compgen -f -X '!*.kora' -- "$cur") $(compgen -f -X '!*.kr' -- "$cur"))
            return 0
            ;;
        db)
            if [[ $cword -eq 2 ]]; then
                COMPREPLY=($(compgen -W "$db_actions" -- "$cur"))
            else
                COMPREPLY=($(compgen -f -X '!*.kora' -- "$cur") $(compgen -f -X '!*.kr' -- "$cur"))
            fi
            return 0
            ;;
        pkg)
            if [[ $cword -eq 2 ]]; then
                COMPREPLY=($(compgen -W "$pkg_actions" -- "$cur"))
            fi
            return 0
            ;;
        init|create|new)
            COMPREPLY=($(compgen -d -- "$cur"))
            return 0
            ;;
        g|generate)
            if [[ $cword -eq 2 ]]; then
                COMPREPLY=($(compgen -W "$gen_actions" -- "$cur"))
            fi
            return 0
            ;;
    esac
}

complete -F _kora_completions kora

