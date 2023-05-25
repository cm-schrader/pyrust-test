import pyrt

class dat:
    name = "record data"

    def __init__(self, wins) -> None:
        self.wins = wins

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


    record = pyrt.save_score(wins, dat(wins*2))

    print(f"{wins} wins")