use std::io;

fn main() {
    /*
     * For windows: This tutorial only cover Code LDDB debugger: https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb from VS Code extensions
     * run debug mode by pressing F5
     * All the json configurations for debug are located in rust-basic-101/vscode/launch.json
     * Set breakpoint
     * CTRL+SHIFT+D for Run and Debug mode
     * 
     * Cascade Local within variable to see information about references pointer
     */
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    borrow_string(&input); // Set breakpoint to debug
    own_string(input); // Set breakpoint to debug
}

fn borrow_string(s: &String) {
    println!("{}", s)
}

fn own_string(s: String) {
    println!("{}", s)
}
