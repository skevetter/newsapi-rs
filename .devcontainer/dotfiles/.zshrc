setup_starship() {
    if [ -f $HOME/.config/starship/starship.toml ]; then
        export STARSHIP_CONFIG=$HOME/.config/starship/starship.toml
    fi

    if ! command -v starship &>/dev/null; then
        curl -sS https://starship.rs/install.sh | sh -s -- --yes
    fi

    eval "$(starship init "$(basename "${SHELL}")")"
}

main() {
    export HISTFILE=/cmd_history/.zsh_history

    setup_starship
}

main
