Read two numbers separated by a space (like "12 23") and print their sum in decimal
Each number is read until a space or newline or EOF
Cells are 8 bit so the sum must be at most 255

Reading a number  cells: N G C T  (N: value  G: continue flag  C: char  T: temp)
starts and ends at N
  loop while G
    read char into C (C is cleared first so EOF gives 0)
    if C is nonzero (not EOF)
    and C minus 10 is nonzero (not newline)
      if C minus 32 is nonzero (not space)
        G = 1
        N = N * 10
        N = N plus (C minus 48)

first number into c0  (uses c1 to c3)
>+[->[-],[----------[----------------------[<+><<[>>>++++++++++<<<-]>>>[<<<+>>>-]<----------------[<<+>>-]]]]<]<

second number into c4  (uses c5 to c7)
>>>>
>+[->[-],[----------[----------------------[<+><<[>>>++++++++++<<<-]>>>[<<<+>>>-]<----------------[<<+>>-]]]]<]<

c0 = c0 plus c4
[<<<<+>>>>-]<<<<

divmod by 10: layout n 0 10 0 0 0 becomes 0 n 10minus(n mod 10) (n mod 10) (n div 10)
c3 = ones  c4 = n div 10
>>++++++++++<<
[->+>-[>+>>]>[+[-<+>]>+>>]<<<<<<]
>[-]>[-]>>
divmod c4 by 10 again: c7 = tens  c8 = hundreds
>>++++++++++<<
[->+>-[>+>>]>[+[-<+>]>+>>]<<<<<<]
>[-]>[-]>>

if hundreds nonzero: print hundreds then print tens (even if zero)
[<<<++++++[>>>++++++++<<<-]>>>.[-]<<<++++++[>>++++++++<<-]>>.[-]>]
else if tens nonzero: print tens
<[<<++++++[>>++++++++<<-]>>.[-]]
print ones (c3)
<<<<<++++++[>++++++++<-]>.
newline
[-]++++++++++.
