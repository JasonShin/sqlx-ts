#!/bin/sh

set -e

help() {
    cat <<'EOF'
Install a binary release of a Rust crate hosted on GitHub
Usage:
    install.sh [options]
Options:
    -h, --help            Display this message
    -f, --force           Force overwriting an existing binary
    --os OS               Your current OS, it's used to determine the type of binary to be installed (one of macos or win32 or linux)
    --artifact ARTIFACT   Specific artifact to install. Please find the artifact name from https://github.com/JasonShin/sqlx-ts/releases (e.g. sqlx_ts_v0.1.0_x86_64-apple-darwin.zip)
    --tag TAG             Tag (version) of the crate to install (default <latest release>)
    --to LOCATION         Where to install the binary (default to ~/.cargo/bin)
EOF
}

say() {
    echo "install.sh: $1"
}

say_err() {
    say "$1" >&2
}

err() {
    if [ ! -z $td ]; then
        rm -rf $td
    fi

    say_err "ERROR $1"
    exit 1
}

need() {
    if ! command -v $1 > /dev/null 2>&1; then
        err "need $1 (command not found)"
    fi
}

force=false
while test $# -gt 0; do
    case $1 in
        --os)
            os=$2
            shift
            ;;
        --artifact)
            artifact=$2
            shift
            ;;
        --force | -f)
            force=true
            ;;
        --help | -h)
            help
            exit 0
            ;;
        --tag)
            tag=$2
            shift
            ;;
        --to)
            dest=$2
            shift
            ;;
        *)
            ;;
    esac
    shift
done

################
# Dependencies #
################

if [ -z $os ] && [ -z $artifact ]; then
  echo "You must provide your OS type for installation"
  echo "Currently supporting macos, win32 and linux"
  echo "You can provide a specific artifact to install with --artifact, find the name of the artifact from https://github.com/JasonShin/sqlx-ts/releases"
  echo "e.g. sqlx_ts_v0.1.0_x86_64-apple-darwin.zip (It must be a name in a format of sqlx-ts_v1
  .2.3_arch.tar.gz)"
  exit 1
fi

need basename
need curl
need install
need mkdir
need mktemp

if [ "$os" == "macos" ] || [ "$os" == "linux" ]; then
  need tar
else
  need unzip
fi

need sed

# Optional dependencies
if [ -z $tag ]; then
    need cut
fi

if [ -z $tag ]; then
    need rev
fi

if [ -z $to ]; then
    need grep
    need rustc
fi
################

url="https://github.com/jasonshin/sqlx-ts"
say_err "GitHub repository: $url"

url="$url/releases"

if [ -z $tag ]; then
    if [ ! -z $artifact ]; then
      echo "artifact was given, it will override tag - artifact: $artifact, tag: $tag"
    fi

    tag=$(curl --silent "https://api.github.com/repos/jasonshin/sqlx-ts/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    say_err "Tag: latest ($tag)"
else
    say_err "Tag: $tag"
fi

if [ -z $dest ]; then
    dest="$HOME/.cargo/bin"
fi

say_err "Installing to: $dest"

# if a full artifact path is given, use that
# if only OS is given use OS + version | latest
if [ -z $artifact ]; then
  if [ "$os" == "macos" ]; then
    target="x86_64-apple-darwin.tar.gz"
  elif [ "$os" == "windows" ]; then
    target="x86_64-pc-windows-gnu.zip"
  elif [ "$os" == "linux" ]; then
    target="unknown-linux-musl.tar.gz"
  else
    echo "Cannot find a matching OS for $os"
    exit 1
  fi
  url="$url/download/$tag/sqlx_ts_${tag}_${target}"
else
  tag="$(cut -d'_' -f3 <<< "$artifact")"
  url="$url/download/$tag/$artifact"
fi

td=$(mktemp -d || mktemp -d -t tmp)

echo "URL to download $url"
if [ "$os" == "macos" ] || [ "$os" == "linux" ]; then
  curl -sL $url | tar -C $td -xz
else
  curl -sL -o ./sqlx-ts-latest.zip $url
  unzip ./sqlx-ts-latest.zip -d $td
  rm -f ./sqlx-ts-latest.zip
fi

# shellcheck disable=SC2045
for f in $(ls $td); do
    test -x $td/$f || continue

    if [ -e "$dest/$f" ] && [ $force = false ]; then
        err "$f already exists in $dest"
    else
        mkdir -p $dest
        install -m 755 $td/$f $dest
    fi
done

rm -rf $td
