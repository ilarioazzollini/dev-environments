# Development on an Ubuntu host PC

We assume that the [Docker engine](https://docs.docker.com/engine/install/ubuntu/) is installed and running on the PC. If that is the case, we can open a terminal and check Docker version by

```bash
$ docker --version
```

If we did not get any errors, we can safely proceed.

## Build the Docker container image

Our host PC has an Ubuntu OS, and we will run a Linux container. In order to build our Docker container image for Rust development, we can open a terminal, and:

```bash
$ cd <dev-environments/Rust>
$ export RUST_VERSION=1.91.0
$ docker build \
    --build-arg RUST_VERSION=${RUST_VERSION} \
    -f docker/Dockerfile \
    -t rust-dev:${RUST_VERSION} \
    .
```

Then, if the build was successful, you should see the image in the list of avaliable images (run `docker image ls`).

## Run the Docker container

Now, let us run a container from the image that we just built

```bash
$ cd <dev-environments/Rust>
$ export RUST_VERSION=1.91.0
$ docker run \
    -it \
    --rm \
    --privileged \
    --network=host \
    -v .:/root/Rust \
    -w /root/Rust \
    --name rust-dev-cnt \
    rust-dev:${RUST_VERSION} \
    bash
```

Now we should be inside a terminal in the Docker container we just launched. If you want to check its status, open another terminal on the host and run `docker container ls`, and you should see our container named `rust-dev-cnt` running.

Now, let us go back to the terminal inside the container. If everything worked correctly we should be at this location:

```bash
$ pwd
/root/Rust
```

and we should see the whole content of the Rust folder

```bash
$ ls
README.md  docker  docs hello_world
```

as we are now "sharing" this folder between the host PC and the container (we did it while running the container by means of the "mount volume" flag `-v .:/root/Rust`).

We can check that `rustc` and `cargo` are correctly installed and available inside this container (with the specific version we chose):

```zsh
$ cargo --version
cargo 1.91.0 (ea2d97820 2025-10-10)
$ rustc --version
rustc 1.91.0 (f8297e351 2025-10-28)
```

## Hello, Rust!

Inside the running container, we can now navigate to the root directory of the `hello_world` executable project, format it, check it, run linters, build it, and finally run the executable (`cargo run`), by:

```zsh
$ cd hello_world
$ cargo fmt
$ cargo check
$ cargo clippy
$ cargo build
$ cargo run
```

## Stopping and removing the running container

When we are done working inside the container, we can simply close the terminal:

```bash
$ exit
exit
```

and check that the container was correctly stopped and removed by running `docker container ls -a`. Notice that the container was stopped as soon as the process with `pid=1` stopped running (the bash terminal). Then, the container was removed as soon as it stopped because of the `--rm` flag we used while running it.

## Development using VS Code w/ Dev Containers extension

In the following, we propose the setup of [VS Code](https://code.visualstudio.com/download) as in IDE. Be sure to also add it to path while (or after) installing it.

If we correctly installed it and added it to path, we can open a terminal and check its version by

```bash
$ code --version
```

If we did not get any errors, we can safely proceed to install the needed extensions on the host PC as follows:

```bash
$ code --force --install-extension ms-azuretools.vscode-containers
$ code --force --install-extension ms-vscode-remote.remote-containers
```

Then, we can get the complete development setup by following these steps. Open a terminal, navigate to our Rust folder, and open VS code from there:

```zsh
$ cd <dev-environments/Rust>
$ code .
```

Then, open the Command Palette (`Cmd+Shift+P`), start typing `Dev Containers: Reopen in Container` and then click on it (or simply press `Enter`). VS Code will now close the current window, then run a `rust-dev` Docker container, install and open a new instance of VS Code inside that container, and install the desired extensions. VS Code does all this by following the instructions contained in the [devcontainer.json](../.devcontainer/devcontainer.json) file.

Now let us try to open the file [main.rs](../hello_world/src/main.rs) and start writing some new Rust code. We should see that the installed packages + VS Code extensions give us intellisense, errors in line as we write, format on save, etc.
