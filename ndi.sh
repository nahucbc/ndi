#!/bin/sh

# Platform
# Options:
# - XFree86
# - Solaris
#
# TYPE
# Options:
# - Linux
# - FreeBSD
# 
# ARCH
# Options:
# - x86_64
# - x86
# - aarch64
# - arm
#
# VERSION
# Options:
#
# Linux x86_64/AMD64/EM64T
#
# Latest Legacy GPU version (470.xx series): 470.256.02
# Latest Legacy GPU version (390.xx series): 390.157
# Latest Legacy GPU version (340.xx series): 340.108
# Latest Legacy GPU version (304.xx series): 304.137
# Latest Legacy GPU Version (71.86.xx series): 71.86.15
# Latest Legacy GPU Version (96.43.xx series): 96.43.23
# Latest Legacy GPU Version (173.14.xx series): 173.14.39
#
# Linux x86/IA32
#
# Latest Legacy GPU version (390.xx series): 390.157
# Latest Legacy GPU version (340.xx series): 340.108
# Latest Legacy GPU version (304.xx series): 304.137
# Latest Legacy GPU version (71.86.xx series): 71.86.15
# Latest Legacy GPU version (96.43.xx series): 96.43.23
# Latest Legacy GPU version (173.14.xx series): 173.14.39
#
# Linux aarch64
#
# Latest Legacy GPU version (470.xx series): 470.256.02
#
# Linux 32-bit ARM
#
# Latest Legacy GPU version (390.xx series): 390.157
#
# FreeBSD x86
#
# Latest Legacy GPU version (390.xx series): 390.157
# Latest Legacy GPU version (340.xx series): 340.108
# Latest Legacy GPU version (304.xx series): 304.137
# Latest Legacy GPU Version (71.86.xx series): 71.86.15
# Latest Legacy GPU Version (96.43.xx series): 96.43.23
# Latest Legacy GPU Version (173.14.xx series): 173.14.39
#
# FreeBSD x64
#
# Latest Legacy GPU version (470.xx series): 470.256.02
# Latest Legacy GPU version (390.xx series): 390.157
# Latest Legacy GPU version (340.xx series): 340.108
# Latest Legacy GPU version (304.xx series): 304.137
#
# Solaris x64/x86
#
# Latest Legacy GPU version (470.xx series): 470.256.02
# Latest Legacy GPU version (390.xx series): 390.157
# Latest Legacy GPU version (340.xx series): 340.108
# Latest Legacy GPU version (304.xx series): 304.137
# Latest Legacy GPU version (96.43.xx series): 96.43.23
# Latest Legacy GPU version (173.14.xx series): 173.14.39

if [ "$(uname --kernel-name)" = "Linux" ]; then PLATFORM="XFree86" TYPE="Linux"

elif [ "$(uname --kernel-name)" = "FreeBSD" ]; then PLATFORM="XFree86" TYPE="FreeBSD"

elif [ "$(uname --kernel-name)" = "SunOS" ]; then PLATFORM="Solaris"

fi

if [ "$(uname --machine)" = "x86_64" ]; then ARCH="x86_64"

elif [ "$(uname --machine)" = "i686" ]; then ARCH="x86"

elif [ "$(uname --machine)" = "aarch64" ]; then ARCH="aarch64"

elif [ "$(uname --machine)" = "armv7l" ]; then ARCH="arm"

fi

INSTALLER_ARGS="--rebuild-initramfs --silent "

while [ "$#" -gt 0 ]; do
    case "$1" in
        -P|--platform)
            shift  
            case "$1" in
                Linux)
                    PLATFORM="XFree86"
                    TYPE="Linux"
                    ;;
                FreeBSD)
                    PLATFORM="XFree86"
                    TYPE="FreeBSD"
                    ;;
                SunOS|Solaris)
                    PLATFORM="Solaris"
                    ;;
                *)
                    echo "Invalid platform option"
                    echo "Options can be: Linux, FreeBSD, SunOS or Solaris"
                    exit 1
                    ;;
            esac
            ;;
        -A|--arch)
            shift  
            ARCH="$1"
            ;;
        -V|--version)
            shift  
            VERSION="$1"
            ;;
        --open)
            INSTALLER_ARGS="$INSTALLER_ARGS--kernel-module-type=open "
            ;;
        --propietary)
            INSTALLER_ARGS="$INSTALLER_ARGS--kernel-module-type=propietary "
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
    shift  
done
if [ -z "${VERSION}" ]; then
    echo "Version is not declared"
    echo "Declare it with -V or --version"
    echo "For example: --version 470.256.02"
    exit
fi

if [ -z "${PLATFORM}" ]; then
    echo "Failed to detect PLATFORM"
    echo "Use instead -P or --platform "
    echo "For example: --platform linux"
    exit
fi

if [ -z "${ARCH}" ]; then
    echo "Failed to detect ARCH"
    echo "Use instead -A or --arch"
    echo "For example: --arch x86_64"
    exit
fi

if [ "$ARCH " = "arm" ]; then
    FILE=NVIDIA-${TYPE}-armv7l-gnueabihf-${VERSION}.run
    URL=https://us.download.nvidia.com/${PLATFORM}/${TYPE}-x86-ARM/${VERSION}/${FILE}

elif [ "$PLATFORM" = "Solaris" ]; then
    FILE=NVIDIA-$PLATFORM-x86-${VERSION}.run
    URL=https://us.download.nvidia.com/solaris/${VERSION}/${FILE}
else
    FILE=NVIDIA-${TYPE}-${ARCH}-${VERSION}.run
    URL=https://us.download.nvidia.com/${PLATFORM}/${TYPE}-${ARCH}/${VERSION}/${FILE}
fi

wget -c $URL
chmod +x $FILE
./${FILE} $INSTALLER_ARGS
rm $FILE