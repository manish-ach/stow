#
# ~/.bashrc
#

# If not running interactively, don't do anything
[[ $- != *i* ]] && return

alias ls='ls --color=auto'
alias grep='grep --color=auto'
PS1='[\u@\h \W]\$ '

# bun
export BUN_INSTALL="$HOME/.bun"
export PATH=$BUN_INSTALL/bin:$PATH
export PATH=$PATH:$HOME/go/bin
export PATH="$HOME/.local/bin:$PATH"

# starship
eval "$(starship init bash)"

#zoxide
eval "$(zoxide init bash)"
alias cd="z"
