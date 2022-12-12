def main():

    calories = 0
    max_calories = 0

    file = open("input.txt", encoding="utf-8")
    #file = open("test.txt", encoding="utf-8")
    
    for line in file:
        if (line == '\n'):
            calories = 0
        else:
            calories += int(line)
        if max_calories < calories:
            max_calories = calories
    
    print(max_calories)

main()