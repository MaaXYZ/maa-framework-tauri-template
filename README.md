# MaaFramework Tauri Template

This is a template for creating a MaaFramework front end with Tauri.

## Getting Started

Note that the building process is only currently tested and supported on Windows. Support for Linux is planned.

### MaaAgentBinary

This repo contains the `MaaAgentBinary` as a submodule. You need to clone this repo with the `--recurse-submodules` flag or run the following commands after cloning:

```bash
git submodule init
git submodule update
```

### MaaFramework

You need to have a latest version of MaaFramework installed in your system. If you already have, make sure that CMake can find it. The easiest way to do this is to add your MaaFramework installation directory to your `CMAKE_PREFIX_PATH` or `PATH` environment variable.

If you don't have it installed, the python script will do that anyway so there is generally no need for you to do it manually.

### Run Dependency Script

If you have an installed version of MaaFramework, you can skip this step and manually copy the MaaFramework dlls to `tauri/` directory.

If not, you can run the `makedeps.py` script to download and extract the latest version of MaaFramework to the `tauri/` directory. Then, as said above, you might need to set your environment variables to point to the MaaFramework installation directory.

### Build and Run

Use of `pnpm` is encouraged, but you can probably use `npm` or `yarn` as well.

```bash
pnpm install
pnpm tauri dev
```

Or a `mock` command is provided where we will not try to really scan the devices but show a mock device, this is useful for UI development.

```bash
pnpm mock
```

Then build the project with:

```bash
pnpm tauri build
```

## Note

This template assumes AdbController but with some modifications, you should be able to use any other controller as well.
