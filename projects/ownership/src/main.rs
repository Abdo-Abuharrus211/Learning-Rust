mod ownership_and_functions;

fn main() {
    // let s1 = String::from("hello");
    // let s2 = s1;
    //
    // println!("{s2}, world!");

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");


    let s = String::from("Bob man");
    ownership_and_functions::takes_ownership(s);
    let x = 5;
    ownership_and_functions::makes_copy(x);
    println!("X's value is {x}");


    let salad = String::from("Love my salads with some Sumac in them!");

    let first_word = first_word(&salad);

    println!("The first word is \"{first_word}\"");

    let arr = [1,2,3,4,5];
    println!("{}",arr[1]);
    let arr_slice = &arr[2..4];
    for &i in arr_slice.iter(){
        println!("{}", i);
    }

}


fn first_word(input: &String) -> &str{
    let bytes = input.as_bytes(); // convert String to array of Bytes
    for(i, &j) in bytes.iter().enumerate(){
        if j == b' '{
            return &input[0..i]
        }
    }
    &input[..]
}