# simple function

def print_hello():
    print("Hello from python")


# Basic math functions


def add(a: int, b: int) -> int:
    return a + b


# string manipulation


def greet(name: str) -> str:
    return f"Hello, {name}!"


# list processing


def square_list(vec: list[int]) -> list[int]:
    return [x * x for x in vec]


# A simple calculator class


class Calculator:
    def __init__(self, initial_value: float = 0.0):
        self.value = initial_value

    def add(self, amount: float) -> None:
        self.value += amount

    def get_value(self) -> float:
        return self.value

    def reset(self) -> None:
        self.value = 0.0


if __name__ == "__main__":
    
    print_hello()

    print(f"5 + 3 = {add(5, 3)}")
    print(greet("World"))
    print(f"Square of [1, 2, 3, 4] = {square_list([1, 2, 3, 4])}")

    calc = Calculator(10.0)
    print(f"Initial calculator value: {calc.get_value()}")
    calc.add(5.0)
    print(f"After adding 5.0: {calc.get_value()}")
    calc.reset()
    print(f"After reset: {calc.get_value()}")

    import numpy as np
    arr = np.array([1, 2, 3])
    print(f"Numpy array: {arr}, sum: {np.sum(arr)}")
