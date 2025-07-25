from cleansweep.commands.command import Command
from cleansweep.types.json import Json 

setting_options: Json = {
    "arg" : [
        "display",
        "modify"
    ],
    "kwarg" : {
        "reset": [
            "all",
            "blacklist-all",
            "whitelist-all"
            # TODO: Continue trend
        ]
    }
}

# Rather than looping through the args/kwargs, loop through the 
class Settings(Command):
    @staticmethod
    def command(*args, **kwargs: str) -> None:
        if len(args) == 0 and len(kwargs) is 0:
            print("Please provide an expected additional parameter")

        if (len(args)+len(kwargs)) != 1:
            print("Please run only one command at a time")
        else:
            command_found: bool = False
            # Loop over args/kwargs to see if the command exists and how to run it
            if len(args) != 0:
                # INFO: Some LSPs flag this as an error when it is 100% valid
                for arg_command in setting_options["arg"]: # type: ignore
                    if arg_command in args:
                        command_found = True
                        # TODO: Match the command to the correct function

            if len(kwargs) != 0:
                for arg_key, arg_values in setting_options["kwarg"].items(): # type: ignore
                    # INFO: .get can return a None, but python would just evaluate to False - LSP doesnt care
                    if kwargs.get(arg_key) in arg_values: # type: ignore
                        command_found = True 
                        # TODO: Match the command to the correct function


            if not command_found:
                print("Your command option was not supported")

    @staticmethod
    def arg_display():
        pass
    
    @staticmethod 
    def arg_modify():
        pass

    @staticmethod 
    def kwarg_reset(param: str):
        pass
