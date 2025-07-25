from abc import ABC, abstractmethod

class Command(ABC):
    @abstractmethod 
    @staticmethod
    def command(*args, **kwargs: str) -> None:
        pass
