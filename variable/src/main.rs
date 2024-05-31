fn main() {
    println!("Hello, world!");
    println!("I'm a Rustacean!");
    println!("I'm a DevOps Engineer!");

    // Deklarasi variabel menggunakan let
    let nama_depan = "Rizqi";
    println!("Nama saya adalah {}", nama_depan);

    // Immutability pada variabel
    // Perlu diketahui, bahwa by default semua variabel adalah immutable. Immutable itu artinya gak bisa diubah nilai/value-nya. Jadi mirip seperti konstanta.
    let mut message_number = 1;
    let message1 = "Hello,";
    let message2 = "Rizqi!";
    println!("message number {}: {} {}", message_number, message1, message2);

    message_number = 3;
    let message3 = "Hello, number2!";
    println!("message number {}: {}", message_number, message3);

    // Argument parameter macro println
    /*
    Jika dilihat ada yg berbeda pada cara deklarasi variabel message3 dan juga pada statement println untuk message3 yang di situ digunakan {1} dan {0}, tidak seperti sebelumnya yg menggunakan {}. Kita akan bahas yg ke-2 terlebih dahulu.

    Jika menggunakan {}, maka string akan di-replace sesuai urutan argument pada pemanggilan println.
    Jika menggunakan {0}, maka string akan di-replace dengan data pada argument ke 1 pemanggilan fungsi println, yang pada contoh di atas adalah message3.
    Jika menggunakan {1}, maka string akan di-replace dengan data pada argument ke 2 pemanggilan fungsi println, yang pada contoh di atas adalah message_number.
    Jika menggunakan {n}, maka string akan di-replace dengan data pada argument ke n+1 pemanggilan fungsi println.

     */

    let mut message_number = 1;
    let message1 = "Hello";
    println!("Message number {}: {}", message_number, message1);

    message_number = 2;
    let message2 = "World!";
    println!("Message number {}: {}", message_number, message2);

    message_number = 3;
    let message3: i8 = 24;
    println!("Message number {}: {}", message_number, message3);

    //Deklarasi variabel tanpa predefined value
    let message_number: i32;
    message_number = 1;
    println!("message number {}", message_number);

    // Deklarasi banyak variabel dalam satu statement
    let (var1, var2) = (24, "hello");
    println!("var1: {0}", var1); // hasilnya => var1: 24
    println!("var2: {0}", var2); // hasilnya => var2: hello

    let (var3, var4): (i8, i8) = (32, 12);
    println!("var3: {0}", var3); // hasilnya => var3: 32
    println!("var4: {0}", var4); // hasilnya => var4: 12

    let (var5, mut var6, var7): (i8, i8, i8) = (64, 12, 4);
    println!("var5: {0}", var5); // hasilnya => var5: 64
    println!("var6: {0}", var6); // hasilnya => var6: 12
    var6 = 24;
    println!("var6: {0}", var6); // hasilnya => var6: 24
    println!("var7: {0}", var7); // hasilnya => var7: 4

    // Deklarasi variabel dengan tipe data ditentukan dari value
    let data1 = 24i8;
    println!("data1: {0}", data1); // hasilnya => data1: 24

    let data1 = 24_i8;
    println!("data1: {0}", data1); // hasilnya => data1: 24

    // Variable Shadowing
    let x = 5;
    println!("x: {}", x); // hasilnya => x: 5

    let x = x + 1;
    println!("x: {}", x); // hasilnya => x: 6
}
