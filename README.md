# ATSAML10 Peripheral Access Crate

These crates are mechanically generated from the .svd files, by `svd2rust`.

## Rebuilding from svds

The latest SAML10 Device Support Pack can be downloaded from http://packs.download.atmel.com/

You will need the `svd` binary from svdtools in your $PATH:

```sh
pip install --user svdtools
export PATH=$PATH:$HOME/.local/bin
```

After extracting the `.pack` file (it's really a zip):

```sh
rm -rf saml10*
./build.sh ~/path/to/extracted/pack/svd/directory
```
