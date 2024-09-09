# Platform
# Options:
# - XFree86
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
# 
# VERSION
# Options:
#

PLATFORM=$1
TYPE=$2
ARCH=$3
VERSION=$4

FILE=NVIDIA-${TYPE}-${ARCH}-${VERSION}.run
URL=https://us.download.nvidia.com/${PLATFORM}/${TYPE}-${ARCH}/${VERSION}/${FILE}
wget $URL