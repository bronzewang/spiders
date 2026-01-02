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
## 安装vim
sudo apt install vim
## 安装及配置yazi
## 安装及配置emacs
## 安装及配置helix
https://github.com/helix-editor/helix/releases/download/25.07.1/helix-25.07.1-x86_64.AppImage
sudo apt install -f helix_25.7.1-1_amd64.deb
## 安装及配置zellij
## 安装trans-shell
sudo apt install translate-shell
## 配置浏览器的默认网址
## lazygit
## shellharden
## 安装aichat,并加入agent配自

# 桥接机
## 加快网络访问设置，隧道和镜像
## 更新安装基本软件
## 安装最新的spider环境

