from typing import TypeVar, Generic, Optional

T = TypeVar("T")

class Stack(Generic[T]):
    def __init__(self) -> None:
        self.__items: list[T] = []
        self.__numItems: int = 0

    def push(self, new_item: T) -> None:
        self.__items.append(new_item)
        self.__numItems += 1

    def pop(self) -> Optional[T]:
        if self.__numItems != 0:
            last_item = self.__items.pop()
            self.__numItems -= 1
            return last_item
        else:
            return None
    
    def expose_items(self) -> list[T]:
        return list(reversed(self.__items))

    def __len__(self) -> int:
        return self.__numItems
