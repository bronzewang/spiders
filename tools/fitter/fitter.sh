#!/bin/env bash
#
# spiders的宿主机、桥接机的部署安装统一脚本
#

# 当这些设备系统安装完成之后，就使用curl获取这个脚本并根据不同的设备环境自动选择执行不同的部署动作

# 选择机器类型
## 宿主机，桥接机
## 设置机器名称，从而产生IP地址

# 宿主机
## set up software mirror
## 通过设置/etc/apt来更改软件镜像
sudo cp /etc/apt/sources.list.d/ubuntu.sources /etc/apt/sources.list.d/ubuntu.sources.bak
sudo cp ubuntu/ubuntu.sources /etc/apt/sources.list.d/

sudo apt update
## 配置git
sudo apt install git
git config --global user.email "cuprumwz@foxmial.com"
git config --global user.name "Bronze Wang"

## 输入法
sudo apt install fcitx5 fcitx5-chinese-addons fcitx5-frontend-gtk4 fcitx5-frontend-gtk3 fcitx5-frontend-gtk2 fcitx5-frontend-qt5
sudo apt install fcitx5-pinyin

## 加快网络访问设置，隧道和镜像
## 更新安装基本软件
## rust环境
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
cp tools/fitter/config/cargo/config.toml ~/.cargo/
curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
## 安装vim
sudo apt install vim
## 安装及配置yazi
## 安装及配置emacs
## 安装及配置helix
https://github.com/helix-editor/helix/releases/download/25.07.1/helix-25.07.1-x86_64.AppImage
sudo apt install -f helix_25.7.1-1_amd64.deb
cargo binstall helix
## 安装及配置zellij
cargo binstall zellij
sudo apt install xclip
## 安装trans-shell
sudo apt install translate-shell
## 配置浏览器的默认网址
## lazygit
##//sudo apt install lazygit   //ubuntu25.10
LAZYGIT_VERSION=$(curl -s "https://api.github.com/repos/jesseduffield/lazygit/releases/latest" | \grep -Po '"tag_name": *"v\K[^"]*')
curl -Lo lazygit.tar.gz "https://github.com/jesseduffield/lazygit/releases/download/v${LAZYGIT_VERSION}/lazygit_${LAZYGIT_VERSION}_Linux_x86_64.tar.gz"
tar xf lazygit.tar.gz lazygit
sudo install lazygit -D -t /usr/local/bin/
## gitu
cargo binstall gitu
## shellharden
cargo binstall shellharden
## 安装aichat,并加入agent配自
cargo binstall aichat
## iamb
cargo binstall iamb
## remmina的默认配置

# 桥接机
## 加快网络访问设置，隧道和镜像
## 更新安装基本软件
## 安装最新的spider环境

