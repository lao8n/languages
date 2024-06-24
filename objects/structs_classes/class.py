# python class.py

class Employee:
    def __init__(self, id, name, stats, jersey):
        self.id = id
        self.name = name
        self.stats = stats
        self.jersey = jersey

    def __repr__(self):
        return (f"Employee(id={self.id}, name='{self.name}', "
                f"stats={self.stats}, jersey={self.jersey})")
def main():
    # init class
    e1 = Employee(3, "freeman", [3, 4, 5], [5]) # mutable list

    # update
    e1.id = 1
    e1.name = "ohtani"
    e1.stats[0] = 1
    e1.stats[1] = 2
    e1.stats[2] = 3
    e1.stats.append(4)
    e1.jersey[0] = 17

    # print
    print(e1)  # Employee(id=1, name='ohtani', stats=[1, 2, 3, 4], jersey=[17])

if __name__ == "__main__":
    main()