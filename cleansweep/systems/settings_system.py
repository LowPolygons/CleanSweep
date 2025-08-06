from cleansweep.containers.user_settings import UserSettings

class InteractiveSettingsSystem:
    def __init__(self) -> None: 
        self.__user_settings: UserSettings
        pass 
        # ISSUE: Should it own the UserSettings?

    def environment(self, session_close_cutoff: int) -> None:
        # Create a timer for modifying settings incase of inactivity which will then immediately break
        # Interactive environment for updating the settings for the user
        user_input: str = ""

        while (user_input != "exit"):
            user_input = input("Enter anything, or type exit to close the session:\n")
