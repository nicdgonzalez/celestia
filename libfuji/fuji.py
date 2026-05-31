import datetime as dt
import logging
import os
import pathlib
import shutil
import socket
import subprocess
import textwrap
import time
from typing import Any, Callable, Dict, List, Optional, Tuple, TypeVar, cast

import requests

from .plugins import (
    Plugin,
    get_floodgate_plugin,
    get_geyser_plugin,
    get_worldedit_plugin,
)
from .tmux import TmuxSession

R = TypeVar("R")
commands: Dict[str, Callable[..., R]] = {}
command_aliases: Dict[str, str] = {}


def add_command(
    name: Optional[str] = None,
    description: Optional[str] = None,
    aliases: List[str] = None,
    hidden: bool = False,
) -> Callable[[Callable[..., R]], Callable[..., R]]:
    """Add a command to the commands dictionary.

    Parameters
    ----------
    name: :class:`str`, optional
        The name of the command. Defaults to the function name.
    description: :class:`str`, optional
        The description of the command. Defaults to the function docstring.
    aliases: :class:`list` of :class:`str`, optional
        A list of aliases for the command.
    hidden: :class:`bool`, optional
        Whether the command should be hidden from the help menu.

    Returns
    -------
    :class:`Callable`
        The decorated function.
    """

    def wrapper(fn: Callable[..., R]) -> Callable[..., R]:
        if hidden:
            # No point in performing any checks if the command is hidden
            return fn

        nonlocal name, description

        if (name := name or fn.__name__) in commands.keys():
            raise ValueError(f"Command '{name}' already exists")

        if aliases is not None:
            for alias in aliases:
                if alias in commands or alias in command_aliases:
                    raise ValueError(f"Alias '{alias}' already exists")

                command_aliases[alias] = name

        if description is None:
            description = "\n".join(
                [
                    line.lstrip()
                    for line in fn.__doc__.split("\n\n")[0].split("\n")
                ]
            )

        commands[name] = {
            "callback": fn,
            "description": description,
        }

        return fn

    return wrapper


