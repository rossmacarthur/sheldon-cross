FROM rustembedded/cross:{{target}}

COPY install_openssl.sh /
RUN bash /install_openssl.sh {{install_openssl_args}}

ENV OPENSSL_DIR=/openssl \
{{~#if is_musl }}
    OPENSSL_STATIC=1 \
{{~/if}}
    OPENSSL_INCLUDE_DIR=/openssl/include \
    OPENSSL_LIB_DIR=/openssl/lib
