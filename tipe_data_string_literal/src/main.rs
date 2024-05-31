fn main() {
    // Escape karakter menggunakan \
    let var2 = "hello \
    \"rust\" \
    and \
    \"world\"";
    println!("{}", var2);

    // Multiline string literal
    let var3 = "baris satu
    baris dua
    baris tiga";
    println!("{}", var3);

    // Raw string
    let var5 = r#"
    {
        "name": "tim drake",
        "gender": "male"
    }
"#;
    println!("{}", var5);

    let var6 = "
    {
        \"name\": \"cassandra cain\",
        \"gender\": \"female\"
    }
";
    println!("{}", var6);
}
