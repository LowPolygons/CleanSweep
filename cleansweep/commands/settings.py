from cleansweep.commands.command import Command

settings_kwargs: list[str] = []

class Settings(Command):
    def command(self, **kwargs: str) -> None:
        pass
