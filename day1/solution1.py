def main(filepath):
    with open(filepath) as input_file:
        lines = input_file.read().split("\n")

    numbers = []
    for line in lines:
        if not line:
            continue

        digits = [char for char in line if char.isdigit()]
        first_and_last_digits = "".join([digits[0], digits[-1]])
        numbers.append(int(first_and_last_digits))

    return sum(numbers)


if __name__ == "__main__":
    print(main("input.txt"))
