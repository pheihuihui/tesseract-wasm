#!/bin/sh

NPROC=$(cat /proc/cpuinfo | awk '/^processor/{print $3}' | wc -l)
empath=$(echo $(which emsdk) | sed 's,/*[^/]\+/*$,,')
EM_TOOLCHAIN_FILE=$empath/upstream/emscripten/cmake/Modules/Platform/Emscripten.cmake
CUR_DIR=${PWD}
SYS_BUILD_DIR=${CUR_DIR}/build
INSTALL_DIR=${SYS_BUILD_DIR}/usr
LIB_DIR=${INSTALL_DIR}/lib
INC_DIR=${INSTALL_DIR}/include
ZLIB_LIB=${LIB_DIR}/libz.a
JPEG_LIB=${LIB_DIR}/libjpeg.a
PNG_LIB=${LIB_DIR}/libpng.a
TIFF_LIB=${LIB_DIR}/libtiff.a
LIBM_LIB=${LIB_DIR}/libopenlibm.a
echo "EM_TOOLCHAIN_FILE location: "
echo $EM_TOOLCHAIN_FILE
echo "current directory: "
echo $CUR_DIR

create_dir() {
    rm -rf ${SYS_BUILD_DIR}
    mkdir -p ${SYS_BUILD_DIR}
    mkdir -p ${INSTALL_DIR}
}

build_zlib() {
    cd ${SYS_BUILD_DIR}
    rm -rf zlib/
    mkdir zlib/
    cd zlib/
    emmake cmake ${CUR_DIR}/sys/zlib \
        -DCMAKE_INSTALL_PREFIX=${INSTALL_DIR} \
        -DCMAKE_TOOLCHAIN_FILE=${EM_TOOLCHAIN_FILE} \
        -D BUILD_SHARED_LIBS=OFF
    emmake make install -j${NPROC}
}

build_libm() {
    cd ${CUR_DIR}/sys/openlibm
    emmake make -j${NPROC}
    cp libopenlibm.a ${LIB_DIR}/
}

build_libjpeg() {
    cd ${SYS_BUILD_DIR}
    rm -rf libjpeg/
    mkdir libjpeg/
    cd libjpeg/
    emmake cmake ${CUR_DIR}/sys/libjpeg \
        -DCMAKE_INSTALL_PREFIX=${INSTALL_DIR} \
        -DCMAKE_TOOLCHAIN_FILE=${EM_TOOLCHAIN_FILE} \
        -D BUILD_SHARED_LIBS=OFF
    emmake make install -j${NPROC}
}

build_libpng() {
    cd ${SYS_BUILD_DIR}
    rm -rf libpng/
    mkdir libpng/
    cd libpng/
    emmake cmake ${CUR_DIR}/sys/libpng \
        -DCMAKE_INSTALL_PREFIX=${INSTALL_DIR} \
        -DCMAKE_TOOLCHAIN_FILE=${EM_TOOLCHAIN_FILE} \
        -D ZLIB_LIBRARY=${ZLIB_LIB} \
        -D ZLIB_INCLUDE_DIR=${INC_DIR} \
        -D M_LIBRARY:PATH=${LIBM_LIB} \
        -D PNG_STATIC=ON \
        -D PNG_SHARED=OFF \
        -D PNG_TESTS=NO
    emmake make install -j${NPROC}
}

build_libtiff() {
    cd ${CUR_DIR}/sys/libtiff
    sh autogen.sh
    emconfigure ./configure --prefix=${INSTALL_DIR} --disable-shared
    emmake make install -j${NPROC}
}

build_leptonica() {
    cd ${SYS_BUILD_DIR}
    rm -rf leptonica/
    mkdir leptonica/
    cd leptonica/
    emmake cmake ${CUR_DIR}/sys/leptonica \
        -DCMAKE_INSTALL_PREFIX=${INSTALL_DIR} \
        -DCMAKE_TOOLCHAIN_FILE=${EM_TOOLCHAIN_FILE} \
        -D ZLIB_LIBRARY=${ZLIB_LIB} \
        -D ZLIB_INCLUDE_DIR=${INC_DIR} \
        -D JPEG_LIBRARY=${JPEG_LIB} \
        -D JPEG_INCLUDE_DIR=${INC_DIR} \
        -D PNG_LIBRARY=${PNG_LIB} \
        -D PNG_PNG_INCLUDE_DIR=${INC_DIR} \
        -D TIFF_LIBRARY=${TIFF_LIB} \
        -D TIFF_INCLUDE_DIR=${INC_DIR}
    emmake make install -j${NPROC}
}

build_tesseract() {
    cd ${SYS_BUILD_DIR}
    rm -rf tesseract/
    mkdir tesseract/
    cd tesseract/
    emmake cmake ${CUR_DIR}/sys/tesseract \
        -DCMAKE_INSTALL_PREFIX=${INSTALL_DIR} \
        -DCMAKE_TOOLCHAIN_FILE=${EM_TOOLCHAIN_FILE} \
        -DLeptonica_DIR=${SYS_BUILD_DIR}/leptonica
    emmake make install -j${NPROC}
}

build_lept_sys() {
    cd ${SYS_BUILD_DIR}
    rm -rf lept/
    mkdir lept
    cd lept/
    cmake ${CUR_DIR}/sys/leptonica \
        -DCMAKE_INSTALL_PREFIX=${INSTALL_DIR} \
        -D BUILD_SHARED_LIBS=OFF
    make install -j${NPROC}
}

build_tess_sys() {
    cd ${SYS_BUILD_DIR}
    rm -rf tess/
    mkdir tess
    cd tess/
    cmake ${CUR_DIR}/sys/tesseract -DCMAKE_INSTALL_PREFIX=${INSTALL_DIR}
    make install -j${NPROC}
}

main() {
    # create_dir
    # build_zlib
    # build_libm
    # build_libjpeg
    # build_libpng
    # build_libtiff
    # build_leptonica
    build_tesseract
    # build_lept_sys
    # build_tess_sys
}

main "$@"