from cleansweep.types.json import Json
from typing import TypeVar, Type, cast

T = TypeVar("T")
U = TypeVar("U")

class JsonReader:
    @classmethod
    def value_error(cls, obj: object, expected: Type[U]):
        raise ValueError(f"Object of type {type(obj).__name__} was expected to be of type {type(expected).__name__}")
    
    @classmethod
    def expected_type(cls, obj: Json, desired_type: Type[T]) -> T:
        if isinstance(obj, desired_type):
            return cast(T, obj)
        return cls.value_error(obj, desired_type)

    @classmethod
    def extract_bool(cls, obj: Json) -> bool:
        return cls.expected_type(obj, bool)

    @classmethod
    def extract_int(cls, obj: Json) -> int:
        return cls.expected_type(obj, int)

    @classmethod
    def extract_float(cls, obj: Json) -> float:
        return cls.expected_type(obj, float)

    @classmethod
    def extract_str(cls, obj: Json) -> str:
        return cls.expected_type(obj, str)

    @classmethod
    def extract_list_of_type(cls, obj: Json, desired_type: Type[T]) -> list[T]:
        validated_list = cls.expected_type(obj, list)

        for item in validated_list:
            if not isinstance(item, desired_type):
                return cls.value_error(item, desired_type)
        return cast(list[T], obj)

    @classmethod
    def extract_dict_of_type(cls, obj: Json, key_type: Type[T], value_type: Type[U]) -> dict[T, U]:
        validated_dict = cls.expected_type(obj, dict)

        for key, value in validated_dict.items():
            if not isinstance(key, key_type):
                return cls.value_error(key, key_type)
            if not isinstance(value, value_type):
                return cls.value_error(value, value_type)
        
        return cast(dict[T, U], obj)
