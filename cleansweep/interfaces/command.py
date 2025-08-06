from abc import ABC, abstractmethod
from argparse import Namespace, _SubParsersAction 

class CommandInterface(ABC):
    @staticmethod
    @abstractmethod 
    def command(args: Namespace) -> None:
        pass

    @classmethod
    @abstractmethod 
    def register_subparser(cls, subparsers: _SubParsersAction) -> None:
        pass
