Read two binary numbers separated by a space (like "101 11") and print their sum in binary
Each number is read until a space or newline or EOF
Cells are 8 bit so the sum must be at most 11111111 (255)

c0 is left as 0 and used as a stop marker when printing
>

Reading a number  cells: N G C T  (N: value  G: continue flag  C: char  T: temp)
starts and ends at N
  loop while G
    read char into C (C is cleared first so EOF gives 0)
    if C is nonzero (not EOF)
    and C minus 10 is nonzero (not newline)
      if C minus 32 is nonzero (not space)
        G = 1
        N = N * 2
        N = N plus (C minus 48)

first number into c1  (uses c2 to c4)
>+[->[-],[----------[----------------------[<+><<[>>>++<<<-]>>>[<<<+>>>-]<----------------[<<+>>-]]]]<]<

second number into c5  (uses c6 to c8)
>>>>
>+[->[-],[----------[----------------------[<+><<[>>>++<<<-]>>>[<<<+>>>-]<----------------[<<+>>-]]]]<]<

c1 = c1 plus c5
[<<<<+>>>>-]<<<<

Convert to binary by repeatedly halving
Halving  cells: n r q e  (n: value  r: n mod 2  q: n div 2  e: temp flag)
  loop while n
    n = n minus 1
    if r is 1 then (r = 0 and q = q plus 1) else r = 1
Then store r plus 1 in the r cell (1 means bit 0 and 2 means bit 1)
and continue with q as the next n (two cells to the right)
Bits end up in c2 c4 c6 and so on (lowest bit first)

first halving always runs so that a sum of 0 prints "0"
[->>>+<<[->+>-<<]>>[-<<+>>]<<<]>+>
repeat while q is nonzero
[[->>>+<<[->+>-<<]>>[-<<+>>]<<<]>+>]

Print bits from highest to lowest  (stops at c0)
each bit cell holds 1 or 2 so add 47 to get the character 0 or 1
<[>+++++++[<+++++++>-]<--.[-]<<]

newline
++++++++++.
