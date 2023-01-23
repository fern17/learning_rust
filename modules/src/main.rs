mod my_mod {
    pub fn print_external() {
        println!("Printing from the exterior");
    }
    pub fn print_internal() {
        internal_function();
    }
    fn internal_function() {
        println!("Printing from the interior");
    }
    pub mod nested_module {
        pub fn print_external() {
            println!("Printing from the nested module");
        }
    }
}

mod module_in_file;

fn main() {
    my_mod::print_external();
    my_mod::print_internal();
    //my_mod::internal_function(); // fails, not visible
    my_mod::nested_module::print_external();
    module_in_file::print_another_file();
}