class Fuji:
    """The main Fuji class."""

    BASE_DIR = pathlib.Path.home().joinpath(".fuji")
    DEFAULT_SERVER = BASE_DIR.joinpath("server-celestia")

    def __init__(self) -> None:
        self._log = logging.getLogger(__name__)
        self.logging_setup()

        self._tmux = TmuxSession("fuji")

    def logging_setup(self) -> None:
        """Setup the logging configuration."""

        handlers: List[logging.Handler] = []
        formatter = logging.Formatter(
            fmt="%(asctime)s [%(levelname)s] %(name)s: %(message)s",
            datefmt="%Y-%m-%d %H:%M:%S",
        )

        (logs := self.BASE_DIR / "logs").mkdir(exist_ok=True, parents=True)
        file_name = logs.joinpath(
            dt.datetime.now().strftime("%Y-%m-%d_%H-%M-%S") + ".log"
        )
        file_handler = logging.FileHandler(file_name)
        del logs, file_name
        handlers.append(file_handler)

        stream_handler = logging.StreamHandler()
        handlers.append(stream_handler)

        for handler in handlers:
            handler.setFormatter(formatter)
            self._log.addHandler(handler)

        self._log.setLevel(logging.DEBUG)

    @add_command()
    def help(self, *args: Tuple[str, ...]) -> None:
        """Shows this help message."""

        print(
            "A command line tool for managing Paper Minecraft servers.\n"
            "\n"
            "Usage: fuji <command> [options]\n"
            "\n"
            "Commands:\n"
        )

        if len(commands) == 0:
            print("  No commands found.")
            return

        longest_name = max(len(name) for name in commands.keys())

        for name, info in commands.items():
            print(f"  {name.ljust(longest_name)}  ", end="")

            if info["description"] is not None:
                print(
                    textwrap.fill(
                        info["description"],
                        width=80,
                        initial_indent="",
                        subsequent_indent=" " * (longest_name + 4),
                    )
                )
            else:
                print("\n", end="")

    def _fuji_init(self, server_path: pathlib.Path) -> None:
        """Initializes Fuji."""

        if not server_path.exists():
            self._log.info("Creating the server directory")
            server_path.mkdir(parents=True)

        if (fuji_init := server_path.joinpath("fuji-init")).exists():
            self._log.error("Fuji has already been initialized")
            return

        self._log.info("Initializing Fuji")

        # Get the latest version of PaperMC
        server_jar = self.BASE_DIR.joinpath("server.jar")
        file_name, file_data = self.get_paper_jar(server_jar)

        if server_jar.resolve() != file_name:
            self._log.info("Updating the PaperMC server")
            paper_jar = self.BASE_DIR.joinpath(file_name)
            _ = paper_jar.write_bytes(file_data)

            self._log.info("Updating the server.jar symlink")
            if server_jar.exists():
                server_jar.unlink()

            server_jar.symlink_to(paper_jar)

        # The first time the server is started, the `eula.txt` file will be
        # created. We need to agree to the EULA before starting the server.
        cmd = f"java -jar {server_jar.as_posix()} --nogui"
        _ = subprocess.run(
            cmd, shell=True, cwd=server_path, stdout=subprocess.DEVNULL
        )

        # Agree to the EULA
        _ = input(
            "By continuing, you agree to the Minecraft EULA: "
            "https://aka.ms/MineraftEULA\n"
            "Press ENTER to continue..."
        )

        _ = server_path.joinpath("eula.txt").write_text("eula=true")

        # Set custom server properties
        server_properties = server_path.joinpath("server.properties")
        properties = {
            k: v
            for k, v in (
                line.split("=")
                for line in server_properties.read_text().splitlines()
                if line != "" and not line.startswith("#")
            )
        }

        properties["level-seed"] = "6805533843294021817"
        properties["level-name"] = "celestia"
        properties["difficulty"] = "hard"
        properties["motd"] = "Celestia Survival Multiplayer 1.20.2"

        server_properties.write_text(
            "\n".join(f"{k}={v}" for k, v in properties.items())
        )

        # Download and install the GeyserMC and Floodgate plugins
        # for cross-play with Bedrock Edition clients
        plugins = server_path.joinpath("plugins")
        assert plugins.parent.exists(), "Server directory does not exist"
        plugins.mkdir(exist_ok=True)
        self.install_plugin(plugins, get_geyser_plugin())
        self.install_plugin(plugins, get_floodgate_plugin())
        self.install_plugin(plugins, get_worldedit_plugin())

        # Create the `fuji-init` file to indicate that Fuji has been setup
        fuji_init.touch()

    def install_plugin(
        self, plugins_path: pathlib.Path, plugin: Plugin
    ) -> None:
        """Installs the specified plugin into the server's `plugins` directory.

        Parameters
        ----------
        plugin_path: :class:`pathlib.Path`
            The path to the plugin directory.
        jar_name: :class:`str`
            The name of the plugin JAR file.
        jar_content: :class:`bytes`
            The contents of the plugin JAR file.
        """

        if not plugins_path.exists():
            self._log.info("Creating plugins directory.")
            plugins_path.mkdir(parents=True)

        self._log.info(f"Installing plugin: {plugin.name}")
        _ = plugins_path.joinpath(plugin.name).write_bytes(plugin.content)

    # TODO: Review all of this logic step-by-step. This is arguably the most
    # important function of the entire program...
    # Also, make this non-blocking... Maybe start it as a background process?
    @add_command()
    def start(self, *args: Tuple[str, ...]) -> None:
        """Starts the server. Attempts to reconnect if the server crashes."""

        server = self.DEFAULT_SERVER

        if len(args) > 0:
            # Stuff like this should probably be tracked internally somehow...
            # Maybe create a dict with names for different servers so the client
            # can easily switch between servers? Maybe also containerize with
            # Docker so you can host multiple servers on a singlr PC using
            # different ports?
            custom_server = pathlib.Path(args[0])

            if not custom_server.exists():
                self._log.error("Server directory does not exist")
                return

            if not custom_server.is_dir():
                self._log.error("Custom server directory is not a directory")
                return

            self._log.info(f"Starting the server from: {custom_server}")
            server = custom_server

        # TODO: Is it really necessary to create a function? Could this be done
        # using recursion instead?
        def start_server() -> None:
            lock_file = self.BASE_DIR.joinpath("fuji.lock")

            if lock_file.exists():
                # The server is already starting up; return to prevent
                # repeatedly sending the start command to the server
                return

            if self.server_online():
                self._log.warning(
                    "Attempted to start server while it was online"
                )
                return

            lock_file.touch()
            self._log.info("Starting the Minecraft server")

            if not self.BASE_DIR.joinpath("fuji-init").exists():
                self._fuji_init(server)

            # Ensure the tmux session is running
            if not self._tmux.exists():
                self._tmux.start()

            # Aikar's flags for optimizing the server
            # https://docs.papermc.io/paper/aikars-flags
            flags = " ".join(
                [
                    "-Xms2G",
                    "-Xmx2G",
                    "-XX:+UseG1GC",
                    "-XX:+ParallelRefProcEnabled",
                    "-XX:MaxGCPauseMillis=200",
                    "-XX:+UnlockExperimentalVMOptions",
                    "-XX:+DisableExplicitGC",
                    "-XX:+AlwaysPreTouch",
                    "-XX:G1NewSizePercent=30",
                    "-XX:G1MaxNewSizePercent=40",
                    "-XX:G1HeapRegionSize=8M",
                    "-XX:G1ReservePercent=20",
                    "-XX:G1HeapWastePercent=5",
                    "-XX:G1MixedGCCountTarget=4",
                    "-XX:InitiatingHeapOccupancyPercent=15",
                    "-XX:G1MixedGCLiveThresholdPercent=90",
                    "-XX:G1RSetUpdatingPauseTimePercent=5",
                    "-XX:SurvivorRatio=32",
                    "-XX:+PerfDisableSharedMem",
                    "-XX:MaxTenuringThreshold=1",
                    "-Dusing.aikars.flags=https://mcflags.emc.gs",
                    "-Daikars.new.flags=true",
                ]
            )

            server_jar = self.BASE_DIR.joinpath("server.jar")

            # Start the server in a tmux session
            cmd = (
                # TODO: `send-keys` could probably be added as a method to
                # the TmuxSession class
                f"tmux send-keys -t {self._tmux.name} "
                f"'cd {server.as_posix()} && java {flags} -jar "
                f"{server_jar.as_posix()} --nogui' ENTER &> /dev/null"
            )

            _ = subprocess.run(cmd, shell=True)

            retries = 0
            # Wait for the server to start
            while not self.server_online() and self._tmux.exists():
                if retries >= 30:
                    self._log.error("Failed to start the server")
                    break

                time.sleep(1)
                retries += 1

            # Remove the lock file
            os.remove(lock_file)

        while 1:
            if not self.server_online():
                start_server()

            time.sleep(60)

    # TODO: Maybe create a new module+class for Server-related code?
    def get_paper_jar(
        self, server_jar: pathlib.Path, version: Optional[str] = None
    ) -> Tuple[str, bytes]:
        """Gets the PaperMC server JAR for the specified version.

        Parameters
        ----------
        server_jar: :class:`pathlib.Path`
            The path to the server jar. This is used to check if the server JAR
            is already up-to-date.
        version: :class:`str`
            The version of PaperMC to get. If ``None``, defaults to the latest
            version available.

        Returns
        -------
        :class:`Tuple[str, bytes]`
            The name of the JAR file and the contents of the JAR file.
        """

        url = "https://papermc.io/api/v2/projects/paper"

        if version is None:
            if (response := requests.get(url)).status_code != 200:
                raise RuntimeError("Failed to get latest PaperMC version.")

            version = cast(str, response.json()["versions"][-1])

        url += f"/versions/{version}/builds"

        if (response := requests.get(url)).status_code != 200:
            raise RuntimeError("Failed to get the latest PaperMC build.")

        data: Dict[str, Any] = response.json()["builds"][-1]
        build: int = data["build"]
        file_name: str = data["downloads"]["application"]["name"]

        if server_jar.resolve().name == file_name:
            # The server JAR is already up-to-date
            return file_name, server_jar.read_bytes()

        url += f"/{build}/downloads/{file_name}"

        if (response := requests.get(url)).status_code != 200:
            raise RuntimeError("Failed to download PaperMC server JAR.")

        return file_name, response.content

    @add_command()
    def stop(self, *args: Tuple[str, ...]) -> None:
        """Stops the server."""

        if not self.server_online() and not self._tmux.exists():
            self._log.warning("Attempted to stop server while it was offline")
            return

        self._log.info("Stopping the Minecraft server")
        cmd = f"tmux send-keys -t {self._tmux.name} 'stop' ENTER &> /dev/null"
        _ = subprocess.run(cmd, shell=True)
        self._tmux.stop()

    # TODO: Add tests. I don't think this even works LOL
    @add_command()
    def restart(self, *args: Tuple[str, ...]) -> None:
        """Restarts the server."""

        self.stop(*args)
        self.start(*args)

    def server_online(self) -> bool:
        """Checks if the server is online."""

        address = host, port = ("127.0.0.1", 25565)

        try:
            with socket.create_connection(address, timeout=1):
                return True
        except socket.error:
            return False

    @add_command()
    def status(self, *args: Tuple[str, ...]) -> None:
        """Shows the status of the server."""

        if self.server_online():
            print("Server is online.")
        else:
            print("Server is offline.")

    @add_command()
    def backup(self, *args: Tuple[str, ...]) -> None:
        """Backs up the server."""

        if not self.DEFAULT_SERVER.exists():
            self._log.error(
                "Server directory does not exist. Nothing to backup"
            )
            return

        backups = self.BASE_DIR.joinpath("backups")

        if not backups.exists():
            backups.mkdir()

        backup_name = dt.datetime.now().strftime("%Y-%m-%d_%H-%M-%S")
        backup_dir = backups.joinpath(backup_name)

        self._log.info("Backing up the server")
        shutil.copytree(
            self.DEFAULT_SERVER,
            backup_dir,
            symlinks=True,
            ignore_dangling_symlinks=True,
        )

        self._log.info(f"Compressing the backup: {backup_name}.zip")
        shutil.make_archive(backup_dir, "zip", backup_dir)

    @add_command(hidden=True)
    def restore(self, *args: Tuple[str, ...]) -> None:
        """Restores the server from a backup."""

        raise NotImplementedError

    @add_command(hidden=True)
    def logs(self, *args: Tuple[str, ...]) -> None:
        """Shows the server logs."""

        raise NotImplementedError

    @add_command(name="add-plugin", hidden=True)
    def add_plugin(
        self, file_name: str, url: str, *args: Tuple[str, ...]
    ) -> None:
        """Adds a plugin to the server."""
        
        # TODO: Maybe make url optional and cache links so if
        # a plugin has been fetched before it can be fetched again by name only
        # this would also replace "update-plugin" functionality
        # maybe rename to install-plugin?

        raise NotImplementedError

    @add_command(name="remove-plugin", hidden=True)
    def remove_plugin(self, name: str, *args: Tuple[str, ...]) -> None:
        """Removes a plugin from the server."""

        raise NotImplementedError

    @add_command(name="list-plugins", hidden=True)
    def list_plugins(self, *args: Tuple[str, ...]) -> None:
        """Lists all plugins installed on the server."""

        raise NotImplementedError

    @add_command(name="update-plugins", hidden=True)
    def update_plugins(
        self, name: str = "all", *args: Tuple[str, ...]
    ) -> None:
        """Updates the specified plugin(s)."""

        raise NotImplementedError

    @add_command(name="replace-dimension", hidden=True)
    def replace_dimension(
        self, dimension: str, *args: Tuple[str, ...]
    ) -> None:
        """Replaces the specified dimension with a new one."""

        raise NotImplementedError

    @add_command(name="update-paper", hidden=True)
    def update_paper(self, *args: Tuple[str, ...]) -> None:
        """Updates the Paper Minecraft server."""

        raise NotImplementedError

    @add_command(name="update-fuji", hidden=True)
    def update_fuji(self, *args: Tuple[str, ...]) -> None:
        """Updates Fuji."""
        
        # TODO: Pull latest changes from Github

        raise NotImplementedError

    @add_command(name="send-command", hidden=True)
    def send_command(self, command: str, *args: Tuple[str, ...]) -> None:
        """Sends a command to the server."""

        raise NotImplementedError
