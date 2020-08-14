
def bitboard_of(rank, file):
    if rank < 1 or rank > 8:
        return 0
    if file < 1 or file > 8:
        return 0
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
    print(f"const FILES: [u64; 8] = {files};")

def print_ranks():
    ranks = []
    for rank in range(1, 9):
        val = 0
        for file in range(1, 9):
            val |= bitboard_of(rank, file)
        ranks += [val]
    print(f"const RANKS: [u64; 8] = {ranks};")

def print_rook_rays():
    rook_rays = list()
    for index in range(64):
        x = index % 8
        y = int(index / 8)
        rank = 8 - y
        file = x + 1
        ray = 0
        for slide in range(1, 9):
            ray |= bitboard_of(rank, slide)
            ray |= bitboard_of(slide, file)
        rook_rays.append(ray)
    print(f"const ROOK_RAYS: [u64; 64] = {rook_rays};")

def print_bishop_rays():
    bishop_rays = list()
    for index in range(64):
        x = index % 8
        y = int(index / 8)
        rank = 8 - y
        file = x + 1
        ray = 0
        for slide in range(1, 9):
            ray |= bitboard_of(rank + slide, file + slide)
            ray |= bitboard_of(rank - slide, file - slide)
            ray |= bitboard_of(rank - slide, file + slide)
            ray |= bitboard_of(rank + slide, file - slide)
        bishop_rays.append(ray)
    print(f"const BISHOP_RAYS: [u64; 64] = {bishop_rays};")

def print_knight_masks():
    knight_masks = list()
    for index in range(64):
        x = index % 8
        y = int(index / 8)
        rank = 8 - y
        file = x + 1
        mask = 0
        mask |= bitboard_of(rank - 2, file - 1)
        mask |= bitboard_of(rank - 2, file + 1)
        mask |= bitboard_of(rank + 2, file - 1)
        mask |= bitboard_of(rank + 2, file + 1)

        mask |= bitboard_of(rank - 1, file - 2)
        mask |= bitboard_of(rank - 1, file + 2)
        mask |= bitboard_of(rank + 1, file - 2)
        mask |= bitboard_of(rank + 1, file + 2)
        knight_masks.append(mask)
    print(f"const KNIGHT_MASKS: [u64; 64] = {knight_masks};")

def main():
    print_ranks()
    print_files()
    print_rook_rays()
    print_bishop_rays()
    print_knight_masks()

if __name__ == "__main__":
    main()