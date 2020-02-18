
def bitboard_of(rank, file):
    y = 8 - rank
    x = file - 1
    return 1 << (y*8 + x)

def print_files():
    files = []
    for file in range(1, 9):
        val = 0
        for rank in range(1, 9):
            val |= bitboard_of(rank, file)
        files += [val]
    print(f"static FILES: [u64; 8] = {files};")

def print_ranks():
    ranks = []
    for rank in range(1, 9):
        val = 0
        for file in range(1, 9):
            val |= bitboard_of(rank, file)
        ranks += [val]
    print(f"static RANKS: [u64; 8] = {ranks};")

def main():
    print_ranks()
    print_files()

if __name__ == "__main__":
    main()