load_profile() {
    if [ -f "$HOME/.profile" ]; then
        source "$HOME/.profile"
    fi
}

prepend_path() {
    local dir="$1"
    if [ -d "$dir" ] && [[ ":$PATH:" != *":$dir:"* ]]; then
        PATH="$dir:$PATH"
    fi
}

append_path() {
    local dir="$1"
    if [ -d "$dir" ] && [[ ":$PATH:" != *":$dir:"* ]]; then
        PATH="$PATH:$dir"
    fi
}

initialize_path() {
    append_path "${HOME}/bin"
    append_path "${HOME}/.local/bin"
    append_path "${HOME}/.cargo/bin"

    export PATH
}

omz_plugin_install() {
    local plugin
    local url
    local plugins_dir

    plugin=$(basename "$1")
    url="https://github.com/$1"
    plugins_dir="${ZSH_CUSTOM:-$HOME/.oh-my-zsh/custom}/plugins"

    if [ -d "${plugins_dir}/${plugin}" ]; then
        return
    fi

    mkdir -p "${plugins_dir}/${plugin}"

    if command -v git-lfs &>/dev/null; then
        git clone --depth=1 "$url" "${plugins_dir}/${plugin}"
    else
        archive="${url}/archive/master.tar.gz"
        curl -sSfL "$archive" | tar -xz -C "${plugins_dir}/${plugin}" --strip-components=1
    fi
}

load_oh_my_zsh() {
    local omz

    omz="${ZSH:-$HOME/.oh-my-zsh}"
    if [ ! -d "${omz}" ]; then
        sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"
    fi

    omz_plugin_install zsh-users/zsh-autosuggestions
    omz_plugin_install zsh-users/zsh-syntax-highlighting
    omz_plugin_install zsh-users/zsh-history-substring-search
    omz_plugin_install qoomon/zsh-lazyload

    ZSH_THEME=""

    plugins=(
        git
        docker
        zsh-autosuggestions
        zsh-syntax-highlighting
        zsh-history-substring-search
        zsh-lazyload
    )

    if [ -e "${omz}/oh-my-zsh.sh" ]; then
        zstyle ':omz:update' mode disabled
        source "${omz}/oh-my-zsh.sh"
    fi
}

main() {
    load_profile
    initialize_path
    load_oh_my_zsh

    zstyle ':completion:\*' menu select

    autoload bashcompinit && bashcompinit
    autoload -Uz compinit && compinit

    if command -v fzf &>/dev/null; then
        source <(fzf --zsh)
    fi
}

main
