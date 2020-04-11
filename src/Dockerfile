FROM rustembedded/cross:{{triple}}

{{#if install_gnupg~}}
RUN apt-get update \
  && apt-get install -y --no-install-recommends gnupg \
  && rm -rf /var/lib/apt/lists/*

{{/if~}}

COPY install_git.sh /
RUN bash /install_git.sh

COPY install_openssl.sh /
RUN bash /install_openssl.sh {{install_openssl_args}}

ENV OPENSSL_DIR=/openssl \
    OPENSSL_INCLUDE_DIR=/openssl/include \
    OPENSSL_LIB_DIR=/openssl/lib
