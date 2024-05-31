fn main() {
    // Operator aritmatika
    let (num1, num2) = (12, 4);

    let value_addition = num1 + num2;
    println!("{} + {} = {}", num1, num2, value_addition);
// output => 12 + 4 = 16

    let value_sub = num1 - num2;
    println!("{} - {} = {}", num1, num2, value_sub);
// output => 12 - 4 = 8

    let value_mut = num1 * num2;
    println!("{} * {} = {}", num1, num2, value_mut);
// output => 12 * 4 = 48

    let value_div = num1 / num2;
    println!("{} / {} = {}", num1, num2, value_div);
// output => 12 / 4 = 3

    let value_mod = num1 % num2;
    println!("{} % {} = {}", num1, num2, value_mod);
// output => 12 % 4 = 0

    let (num3, num4) = (10, 5);
    let hasil = num3 + num4;
    println!("{} + {} = {}", num3, num4, hasil);

    //Operator perbandingan
    /*
    Simbol	Kegunaan untuk mengecek
==	apakah kiri sama dengan kanan?
!=	apakah kiri tidak sama dengan kanan?
>	apakah kiri lebih besar dari kanan?
<	apakah kiri lebih kecil dari kanan?
>=	apakah kiri lebih besar atau sama dengan kanan?
<=	apakah kiri lebih kecil atau sama dengan kanan?
    */
    let number_a = 12;
    let number_b = 24;

    let res_one = number_a == number_b;
    println!("res_one: {res_one}");

    let res_two = number_a != number_b;
    println!("res_two: {res_two}");

    let res_three = number_a > number_b;
    println!("res_three: {res_three}");

    let res_four = number_a < number_b;
    println!("res_four: {res_four}");

    let res_five = number_a >= number_b;
    println!("res_five: {res_five}");

    let res_six = number_a <= number_b;
    println!("res_six: {res_six}");

    // Named argument macro println
    let res_one = number_a == number_b;
    println!("res_one: {res_one}");
// output => res_one: false

    let res_two = number_a != number_b;
    println!("res_two: {res_two}");
// output => res_one: true

    //Operator negasi
    /*
    Simbol	Kegunaan	Catatan
-	negasi numerik	bisa digunakan pada tipe data integer dan float
!	logika NOT atau bitwise NOT	bisa digunakan pada tipe data integer dan bool
    */

    let (value_left, value_right) = (12, -12);
    let res_one = -value_left == value_right;
    let res_two = !(value_left == value_right);
    println!("{res_one} {res_two}");
// output => true true
}
