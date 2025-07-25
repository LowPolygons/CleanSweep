from abc import ABC, abstractmethod

class Command(ABC):
    @abstractmethod
    def command(self, **kwargs: str) -> None:
        pass
