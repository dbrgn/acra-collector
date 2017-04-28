# Releasing

Set variables:

    export VERSION=X.Y.Z
    export GPG_KEY=EA456E8BAF0109429583EED83578F667F2F3A5FA

Update version numbers:

    vim -p Cargo.toml

Commit & tag:

    git commit -S${GPG_KEY} -m "Release v${VERSION}"
    git tag -s -u ${GPG_KEY} v${VERSION} -m "Version ${VERSION}"

Build & sign:

    ./build-debian.sh
    gpg -o target/release-debian/acra-collector.sig --detach-sig target/release-debian/acra-collector

Push:

    git push && git push --tags
