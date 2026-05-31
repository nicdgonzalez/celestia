import sys

from .plugins import Floodgate, GeyserSpigot
from .server import PaperServer


def main() -> int:
    """Main entry point for the program."""

    server: PaperServer = PaperServer()

    server.install_plugin(GeyserSpigot(), "Geyser-Spigot.jar")
    server.install_plugin(Floodgate(), "Floodgate.jar")
