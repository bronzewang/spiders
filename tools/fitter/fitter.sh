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
sudo apt install tree
sudo apt install build-essential
## rust环境
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
cp tools/fitter/config/cargo/config.toml ~/.cargo/
curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
## 安装vim
sudo apt install vim
## 安装及配置yazi
sudo snap install yazi --classic
## fonts emoj
sudo apt install fonts-firacode
## 安装及配置emacs
sudo apt install emacs
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
### http://192.168.3.1/cgi-bin/luci/|http://192.168.3.8/cgi-bin/luci/|https://192.168.3.10:8006/#v1:0:18:4:::::::|https://192.168.3.11:8006/#v1:0:18:4:::::::|http://192.168.3.20:5666/login|http://192.168.3.21:5666/|https://192.168.3.30/explore/repos|http://192.168.3.47:3000/?orgId=1&from=now-6h&to=now&timezone=browser|https://chat.deepseek.com/|https://fanyi.baidu.com/mtpe-individual/transText#/
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
~/.config/iamb/config.toml
## tokei
cargo binstall tokei
## remmina的默认配置
sudo apt install remmina remmina-plugin-rdp remmina-plugin-vnc
## 安装emoji字体
sudo apt install fonts-noto
### Ubuntu 24.04 需要这些字体
sudo apt update
sudo apt install -y \
    fonts-noto-color-emoji \
    fonts-noto-cjk \
    fonts-dejavu-core \
    fonts-symbola \
    fonts-jetbrains-mono
### 或者安装Nerd Fonts（包含图标字体）
cd /tmp
wget -O nerd-fonts.zip https://github.com/ryanoasis/nerd-fonts/releases/latest/download/JetBrainsMono.zip
unzip -d ~/.local/share/fonts nerd-fonts.zip
fc-cache -fv
### 配置文件首选项，自定义字体 nerd font,选择第一个
## 安装zed
curl -f https://zed.dev/install.sh | sh
## 安装rpi-imager
sudo apt install rpi-imager
## sqlite
sudo apt install sqlite3

# 桥接机
## 开启ssh服务, 理论上应该烧录镜像时开启
sudo systemctl enable ssh
sudo systemctl start ssh
## 更改主机名称
#!/bin/bash
# 保存為 change_hostname.sh，然後執行: sudo bash change_hostname.sh

# 設置新主機名
NEW_HOSTNAME="raspberrypi-new"

echo "正在將主機名更改為: $NEW_HOSTNAME"

# 停止可能衝突的服務
sudo systemctl stop avahi-daemon 2>/dev/null

# 1. 更新 /etc/hostname
echo "$NEW_HOSTNAME" | sudo tee /etc/hostname > /dev/null

# 2. 更新 /etc/hosts
if grep -q "127.0.1.1" /etc/hosts; then
    sudo sed -i "s/127.0.1.1.*/127.0.1.1\t$NEW_HOSTNAME/" /etc/hosts
else
    echo "127.0.1.1\t$NEW_HOSTNAME" | sudo tee -a /etc/hosts > /dev/null
fi

# 3. 使用 hostnamectl 設定
sudo hostnamectl set-hostname "$NEW_HOSTNAME"

# 4. 更新 Avahi (mDNS/Bonjour) 設定
if [ -f /etc/avahi/avahi-daemon.conf ]; then
    sudo sed -i "s/^#host-name=.*/host-name=$NEW_HOSTNAME/" /etc/avahi/avahi-daemon.conf
    sudo sed -i "s/^host-name=.*/host-name=$NEW_HOSTNAME/" /etc/avahi/avahi-daemon.conf
fi

# 5. 重新啟動相關服務
sudo systemctl restart systemd-hostnamed
sudo systemctl restart avahi-daemon 2>/dev/null
sudo systemctl restart dhcpcd 2>/dev/null

# 6. 更新當前 shell 的環境變數
export HOSTNAME="$NEW_HOSTNAME"

echo "========================================"
echo "主機名更改完成！"
echo "新主機名: $NEW_HOSTNAME"
echo ""
echo "驗證命令:"
echo "  hostname          # 顯示當前主機名"
echo "  hostnamectl       # 顯示詳細主機名資訊"
echo "  cat /etc/hostname # 檢查設定文件"
echo ""
echo "建議重新啟動系統使所有服務生效:"
echo "  sudo reboot"
echo "========================================"
## 加快网络访问设置，隧道和镜像
## 更新安装基本软件
## 安装最新的spiders环境
## 询问程序版本信息，dora安装信息
## 是否创建安装软件服务器
## systemd服务配置
