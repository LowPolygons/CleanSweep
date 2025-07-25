from cleansweep.commands.command import Command

# arg name
list_kwargs: list[str] = [
    "directory",
]

class List(Command):
    def command(self, **kwargs: str) -> None:
        pass
