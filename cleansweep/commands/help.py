from cleansweep.commands.command import Command

# arg name : arg options
help_kwargs: dict[str, list[str]] = {
    "command" : [
        "list",
        "purge",
        "scan",
        "settings"
    ],
    "storage" : [
        "log",
        "whitelist"
    ],
    "filtering" : [
        "blacklist",
        "whitelist"
    ]
}

class Help(Command):
    def command(self, **kwargs: str) -> None:
        pass
