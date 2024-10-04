```
                                             __        
                      __                    /\ \       
   __     __   __  __/\_\  __  __  __     __\ \ \____  
 /'__`\ /'_ `\/\ \/\ \/\ \/\ \/\ \/\ \  /'__`\ \ '__`\ 
/\  __//\ \L\ \ \ \_\ \ \ \ \ \_/ \_/ \/\  __/\ \ \L\ \
\ \____\ \____ \ \____/\ \_\ \___x___/'\ \____\\ \_,__/
 \/____/\/___L\ \/___/  \/_/\/__//__/   \/____/ \/___/ 
          /\____/                                      
          \_/__/    
```
# Rust WebAssembly Template with egui

This repository is a template for creating web applications using Rust, egui, and eframe. It allows you to write your application in Rust and compile it to WebAssembly for deployment on the web.

## Instructions

### Prerequisites

#### Ensure you have the following tools installed:
- Rust: Install Rust and cargo via rustup.
- wasm-bindgen-cli: Install using `cargo install wasm-bindgen-cli`.
- wasm32-unknown-unknown target: Add using `rustup target add wasm32-unknown-unknown`.

### Building the Project

Run the build script to compile the project and generate the necessary files for deployment:

```shell
./build.sh
```

#### This script will:

- Compile your Rust code to WebAssembly in release mode.
- Generate JavaScript bindings using wasm-bindgen.
- Copy index.html to the dist directory.
- Place all the necessary files into the dist directory.

### Customization

#### Changing the Application Name

If you want to change the name of your application:

1.	Update Cargo.toml
Change the name field:

```toml
[package]
name = "your-new-app-name"
```

2.	Update build.sh
Modify the APP_NAME variable:

```shell
APP_NAME="your-new-app-name"
```

3.	Update index.html
Update the import path to match the new application name:

```html
<script type="module">
    import init from './your-new-app-name.js';
    init();
</script>
```

### Modifying the Rust Code

Edit `src/main.rs` to change the application’s behavior. The MyApp struct and its implementation define what gets displayed.

### Updating the HTML

Modify index.html to change the web page’s appearance or structure. Ensure the canvas element’s ID matches the one used in main.rs.

### License

This project is licensed under the [MIT License](LICENSE).