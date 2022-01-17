#!/bin/sh

NPROC=$(cat /proc/cpuinfo | awk '/^processor/{print $3}' | wc -l)
empath=$(echo $(which emsdk) | sed 's,/*[^/]\+/*$,,')
EM_TOOLCHAIN_FILE=$empath/upstream/emscripten/cmake/Modules/Platform/Emscripten.cmake
CUR_DIR=${PWD}
SYS_DIR=${CUR_DIR}/sys-build
INSTALL_DIR=${SYS_DIR}/usr
echo "EM_TOOLCHAIN_FILE location: "
echo $EM_TOOLCHAIN_FILE
echo "current directory: "
echo $CUR_DIR

create_dir() {
    rm -rf ${SYS_DIR}
    mkdir -p ${SYS_DIR}
}

build_lept() {
    cd ${SYS_DIR}
    rm -rf lept/
    mkdir lept
    cd lept/
    emmake cmake ${CUR_DIR}/sys/leptonica \
        -DCMAKE_INSTALL_PREFIX=${INSTALL_DIR} \
        -DCMAKE_TOOLCHAIN_FILE=${EM_TOOLCHAIN_FILE} \
        -D BUILD_SHARED_LIBS=OFF
    emmake make install -j${NPROC}
}

build_tess() {
    cd ${SYS_DIR}
    rm -rf tess/
    mkdir tess
    cd tess/
    emmake cmake ${CUR_DIR}/sys/tesseract \
        -DCMAKE_INSTALL_PREFIX=${INSTALL_DIR} \
        -DCMAKE_TOOLCHAIN_FILE=${EM_TOOLCHAIN_FILE} \
        -DLeptonica_DIR=${SYS_DIR}/lept \
        -D BUILD_SHARED_LIBS=OFF
    # emmake make install -j${NPROC}
}

build_lept_sys() {
    cd ${SYS_DIR}
    rm -rf lept/
    mkdir lept
    cd lept/
    cmake ${CUR_DIR}/sys/leptonica -DCMAKE_INSTALL_PREFIX=${INSTALL_DIR}
    make install -j${NPROC}
}

build_tess_sys() {
    cd ${SYS_DIR}
    rm -rf tess/
    mkdir tess
    cd tess/
    cmake ${CUR_DIR}/sys/tesseract -DCMAKE_INSTALL_PREFIX=${INSTALL_DIR}
    make install -j${NPROC}
}

main() {
    create_dir
    # build_lept
    # build_tess
    build_lept_sys
    build_tess_sys
}

main "$@"