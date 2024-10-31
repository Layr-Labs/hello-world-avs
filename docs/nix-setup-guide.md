# Nix Setup Guide 

Here you will find instructions on how to install and configure [nix package manager](https://nixos.wiki/wiki/Nix_package_manager) to work with 
projects.

## Installing / Upgrading 

If you don't have nix installed, follow the instructions on the 
[official website](https://nixos.org/download), or simply use one of following
commands in a terminal:

- For multi-user installation (i.e., system-wide setup, **recommended**)
```bash
sh <(curl -L https://nixos.org/nix/install) --daemon
```
- For single-user installation (i.e., local setup)
```bash 
sh <(curl -L https://nixos.org/nix/install) --no-daemon
```

We use nix `flakes` which require nix version `>= 2.4`, therefore if you already 
have nix installed, make sure to 
[upgrade](https://nixos.org/manual/nix/stable/installation/upgrading) to a 
recent version.

## Becoming a Truster User

If you are on NixOS or have installed nix in multi-user mode, you **must** be 
a [trusted user](https://nixos.org/nix/manual/#ssec-multi-user), which is 
necessary to enable access to our binary cache.

On non-NixOS systems, append the following line to the system-wide configuration 
`/etc/nix/nix.conf`:
```txt
trusted-users = USER root
```
**Where `USER` is the result of running `whoami` in a terminal.**

You can also use a group name by prefixing it with `@`. For instance, to add all 
members of the `wheel` group:
```txt
trusted-users = @wheel root
```

On NixOS, add the user/group name to the list under 
[`nix.settings.trusted-users`](https://search.nixos.org/options?show=nix.settings.trusted-users).

## Setting Up for Flakes

We use [Nix flakes](https://nixos.wiki/wiki/Flakes). To configure for flakes, follow the instruction  on the [Nixos Wiki](https://nixos.wiki/wiki/Flakes) or simply do one of the following depending on your system:

- On non-NixOS systems, edit one of the following two files (create them if they don't exist):
   - `/etc/nix/nix.conf` for system-wide settings on a multi-user installation 
   - `~/.config/nix/nix.conf` for user-wide settings on a single-user installation

By appending the following lines:
```txt
experimental-features = nix-command flakes
```

On NixOS systems, set the following NixOS options:
```nix
 nix.settings.experimental-features = [ "nix-command" "flakes" ];
```

## Notes for Apple Users

Apple Silicon users can run any Intel binary out-of-the-box thanks to Rosetta
emulation, but when working with nix flakes, the `aarch64-darwin` system will be
selected by default.

However, some projects at cannot be built natively on `aarch64-darwin`.

Therefore you must specify the `--system` explicitly to target `x86_64-darwin`.
```bash
nix (develop|build|run|check) --system x86_64-darwin # Will use the x86_64-darwin derivation
nix (develop|build|run|check) # Will use the aarch64-darwin derivation, if available
```

To enable this, you must append the following lines to your `/etc/nix/nix.conf` 
or `~/.config/nix/nix.conf`:
```txt
extra-platforms = x86_64-darwin aarch64-darwin
extra-sandbox-paths = /System/Library/Frameworks /System/Library/PrivateFrameworks /usr/lib /private/tmp /private/var/tmp /usr/bin/env
```

You may need to reload the nix daemon for changes to take effect:
```bash
sudo launchctl stop org.nixos.nix-daemon
sudo launchctl start org.nixos.nix-daemon
```

## Development Shell and VSCode 

Now that you have nix installed and configured, you may enter the development 
shell:
```
nix develop
```
If you are on Apple Silicon and a native shell is not available, you will want 
to run this instead:
```
nix develop --system x86_64-darwin
```

If you are a `VSCode` user, you may also start your development session by 
executing the following command:
```
nix develop --command code
```

If you are running `nix develop` for the first time, please enter `y` at the 
following prompts to create the local nix settings file 
`~/.local/share/nix/trusted-settings.json`:
```txt  
> do you want to allow configuration setting 'accept-flake-config' to be set to 'true' (y/N)?
> do you want to permanently mark this value as trusted (y/N)?
```

It is particularly important to accept the `extra-substituters` and 
`extra-trusted-public-keys` settings as these will grant access to our binary 
cache.

When `nix develop` is run for the first time, a significant amount of 
dependencies will be downloaded, built and installed. This process may take a 
couple of hours but is expected to happen only once. However, if you ever 
witness that GHC is also being built from scratch, then it is likely that your 
binary cache has not been configured properly or is not being considered.
Accepting the configuration settings as outlined above should be sufficient to 
avoid this. Nevertheless, if your caches are still broken, you'll want to review
this document carefully to ensure that your nix installation is properly 
configured.

You will know that your caches are broken if you see this message:
```
warning: ignoring untrusted substituter 'https://cache.iog.io', you are not a trusted user.
```

If is possible that your settings are correct but the nix daemon is not properly 
considering them. In this case, stop the `nix develop` process and restart the 
`nix-daemon` as follows:
```bash 
sudo systemctl stop nix-daemon.service
``` 
Once done, you can launch the `nix develop` process again.
