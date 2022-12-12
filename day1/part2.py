def main():

    calories = 0
    size = 3
    max_calories = [0]*size

    file = open("input.txt", encoding="utf-8")
    #file = open("test.txt", encoding="utf-8")

    for line in file:
        if (line == '\n'):
            calories = 0
        else:
            calories += int(line)
        if calories > max_calories[0]:
            max_calories[0] = calories
            max_calories.sort()

    
    print(max_calories)
    print(sum(max_calories))

main()