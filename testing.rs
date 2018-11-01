fn main() {
    let n = [0, 1, 2, 3, 4];
    let mut big_arr = Vec::new();
    let mut cuts = 0;
    
    while big_arr.len() < 16 {
        let mut i = 0;
        let mut arr = Vec::new();
        let mut last_cut = 0;
        while i < 4 {
            if (cuts >> i) & 1 == 1 {
                arr.push(&n[last_cut..i + 1]);
                last_cut = i + 1;
            }
            
            i += 1;
        }
        
        arr.push(&n[last_cut..]);
        big_arr.push(arr);
        
        cuts += 1;
    }
    
    for item in big_arr {
        print!("{:?}, ", item);
        
        let mut i = item
        
        println!("");
    }
}

/* if a is b c else if d e else f;

0000 if is else if else
0001 if is else if, else
0010 if is else, if else
0011 if is else, if, else
0100 if is, else if else
0101 if is, else if, else
0110 if is, else, if else
0111 if is, else, if, else
1000 if, is else if else
1001 if, is else if, else
1010 if, is else, if else
1011 if, is else, if, else
1100 if, is, else if else
1101 if, is, else if, else
1110 if, is, else, if else
1111 if, is, else, if, else

0 1 2 3 4 if, is, else, if, else
01 2 3 4 if is, else, if, else
02 1 3 4 if else, is, if, else
03 1 2 4
04 1 2 3
012 34
013 24
014 23
023 14
024 13
034 12
0123 4
0124 3
0134 2
0234 1
01234
0 12 34
0 13 24
0 123 4
0 124 3
0 134 2
0 1234 */