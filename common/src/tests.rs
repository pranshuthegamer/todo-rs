use std::fmt::format;

use super::*;
use crate::schemas::*;

#[test]
fn hashing(){
    const TESTSIZE: usize = 400000;
    let mut testarray: Vec<[u8; 16]> = Vec::with_capacity(TESTSIZE);
    for i in 0..TESTSIZE {
        let password = format!("{}", i + 20000); 
        let password = password + &format!("{}", &i);
        let password = Password::new(&password);
        testarray.push(password.to_hash());
        for x in 0..i {
            if x < i {
                if testarray[i] == testarray[x] {
                    panic!("check of {} against {} failed at iteration", &i, &x);
                }
            }
        }
        println!("{:#08}   hash {}", &i, testarray[i].format_hex(&' '));
    }
}
