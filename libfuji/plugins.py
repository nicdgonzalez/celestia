import collections

import requests

__all__ = [
    "Plugin",
    "get_geyser_plugin",
    "get_floodgate_plugin",
    "get_worldedit_plugin",
]

BASE_SPIGOT_URL_FMT = (
    "https://download.geysermc.org/v2/projects/%s"
    "/versions/latest/builds/latest/downloads/spigot"
)

Plugin = collections.namedtuple("Plugin", ["name", "content"])


def get_geyser_plugin() -> Plugin:
    """Gets the Geyser plugin.

    Returns
    -------
    :class:`Plugin`
        The name of the plugin file and the contents of the plugin file.
    """

    url = BASE_SPIGOT_URL_FMT % "geyser"

    if (response := requests.get(url)).status_code != 200:
        raise RuntimeError("Failed to download Geyser-Spigot plugin.")

    return Plugin(name="Geyser-Spigot.jar", content=response.content)


def get_floodgate_plugin() -> Plugin:
    """Gets the Floodgate plugin.

    Returns
    -------
    :class:`Plugin`
        The name of the plugin file and the contents of the plugin file.
    """

    url = BASE_SPIGOT_URL_FMT % "floodgate"

    if (response := requests.get(url)).status_code != 200:
        raise RuntimeError("Failed to download floodgate-spigot plugin.")

    return Plugin(name="floodgate-spigot.jar", content=response.content)


def get_worldedit_plugin() -> Plugin:
    """Gets the WorldEdit plugin.

    Returns
    -------
    :class:`Plugin`
        The name of the plugin file and the contents of the plugin file.
    """

    url = "https://dev.bukkit.org/projects/worldedit/files/latest"

    if (response := requests.get(url)).status_code != 200:
        raise RuntimeError("Failed to download WorldEdit plugin.")

    return Plugin(
        name="worldedit-bukkit.jar",
        content=response.content,
    )
