from abc import ABC, abstractmethod
from cleansweep.types.json import Json 
from typing import TypeVar, Generic

T = TypeVar("T")

class JsonCodecInterface(ABC, Generic[T]):
    @staticmethod
    @abstractmethod
    def encode_to_json(obj: T) -> Json:
        pass

    @staticmethod
    @abstractmethod
    def create_from_json(obj: Json) -> T:
        pass
