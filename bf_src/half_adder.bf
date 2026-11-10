Half adder
Input: two bits A and B separated by a space (like "1 1")
Output: S and C separated by a space (S = A xor B  C = A and B)

   A B | S C
   0 0 | 0 0
   0 1 | 1 0
   1 0 | 1 0
   1 1 | 0 1

Read input  c0 = A  c1 = B  (ASCII)
,>,[-],

Subtract 48 (the character 0) from c0 and c1 using c2 as a counter
>++++++[<--------<-------->>-]<

c0 = A plus B  (0 or 1 or 2)
[<+>-]<

Half adder  cells: c0 = A plus B  c1 = S  c2 = C
  if c0 is at least 1: c0 = c0 minus 1  S = 1
    if c0 is still at least 1 (both bits were 1): c0 = c0 minus 1  S = 0  C = 1
[->+<[->->+<<]]

Print S  space  C  newline
add 48 to S and C using c3 as a counter
>>>++++++[<++++++++<++++++++>>-]
<<.
space (32) built in c4 using c3 as a counter
>>++++++++[>++++<-]>.
print C (c2)
<<.
newline (c3 is 0)
>++++++++++.
