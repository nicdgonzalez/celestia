import logging
import subprocess

__all__ = ["TmuxSession"]


class TmuxSession:
    def __init__(self, name: str) -> None:
        self._log = logging.getLogger(__name__)
        self.name = name

    def exists(self) -> bool:
        """Check if a tmux session exists.

        Returns
        -------
        :class:`bool`
            Whether the session exists or not.
        """

        try:
            _ = subprocess.check_output(
                f"tmux has-session -t {self.name}",
                shell=True,
                stderr=subprocess.DEVNULL,
            )
        except subprocess.CalledProcessError:
            return False
        else:
            return True

    def start(self) -> None:
        """Start a tmux session."""

        if self.exists():
            self._log.warning(f"Session '{self.name}' already exists")
            return

        subprocess.run(
            f"tmux new-session -d -s {self.name}",
            shell=True,
        )

    def stop(self) -> None:
        """Stop a tmux session."""

        if not self.exists():
            self._log.warning(f"Session '{self.name}' does not exist")
            return

        subprocess.run(
            f"tmux kill-session -t {self.name}",
            shell=True,
        )
