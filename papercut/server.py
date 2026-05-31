import os
import pathlib
import subprocess
from os import path
from typing import Any, ClassVar, cast, final

import requests

from .plugins import Plugin

DEFAULT_SERVER_DIR = pathlib.Path(os.getcwd() + "/MinecraftServer").absolute()


@final
class PaperServer:
    """Represents a PaperMC Minecraft server."""

    BASE_URL: ClassVar[str] = "https://papermc.io/api/v2/projects/paper"

    def __init__(self, server_dir: pathlib.Path = DEFAULT_SERVER_DIR) -> None:
        self.server_dir = server_dir

    def download(self, version: str | None = None) -> None:
        """Downloads the PaperMC server.jar file.

        Parameters
        ----------
        version: :class:`str`
            The version of the PaperMC server to download. Defaults to
            the latest version if not specified.

        Raises
        ------
        :exc:`RuntimeError`
            If the download fails.
        """

        if not path.exists(self.server_dir):
            os.mkdir(self.server_dir)

        if version is None:
            if (version := self.get_latest_version()) is None:
                raise RuntimeError("Failed to get latest version of PaperMC.")

        if (file_content := self.get_file_content(version)) is None:
            raise RuntimeError("Failed to download PaperMC server.jar.")

        file_path: pathlib.Path = self.server_dir / "server.jar"

        with open(file_path, "wb") as file:
            file.write(file_content)

        command: list[str] = ["java", "-jar", file_path.as_posix(), "--nogui"]
        subprocess.run(command, cwd=self.server_dir, check=True)

    def install_plugin(self, plugin: Plugin, save_as: str) -> None:
        """Installs a plugin to the PaperMC server.

        Parameters
        ----------
        plugin: :class:`Plugin`
            The plugin to install.
        save_as: :class:`str`
            The name to save the plugin as.

        Raises
        ------
        :exc:`RuntimeError`
            If the plugin fails to install.
        """

        try:
            content: bytes = plugin.download()
        except Exception as exc:
            error: str = f"Failed to install plugin '{save_as}'"
            raise RuntimeError(error) from exc
        else:
            with open(save_as, "wb") as f:
                f.write(content)

    @classmethod
    def get_latest_version(cls) -> str:
        """Gets the latest version of the PaperMC server.

        Returns
        -------
        :class:`str`
            The latest version of the PaperMC server.

        Raises
        ------
        :exc:`RuntimeError`
            If the request fails.
        """

        response: requests.Response = requests.get(cls.BASE_URL)

        if response.status_code != 200:
            raise RuntimeError("Failed to get latest version of PaperMC.")

        return cast(str, response.json()["versions"][-1])

    @classmethod
    def get_file_content(cls, version: str) -> bytes:
        """Gets the content of the PaperMC server.jar file.

        Parameters
        ----------
        version: :class:`str`
            The version of the PaperMC server to download.

        Returns
        -------
        :class:`bytes`
            The content of the PaperMC server.jar file.

        Raises
        ------
        :exc:`RuntimeError`
            If the request fails.
        """

        url: str = cls.BASE_URL + f"/versions/{version}/builds"
        response: requests.Response = requests.get(url)

        if response.status_code != 200:
            raise RuntimeError("Failed to get latest version of PaperMC.")

        latest_build: dict[str, Any] = response.json()["builds"][-1]
        file_name: str = latest_build["downloads"]["application"]["name"]
        build_number: int = latest_build["build"]
        url += f"/{build_number}/downloads/{file_name}"

        try:
            file: requests.Response = requests.get(url)
        except Exception as exc:
            raise exc
        else:
            return file.content

