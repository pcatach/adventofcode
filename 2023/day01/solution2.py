SPELLED_OUT_DIGITS = {
    "one": 1,
    "two": 2,
    "three": 3,
    "four": 4,
    "five": 5,
    "six": 6,
    "seven": 7,
    "eight": 8,
    "nine": 9,
}


def convert_words_to_digits(line: str):
    i = 0
    converted_line = ""
    while i < len(line):
        char = line[i]
        for word, digit in SPELLED_OUT_DIGITS.items():
            if line[i : i + len(word)] == word:
                converted_line += str(digit)
        converted_line += char
        i += 1
    return converted_line


def main(filepath):
    with open(filepath) as input_file:
        lines = input_file.read().split("\n")

    numbers = []
    for line in lines:
        if not line:
            continue
        converted_line = convert_words_to_digits(line)
        digits = [char for char in converted_line if char.isdigit()]
        first_and_last_digits = "".join([digits[0], digits[-1]])
        numbers.append(int(first_and_last_digits))

    return sum(numbers)


if __name__ == "__main__":
    print(main("example2.txt"))
    # print(main("input.txt"))
