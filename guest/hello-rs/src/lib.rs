// Use a procedural macro to generate bindings for the world we specified in
// `demo.wit`
wit_bindgen_guest_rust::generate!({ path: "../../wit/demo.wit", world: "demo" });

// Define a custom type and implement the generated `Demo` trait for it which
// represents implementing all the necesssary exported interfaces for this
// component.
struct MyDemo;

impl exports::Exports for MyDemo {
    fn run() {
        imports::print("Hello from the guest (Rust)!");
    }
}

export_demo!(MyDemo);
