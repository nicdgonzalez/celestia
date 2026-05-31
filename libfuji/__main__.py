import logging
import sys

from .fuji import Fuji, command_aliases, commands

_log = logging.getLogger(__name__)


def main() -> int:
    """The main entry-point for the application.

    Returns
    -------
    :class:`int`
        The exit code.
    """

    fuji = Fuji()

    if len(sys.argv) < 2:
        fuji.help()
        return 1

    command_name = sys.argv[1]

    if command_name in command_aliases:
        command_name = command_aliases[command_name]

    try:
        callback = commands[command_name]["callback"]
    except KeyError:
        _log.error(f"ERROR: Fuji: Command '{command_name}' not found")
        return 1
    else:
        callback(fuji, *sys.argv[2:])

    return 0


if __name__ == "__main__":
    sys.exit(main())
