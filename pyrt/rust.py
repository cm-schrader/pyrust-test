import pyrt

wins = 0
while True:
    try:
        if pyrt.rps_round(input("Your Move: ")):
            wins += 1
            print("Round won")
        else:
            print("Round lost")
    except:
        print("Bad Input")

    print(f"{wins} wins")