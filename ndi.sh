PLATFORM=$1
TYPE=$2
ARCH=$3
VERSION=$4

url=https://us.download.nvidia.com/${PLATFORM}/${TYPE}-${ARCH}/${VERSION}/NVIDIA-${TYPE}-${ARCH}-${VERSION}.run
wget $url