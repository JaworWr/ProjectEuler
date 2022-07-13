// https://en.wikipedia.org/wiki/Hook_length_formula
// to get the Young diagram, assign the values s.t. rows are of the form 2^x*3^l
// and columns are of the form 2^k*3^x, for constant k, l
// Hook length formula gives the number of valid assignments of indices to the values

pub fn solve() {
    const N: u64 = 10u64.pow(18);
    
    let mut ways = 0f64;
    // vals[i][j] = "2^i*3^j <= N"
    let mut val = [[false; 80]; 80];

    
    let mut n = 0;
    for i in 0..60 {
        for j in 0..50 {
            let k = 2u64.pow(i) * 3u64.pow(j);
            if k > N {
                break;
            }
            val[i as usize][j as usize] = true;
            n += 1;
            ways += (n as f64).ln();
        }
    }
    println!("{} {:.10e}", ways, ways.exp());

    for i in 0..80 {
        for j in 0..80 {
            if !val[i][j] {
                break;
            }
            let mut s = 1;
            for k in j+1..80 {
                if !val[i][k] {
                    break;
                }
                s += 1;
            }
            for k in i+1..80 {
                if !val[k][j] {
                    break;
                }
                s += 1;
            }
            ways -= (s as f64).ln();
        }
    }
    println!("{} {:.10e}", ways, ways.exp());
}
