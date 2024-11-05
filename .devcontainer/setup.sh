#!/bin/bash

git config --global user.signingkey /home/vscode/.ssh/id_ed25519.pub
git config --global --add --bool push.autoSetupRemote true
echo 'autoload -Uz compinit && compinit' >> ~/.zshrc && . ~/.zshrc

sudo chown -R vscode:vscode ~/target

# foundryup
