# Releasing

Set variables:

    export VERSION=X.Y.Z
    export GPG_KEY=EA456E8BAF0109429583EED83578F667F2F3A5FA
    export RELEASEDIR=acra-collector-v${VERSION}

Update version numbers:

    vim -p Cargo.toml

Commit & tag:

    git commit -S${GPG_KEY} -m "Release v${VERSION}"
    git tag -s -u ${GPG_KEY} v${VERSION} -m "Version ${VERSION}"

Build & sign:

    ./build-debian.sh
    mkdir -p builds && cd builds
    cp -Rv ../target/release-debian $RELEASEDIR
    strip $RELEASEDIR/acra-collector
    gpg -o $RELEASEDIR/acra-collector.sig --detach-sig $RELEASEDIR/acra-collector
    cp ../LICENSE-MIT ../LICENSE-APACHE $RELEASEDIR/
    tar cfvz acra-collector-v${VERSION}-debian10.tar.gz $RELEASEDIR
    cd ..

Push:

    git push && git push --tags
