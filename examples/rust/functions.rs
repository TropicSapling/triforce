#[allow(unused)]
fn timestimes_ppl(base_ppl: isize, exp_ppl: usize) -> isize {
    unsafe {
        let mut res_ppl = std::mem::uninitialized();
        exp_ppl == 0 && {
            res_ppl = 1;
            true
        } || {
            res_ppl = if_else_ppl(
                exp_ppl % 2 == 0,
                timestimes_ppl(base_ppl, exp_ppl / 2) * timestimes_ppl(base_ppl, exp_ppl / 2),
                base_ppl
                    * timestimes_ppl(base_ppl, exp_ppl / 2)
                    * timestimes_ppl(base_ppl, exp_ppl / 2),
            );
            true
        };
        res_ppl
    }
}
#[allow(unused)]
use std::fs::File as FileHandler;
fn backwards_println_ppl(i_ppl: isize) {
    unsafe {
        let mut res_ppl = std::mem::uninitialized();
        true && {
            res_ppl = {
                println!("{}", i_ppl);
            };
            true
        };
        res_ppl
    };
    unsafe {
        let mut res_ppl = std::mem::uninitialized();
        9 + 10 == 21
            || 10 + 9 == 21 && {
                res_ppl = {
                    println!("{}", false);
                };
                true
            }
            || {
                res_ppl = if_else_ppl(
                    9 + 10 != 21 && 10 + 9 != 21,
                    {
                        if_ppl(true, {
                            println!("{}", true);
                            println!("{}", true);
                        });
                    },
                    {
                        println!("{}", false);
                    },
                );
                true
            };
        res_ppl
    };
    println!("{}", i_ppl);
}
fn plusplus_plus_plusplus_ppl(a_ppl: isize, b_ppl: isize, c_ppl: isize, d_ppl: isize) -> isize {
    return a_ppl + b_ppl + c_ppl + d_ppl;
}
fn plusplus_minus_plusplus_ppl(a_ppl: isize, b_ppl: isize, c_ppl: isize, d_ppl: isize) -> isize {
    a_ppl + b_ppl - c_ppl - d_ppl
}
fn no_args_ppl() {
    println!("{}", "Function without args successfully called.");
}
fn main() {
    backwards_println_ppl(
        plusplus_plus_plusplus_ppl(
            1,
            2,
            3 - 1 * 4,
            plusplus_minus_plusplus_ppl(5 + 1, 6 + 1, 7, 8),
        ) - 1,
    );
    let mut res_ppl = 1 * 2
        + {
            println!("{}", "This was printed from inside an expression!");
            timestimes_ppl(3, 4)
        } * 5
        + 6;
    {
        res_ppl += 123;
        true
    };
    println!("{}", res_ppl);
    no_args_ppl();
    {};
    println!("{}", {});
    {};
}
