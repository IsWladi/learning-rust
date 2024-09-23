// const don't have type inference
// const variable can be declared in the global scope
const MY_CONST_VAR: i32 = 10;

fn main() {
    println!("Hello, world!");
    // this variables are immutables
    let my_var: i32 = 10;
    let my_var2: i32 = 20;
    let my_var3 = 30; // Rust can infer the type of the variable if it's not specified

    const MY_CONST_VAR2: i32 = 20;

    // this will not work because MY_CONST_VAR2 is not a const variable
    // const MY_CONST_VAR3: i32 = MY_CONST_VAR + my_var2;

    println!("my_var + my_var2: {}", my_var + my_var2);
    println!("my_var3: {my_var3}");
    println!("MY_CONST_VAR: {}", MY_CONST_VAR);
    println!("MY_CONST_VAR2: {}", MY_CONST_VAR2);
    shadowing(10);
}

fn shadowing(value: i32) {
    let shadowed_var: i32 = value;
    let shadowed_var = shadowed_var + 10;

    {
        let shadowed_var = shadowed_var + 10;
        println!("shadowed_var inner scope: {}", shadowed_var);
    }

    println!("shadowed_var outer scope: {}", shadowed_var);

    // we can change the type of the variable with shadowing
    let spaces = "   ";
    println!("spaces: {}", spaces);
    let spaces = spaces.len();
    println!("spaces: {}", spaces);
}
