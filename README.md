# tab-player-cli

**NOTE: Not easily useable until a better method of loading samples is figured out.**

Write string instrument tabs in a plain text file and play them in the command line!

Empty template example:

```
   1/4         2/4         3/4         4/4
    |           |           |           |           |           |           |           |
E4 |-- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --|-- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --|
B3 |-- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --|-- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --|
G3 |-- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --|-- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --|
D3 |-- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --|-- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --|
A2 |-- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --|-- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --|
E2 |-- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --|-- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --|
```

Insert fret numbers and note durations with `>>`: 
```
   1/4         2/4         3/4         4/4
    |           |           |           |           |           |           |           |
E4 |-- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --|-- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --|
B3 |-- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --|-- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --|
G3 |-- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --|-- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --|
D3 |-- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --|-- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --|
A2 | 7 >> >> >> >> >>  7 >> 10 >> >>  7 >> >>  5 >>| 3 >> >> >> >> >> >> >> -- -- -- -- -- -- -- --|
E2 |-- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --|-- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --|
```
then run `tab-player-cli --play <TAB_FILE> <SAMPLES_FOLDER>`
