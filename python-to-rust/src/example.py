def greet(name:str) -> str:
    return f"Hello, {name}! This message is from Python."


def add(a, b):
    return a + b


def factorial(n):
    if n <= 1:
        return 1
    return n * factorial(n - 1)


def fibonacci(n):
    if n <= 0:
        return 0
    elif n == 1:
        return 1
    else:
        return fibonacci(n - 1) + fibonacci(n - 2)


def process_list(numbers):
    if not numbers:
        return {"sum": 0, "average": 0, "count": 0}

    total = sum(numbers)
    average = total / len(numbers)

    return {"sum": total, "average": average, "count": len(numbers)}


class Calculator:
    def __init__(self):
        self.history = []

    def add(self, a, b):
        result = a + b
        self.history.append(f"{a} + {b} = {result}")
        return result

    def subtract(self, a, b):
        result = a - b
        self.history.append(f"{a} - {b} = {result}")
        return result

    def get_history(self):
        return self.history

    def clear_history(self):
        self.history.clear()
