use blub::lut::cos;
use blub::lut::pow2;
use blub::lut::pow_e;

fn main() {
    print!("cos(0) = {}\n", cos(0.0));

    print!("pow2(0) = {}\n", pow2(0.0));
    print!("pow2(1) = {}\n", pow2(1.0));
    print!("pow2(-1) = {}\n", pow2(-1.0));
    print!("pow2(0.5) = {}\n", pow2(0.5));
    print!("pow_e(1.0) = {}\n", pow_e(1.0));
}
