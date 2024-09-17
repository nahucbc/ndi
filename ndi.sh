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

if [[ "$(uname --kernel-name)" == "Linux" ]]; then PLATFORM="XFree86" TYPE="Linux"

elif [[ "$(uname --kernel-name)" == "FreeBSD" ]]; then PLATFORM="XFree86" TYPE="FreeBSD"

elif [[ "$(uname --kernel-name)" == "SunOS" ]]; then PLATFORM="Solaris"

else 
    echo "failed to detect PLATFORM"
    echo "use instead -P or --platform "
    echo "for example: --platform linux"
    exit
fi

if [[ "$(uname --machine)" == "x86_64" ]]; then ARCH="x86_64"

elif [[ "$(uname --machine)" == "i686" ]]; then ARCH="x86"

elif [[ "$(uname --machine)" == "aarch64" ]]; then ARCH="aarch64"

elif [[ "$(uname --machine)" == "armv7l" ]]; then ARCH="arm"

else 
    echo "failed to detect ARCH"
    echo "use instead -A or --arch"
    echo "for example: --arch x86_64"
    exit
fi

args=("$@")

for (( i=0; i<=$#; i++ )); do

    if [[ "${args[$i]}" == "-P" ]] || [[ "${args[$i]}" == "--platform" ]]; then PLATFORM="${args[$i + 1]}"

    elif [[ "${args[$i]}" == "-A" ]] || [[ "${args[$i]}" == "--arch" ]]; then ARCH="${args[$i + 1]}" 

    elif [[ "${args[$i]}" == "-V" ]] || [[ "${args[$i]}" == "--version" ]]; then  VERSION="${args[$i + 1]}" 

    fi
done

FILE=NVIDIA-${TYPE}-${ARCH}-${VERSION}.run
URL=https://us.download.nvidia.com/${PLATFORM}/${TYPE}-${ARCH}/${VERSION}/${FILE}

if [[ "$ARCH " == "arm" ]]; then
    FILE=NVIDIA-${TYPE}-armv7l-gnueabihf-${VERSION}.run
    URL=https://us.download.nvidia.com/${PLATFORM}/${TYPE}-x86-ARM/${VERSION}/${FILE}

elif [[ "$PLATFORM" == "Solaris" ]]; then
    FILE=NVIDIA-$PLATFORM-x86-${VERSION}.run
    URL=https://us.download.nvidia.com/solaris/${VERSION}/${FILE}
fi
wget $URL