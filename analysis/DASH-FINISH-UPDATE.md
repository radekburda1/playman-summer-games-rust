# Race finish sequence

The previous version immediately replaced the stadium with results. It now enters a separate Finish state: the measured time remains frozen while the athlete continues moving, decelerates and then reacts. Confirmation after stopping opens detailed results.

Original source references: `d.e(c)` reduces speed by 630 per tick and stops below 4000, with the original dive adjustment of plus/minus 3900 around a 409600-unit distance threshold. `d.a(int,int[],int,int,int,int,int)` selects reaction animation tables. The record reaction uses table Z (3440, three keyframes, 80 ms) then aa (3446, thirteen keyframes, 80 ms loop); disappointment uses X (3402, fourteen keyframes, 120 ms) then Y (3430, five keyframes, 120 ms loop).

Rust now selects reactions independently of the personal record: 750+ points celebrates, 450–749 uses neutral animation W (3392), and lower scores use disappointment. Original challenge thresholds/rank-based reactions remain outside the single-player port. The finish overlay shows FINISH, the frozen time and score-based feedback while retaining the stadium. This was traced from decompiled methods and the supplied original screenshots; a live synchronized comparison has not been completed.

Two regression tests verify that run-out stops without changing the recorded time or result and that record/non-record outcomes select the recovered reaction sequences.

Feedback correction: original resource b0 offset 62 contains five big-endian score thresholds: 900,750,600,450,300. `d.c(event,result,rank)` maps them to strings 154–159: AWESOME!, GREAT!, NICE!, ALMOST THERE, NOT GOOD ENOUGH, OUCH. `d.n(result)` uses thresholds 750 and 450 for reaction selection. The high-score flag is now separate. Regression examples: 10.71 = 734/NICE; 9.65 = 1024/AWESOME; 12.82 = 344/NOT GOOD ENOUGH.

Result-panel correction: the finish time now uses original sprites 32 (left cap), 31 (tiled center, clipped to text width) and 33 (right cap), following the original numeric panel drawing method. The panel is centered on the measured bitmap-text width rather than stretched across the screen.
