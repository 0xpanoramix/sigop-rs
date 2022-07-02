use sigop_core::optimizer::run;

fn main() {
    let optimized = run("myFunction(address)", 3, 2);

    match optimized {
        None => {
            println!("Either none optimization was found or an error has occurred")
        }
        Some(res) => {
            println!("Found this optimisation: {}", res)
        }
    }
}
