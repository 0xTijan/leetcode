/*
1+1=0 c 1
1+0=1 c 0
0+1=1 c 0
0+0=0 c 0
c 1 1+1=1 c 1
*/
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        // start from the back
        let mut shorter = a.chars().rev();
        let mut longer = b.chars().rev();

        if b.len() < a.len() {
            shorter = b.chars().rev();
            longer = a.chars().rev();
        }

        let mut res: Vec<String> = Vec::new();
        let mut c: i32 = 0; // carry

        for (x, y) in shorter.zip(longer.by_ref()) {
            let mut x_num: i32 = x.to_string().parse().unwrap();
            let mut y_num: i32 = y.to_string().parse().unwrap();

            if c > 0 && x_num + y_num == 2 {
                c = 1;
                res.push("1".to_string());
            } else {
                // update 0 to 1
                if x_num == 0 {
                    x_num = c;
                } else if x_num > 0 && y_num == 0 {
                    y_num = c
                }

                if x_num + y_num == 2 {
                    c = 1;
                    res.push("0".to_string());
                } else if x_num + y_num == 1 {
                    c = 0;
                    res.push("1".to_string());
                } else if x_num + y_num == 0 {
                    c = 0;
                    res.push("0".to_string());
                }
            }
        }

        // append unused part of the longer num
        for y in longer {
            let mut y_num: i32 = y.to_string().parse().unwrap();
            y_num += c;

            if y_num == 2 {
                c = 1;
                res.push("0".to_string());
            } else {
                c = 0;
                res.push(y_num.to_string());
            }
        }

        // add carry if it is there
        if c == 1 {
            res.push("1".to_string());
        }

        res.reverse();
        res.join("")
    }
}