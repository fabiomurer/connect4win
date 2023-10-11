# Database of all connect-4 positions with 12 pieces with their theoretical results and winning distances

**does not constains mirrored positions**

database decoded from [this database](https://github.com/MarkusThill/Connect-Four/blob/master/CFour/src/openingBook/bookDeepDist.dat) in more readable format.

thanks MarkusThill :)

## encoding

### position

es. `1.....11121.221222. 75`

        6 . . . . . . O
        5 . . . . . X O
        4 . . . . . O O
        3 . . . . . X X
        2 . . . . . X O
        1 X . . . . X O
          a b c d e f g

* `1` for `X` and `2` for `O`
* from the first column (a), from the bottom (1) add stone corrsiponding the character
* when `.` continue to add in the next column (a -> b -> c -> ..)

## value

* es. `75` 100-75 = 25, after 25 more move player1 win
* es. `-74` 100-74 = 26, after 26 more move player2 win

4200899 entrys