#!/bin/bash -xe

HOME_PATH=$PWD
TARGET_PATH=$PWD"/build"
OS_PATH=$TARGET_PATH"/wasabi"
APP_NAME="saba"
MAKEFILE_PATH=$HOME_PATH"/Makefile"

# build ディレクトリの作成
if [ -d $TARGET_PATH ]
then
    echo $TARGET_PATH" exist"
else
    echo $TARGET_PATH" doesn't exist"
    mkdir $TARGET_PATH
fi

# WasabiOS をダウンロードする
if [ -d $OS_PATH ]
then
    echo $OS_PATH" exists"
    echo "pulling new changes..."
    cd $OS_PATH
    git pull origin for_saba
else
    echo $OS_PATH" doesn't exist"
    echo "cloning wasabi project..."
    cd $TARGET_PATH
    git clone --branch for_saba git@github.com:hikalium/wasabi.git
fi

cd $HOME_PATH

# Makefile のダウンロード
if [ ! -f $MAKEFILE_PATH ]; then
    echo "downloading Makefile..."
    curl -O https://raw.githubusercontent.com/hikalium/wasabi/for_saba/external_app_template/Makefile
fi

make build
$OS_PATH/scripts/run_with_app.sh ./target/x86_64-unknown-none/release/$APP_NAME
