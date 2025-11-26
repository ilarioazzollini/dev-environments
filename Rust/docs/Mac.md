# Development on a Mac host PC

The easiest way to get Docker on Mac is by installing [Docker Desktop](https://www.docker.com/products/docker-desktop/).

Open Docker Desktop in order to get the Docker engine running. If we correctly installed Docker Desktop and the engine is running, we can open a terminal and check Docker version by

```zsh
docker --version
```

If we did not get any errors, we can safely proceed.

## Build the Docker container image

Our host PC has a Mac OS, but we will always run (some kind of) linux containers. In order to build our Docker container image for Rust development, we can open a terminal, navigate to the Rust folder

```zsh
cd <dev-environments/Rust>
```

then choose the version of Rust we want

```zsh
export RUST_VERSION=1.91.0
```

and then build the Docker container image

```zsh
$ docker build \
    --build-arg RUST_VERSION=${RUST_VERSION} \
    -f docker/Dockerfile \
    -t rust-dev:${RUST_VERSION} \
    .
```

If the build was successful, you should see the image in the list of avaliable images (run `docker image ls`).

## Run the Docker container

In order to run a container from the image we built in the previous section, we need to open a terminal, navigate to the Rust folder

```zsh
cd <dev-environments/Rust>
```

then choose the same version of Rust we used to build the image

```zsh
export RUST_VERSION=1.91.0
```

and then run a Docker container, that we choose to call `rust-dev-cont`, by


```zsh
docker run \
    -it \
    --rm \
    --privileged \
    --network=host \
    -v .:/root/Rust \
    -w /root/Rust \
    --name rust-dev-cont \
    rust-dev:${RUST_VERSION} \
    bash
```

Now we should be inside a terminal in the Docker container we just launched. If you want to check its status, open another terminal on the host and run `docker container ls`, and you should see our container named `rust-dev-cont` running.

Now, let us go back to the terminal inside the container. If everything worked correctly we should be at the location `/root/Rust`. We can check it by

```zsh
pwd
```

and we should see the whole content of the Rust folder by running the `ls` command. This is because we are now "sharing" this folder between the host PC and the container (we did it while running the container by means of the "mount volume" flag `-v .:/root/Rust`).

We can check that `rustc` and `cargo` are correctly installed and available inside this container (with the specific version we chose):

```zsh
cargo --version
rustc --version
```

## Hello, Rust!

Inside the running container, we can now navigate to the root directory of the `hello_world` executable project

```zsh
cd hello_world
```

format it

```zsh
cargo fmt
```

check it (it is a quick build)

```zsh
cargo check
```

run linters

```zsh
cargo clippy
```

build it (in Debug mode by defalt)

```zsh
cargo build
```

and finally run the executable by

```zsh
cargo run
```

## Stopping and removing the running container

When we are done working inside the container, we can simply close the terminal

```zsh
exit
```

and check that the container was correctly stopped and removed by running `docker container ls -a`. Notice that the container was stopped as soon as the process with `pid=1` stopped running (the bash terminal). Then, the container was removed as soon as it stopped because of the `--rm` flag we used while running it.

## Development using VS Code w/ Dev Containers extension

In the following, we propose the setup of [VS Code](https://code.visualstudio.com/download) as in IDE. Be sure to also add it to path while (or after) installing it.

If we correctly installed it and added it to path, we can open a terminal and check its version by

```zsh
code --version
```

If we did not get any errors, we can safely proceed to install the needed extensions on the host PC as follows:

```zsh
code --force --install-extension ms-azuretools.vscode-containers
code --force --install-extension ms-vscode-remote.remote-containers
```

Then, we can get the complete development setup by following these steps. Open a terminal, navigate to our Rust folder

```zsh
cd <dev-environments/Rust>
```

and open VS code from there

```zsh
code .
```

Then, open the Command Palette (`Cmd+Shift+P`), start typing `Dev Containers: Reopen in Container` and then click on it (or simply press `Enter`). VS Code will now close the current window, then run a `rust-dev` Docker container, install and open a new instance of VS Code inside that container, and install the desired extensions. VS Code does all this by following the instructions contained in the [devcontainer.json](../.devcontainer/devcontainer.json) file.

Now let us try to open the file [main.rs](../hello_world/src/main.rs) and start writing some new Rust code. We should see that the installed packages + VS Code extensions give us intellisense, errors in line as we write, format on save, etc.
