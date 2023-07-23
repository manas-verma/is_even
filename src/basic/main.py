import sys


def is_even(n):
    return n % 2 == 0


def main():
    value = int(sys.argv[1])
    parity = "even" if is_even(value) else "odd"
    print(f"{value} is {parity}")


if __name__ == '__main__':
    main()
