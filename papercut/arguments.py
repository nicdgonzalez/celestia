import argparse


def register_commands(parser: argparse.ArgumentParser) -> None:
    parser.add_argument(
        "run",
        help="Runs the PaperMC server.",
    )

    parser.add_argument(
        "new",
        help="Creates a new PaperMC server.",
    )


def register_flags(parser: argparse.ArgumentParser) -> None:
    parser.add_argument(
        "--update-plugins",
        "-u",
        action="store_true",
        help="Updates the plugins in the 'plugins' directory.",
    )
