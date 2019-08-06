# Overview

For the development of a CSI driver for k8s we need to have the ability to
to manipulate ZFS pools. There are existing crates out there but, they are
focused on complete pool management where this solely based on dataset management.

## Build

In order to build this you *must* have the ZFS sources available. And update the path
in `build.rs` in `libzfs-sys`. Other packages may be required (i.e C compilers etc).

Currently, only create and destroy are implemented as those are (for now) the only
things I need.


