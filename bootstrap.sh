#!/usr/bin/env sh

if [ "$(expr substr $(uname -s) 1 5)" = "Linux" ]
then
    ./vendor/premake/linux/premake5 gmake2

elif [ "$(uname)" = "Darwin" ]
then
    ./vendor/premake/mac/premake5 xcode4
fi
