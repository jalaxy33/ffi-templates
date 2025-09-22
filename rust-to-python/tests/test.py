from rust_module import sum_as_string, process_numbers, Calculator

def test_sum_as_string():
    assert sum_as_string(5, 3) == "8"


def test_process_numbers():
    assert process_numbers([1.0, 2.0, 3.0]) == [3.0, 5.0, 7.0]


def test_calculator():
    calc = Calculator(10.0)
    assert calc.get_value() == 10.0

    result = calc.add(5.0)
    assert result == 15.0
    assert calc.get_value() == 15.0

    calc.reset()
    assert calc.get_value() == 0.0


if __name__ == "__main__":
    print("sum_as_string(5, 3) = ", sum_as_string(5, 3))
    print("process_numbers([1.0, 2.0, 3.0]) = ", process_numbers([1.0, 2.0, 3.0]))

    calc = Calculator(10.0)
    print(f"Create Calculator: {calc}")

    result = calc.add(5.0)
    print(f"After adding 5.0: {calc}, result = {result}")
    print(f"Calculator value: {calc.get_value()}")

    calc.reset()
    print(f"After reset: {calc}, value = {calc.get_value()}")


    test_sum_as_string()
    test_process_numbers()
    test_calculator()
    print("All tests passed!")
