#!/bin/sh

set -e

help() {
    cat <<'EOF'
Install a binary release of a Rust crate hosted on GitHub
Usage:
    install.sh [options]
Options:
    -h, --help      Display this message
    -f, --force     Force overwriting an existing binary
    --tag TAG       Tag (version) of the crate to install (default <latest release>)
    --target TARGET Install the release compiled for $TARGET (default <`rustc` host>)
    --to LOCATION   Where to install the binary (default ~/.cargo/bin)
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
        --target)
            target=$2
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
  echo "Currently supporting darwin, win32 and linux"
  echo "You can provide a specific artifact to install with --artifact, find the name of the artifact from https://github.com/JasonShin/sqlx-ts/releases"
  echo "e.g. sqlx-ts_v0.1.0_x86_64-apple-darwin.zip (It must be a name in a format of sqlx-ts_v1.2.3_arch.tar.gz)"
  exit 1
fi

need basename
need curl
need install
need mkdir
need mktemp
need tar
need sed

# Optional dependencies
if [ -z $tag ] || [ -z $target ]; then
    need cut
fi

if [ -z $tag ]; then
    need rev
fi

if [ -z $target ]; then
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

    tag=$(curl -s "$url/latest" | cut -d'"' -f2 | rev | cut -d'/' -f1 | rev)
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
  if [ "$os" == "darwin" ]; then
    target="x86_64-apple-darwin.tar.gz"
  elif [ "$os" == "win32" ]; then
    target="x86_64-pc-windows-gnu.tar.gz"
  elif [ "$os" == "linux" ]; then
    target="unknown-linux-musl.tar.gz"
  else
    echo "Cannot find a matching OS for $os"
    exit 1
  fi
  url="$url/download/$tag/sqlx-ts_$tag\_$target"
else
  tag="$(cut -d'_' -f2 <<< "$artifact")"
  url="$url/download/$tag/$artifact"
fi

td=$(mktemp -d || mktemp -d -t tmp)

curl -sL $url | tar -C $td -xz

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
