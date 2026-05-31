import abc

import requests


class Plugin(abc.ABC):
    """An abstract base class for Minecraft plugins."""

    @abc.abstractmethod
    def download(self) -> bytes:
        """Get the plugin's jar file as bytes."""

        raise NotImplementedError


class GeyserSpigot(Plugin):
    def download(self) -> bytes:
        url: str = (
            "https://download.geysermc.org/v2/projects/geyser"
            "/versions/latest/builds/latest/downloads/spigot"
        )

        response: requests.Response = requests.get(url)

        if response.status_code != 200:
            raise RuntimeError("Failed to download Geyser-Spigot")

        return response.content


class Floodgate(Plugin):
    def download(self) -> bytes:
        url: str = (
            "https://download.geysermc.org/v2/projects/floodgate"
            "/versions/latest/builds/latest/downloads/spigot"
        )

        response: requests.Response = requests.get(url)

        if response.status_code != 200:
            raise RuntimeError("Failed to download Floodgate")

        return response.content
