use std::time::SystemTime;
//D-H密码交换
fn is_prime(n: u128) -> bool {
    !(2..(n)).any(|x| n % x == 0)
}
fn max_prime(n: u128) -> u128 {
    let mut num = n;
    while !is_prime(num) {
        num = num - 1;
    }
    num
}
// x^y mod p 如何计算 13^15679 mod 458
fn pow_mod(x: u128, y: u128, p: u128) -> u128 {
    if y == 0 {
        return 1;
    } else {
        let z = pow_mod(x, y / 2, p); //如果y=5_u128, y/2=2
        if y % 2 == 0 {
            return ((z % p) * (z % p)) % p;
        } else {
            return ((((x % p) * (z % p)) % p) * (z % p)) % p;
        }
    }
}

//求p 原始根 的集合,最多产生元根的上限，从小至大
fn generator_primitive_root(p: u128, num: u128) -> Vec<u128> {
    if !is_prime(p) {
        panic!("p:{} is not prime!", p);
    }
    let mut values: Vec<u128> = Vec::new();
    let target = (1..p).map(|x| x as u32 as u128).collect::<Vec<u128>>();
    // println!("target:{:?}",target);
    // (1..p).map(|y| {print!("2, {}, {} ",y as u32 as u128, p);let powmod = pow_mod(2, y as u32 as u128, p);print!(" {} ;", powmod); powmod}).collect::<Vec<u128>>();
    // println!("p:{}, map {:?}",p, );
    for i in 1..p {
        if values.len() as u128 > num {
            return values;
        }
        let mut temp_value: Vec<u128> = (1..p).map(|y| pow_mod(2, y, p)).collect::<Vec<u128>>();
        temp_value.sort();
        if temp_value == target {
            // println!("temp_value:{:?}",temp_value);
            values.push(i);
            //println!("value:{:?}=>{}",temp_value,x);
        }
    }
    println!("元根value:{:?}", values);
    values
}
//求 指数i, 即为b(任意数)的以a为基数的模p的离散对数。
#[allow(dead_code)]
fn dis_log(b: u128, a: u128, p: u128) -> u128 {
    //对于任意数b及素数p的原始根a，可以找到一个唯一的指数i，满足：b=( a ^ i) mod p，其中 0≤i≤p-1
    let data: Vec<u128> = (0..p - 1).filter(|&i| pow_mod(a, i, p) == b).collect();
    println!("dis_log=> i ={:?}", data);
    data[0]
}
fn test_func(randnum: u128) -> u128 {
    // let mut data = (0..12* 1024 * 1024 / 4)
    //     .map(|y| y as u32)
    //     .collect::<Vec<u32>>();
    // data[1024]=9;
    let max_prime = max_prime(randnum); //求出randnum中最大的质数

    let groups_primitive_root = generator_primitive_root(max_prime, 1); //50个原根
    if groups_primitive_root.len() <= 0 as usize {
        println!("generate groups_primitive_root error!");
    }
    let g_primitive_root = *&groups_primitive_root[0]; //10为随机取，指取第11个元根集合数据
    let p = max_prime;
    let a_private_key = 14_u128; // no open
    let b_private_key = 39_u128; // no open
    let a_send_to_b_num = pow_mod(g_primitive_root, a_private_key, p);
    let b_send_to_a_num = pow_mod(g_primitive_root, b_private_key, p);
    let a_compute_key_num = pow_mod(b_send_to_a_num, a_private_key, p);
    let b_compute_key_num = pow_mod(a_send_to_b_num, b_private_key, p);
    if a_compute_key_num != b_compute_key_num {
        // println!("compute faile, that a is not equal to b!{}",data.len());
        println!("compute faile, that a is not equal to b!");
    }
    // println!("测试中的参数：");
    // println!("g_primitive_root : {:?}", g_primitive_root);
    // println!("a_send_to_b_num  : {:?}", a_send_to_b_num);
    // println!("b_send_to_a_num  : {:?}", b_send_to_a_num);
    // println!("a_compute_key_num: {:?}", a_compute_key_num);
    // println!("b_compute_key_num: {:?}", b_compute_key_num);

    // println!("\n真实环境中的参数：");
    // 真实中的加密参数
    // let g: u128 = 113;
    // let p: u128 = 2_u128.pow(64) - 1; //巨大的质数
    // let a_private_key = 19; //保密
    // let b_private_key = 23; //保密
    // let a_send_to_b_num = pow_mod(g, a_private_key, p);
    // let b_send_to_a_num = pow_mod(g, b_private_key, p);
    // // println!("真实环境中的a发送的值：{}", a_send_to_b_num);
    // // println!("真实环境中的b发送的值：{}", b_send_to_a_num);
    // let a_compute_key_num = pow_mod(b_send_to_a_num, a_private_key, p);
    // let b_compute_key_num = pow_mod(a_send_to_b_num, b_private_key, p);
    // if a_compute_key_num != b_compute_key_num {
    //     println!("real compute faile, that a is not equal to b!");
    // }
    // println!("真实环境中a_compute_key_num:{:?}", a_compute_key_num);
    // println!("真实环境中b_compute_key_num:{:?}", b_compute_key_num);

    pow_mod(48_u128, 23_u128, 187)
}

fn main() {
    println!("pow_mod:{}", pow_mod(2, 1, 1259));
    for i in 1..129 {
        let mut data = (0..i * 1024 * 1024 / 4)
            .map(|y| y as u32)
            .collect::<Vec<u32>>();
        let mut alltime: u128 = 0;
        // for d in &mut data {
        //     *d += 2;
        // }
        for _j in 0..3 {
            let randnum = 1259_u128; //随机输入一个较大的值
            let sy_time = SystemTime::now();
            let _result = test_func(randnum);
            let usetime = SystemTime::now()
                .duration_since(sy_time)
                .unwrap()
                .as_micros();
            alltime += usetime;
        }
        for d in &mut data {
            *d += 2;
        }
        println!("used memory:{:>#5}M，3times all time:{:>#10}us, {:>#7}s; using time:{:>#10}us, data[0]:{}, data.len():{:x}",
        (data.len() as f32)/1024.0/1024.0*4.0, alltime,alltime/1000_000, alltime/10, data[0], data.len());
    }
}
