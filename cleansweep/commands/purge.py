from cleansweep.commands.command import Command

purge_kwargs: list[str] = []

class Purge(Command):
    def command(self, **kwargs: str) -> None:
        pass
