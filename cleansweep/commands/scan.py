from cleansweep.commands.command import Command

scan_kwargs: list[str] = []

class Scan(Command):
    def command(self, **kwargs: str) -> None:
        pass
