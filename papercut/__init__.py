"""
Papercut
========

A command-line tool for managing PaperMC Minecraft servers.
"""

from typing import Literal, NamedTuple


class VersionInfo(NamedTuple):
    major: int
    minor: int
    micro: int
    release: Literal["alpha", "beta", "candidate", "final"]
    serial: int


version_info = VersionInfo(0, 1, 0, "alpha", 0)

__title__ = "papercut"
__author__ = "Nicolas Gonzalez"
__license__ = "Apache 2.0"
__version__ = "{0.major}.{0.minor}.{0.micro}".format(version_info)
__copyright__ = f"Copyright (c) 2023-present {__author__}"

del Literal, NamedTuple, VersionInfo
