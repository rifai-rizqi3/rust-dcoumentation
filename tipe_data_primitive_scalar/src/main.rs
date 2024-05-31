fn main() {
    // Signed integers
    let numerik1 = 24;
    let numerik2: i8 = 2;
    let numerik3: i64 = 12;
    println!("{} | {} | {}", numerik1, numerik2, numerik3);

    let min_i8 = i8::MIN;
    let max_i8 = i8::MAX;

    println!("{} | {}", min_i8, max_i8);

    // Unsigned integers
    let numerik4: u32 = 28;
    let numerik5: u8 = 16;
    let numerik6: u64 = 42;

    println!("{} | {} | {}", numerik4, numerik5, numerik6);

    let min_u8 = u8::MIN;
    let max_u8 = u8::MAX;

    println!("{} | {}", min_u8, max_u8);

    // Floating point
    let fp1: f32 = 3.14;
    let fp2: f64 = 3.1415926535;

    println!("{} | {:.5}", fp1, fp2);

    let min_f32 = f32::MIN;
    let max_f32 = f32::MAX;

    println!("min_f32={} | max_f32={}", min_f32, max_f32);

    // Bool
    let b1 = true;
    let b2 = false;

    println!("{} | {}", b1, b2);

    // Char
    let c1 = 'n';
    let c2 = '-';
    let c3 = '2';

    println!("{} | {} | {}", c1, c2, c3);

    // Pointer scalar
    let ptr1: &i32 = &24;
    println!("{}", ptr1);
}
