/*
* Packages: A Cargo feature that lets you buld, test and share crates
* Crates: A tree of modules that procudes a library or executable
* Modules and use: Let you control the organization, scope and privacy of paths
* Paths: A way of naming an item, such as a struct, function or module
* */

/*
* if you run rustc rather than cargo and pass a single source code file,
* the compiler considers that file to be a crate. Crates can contain modules,
* and the modules may be defined in other files that get compiled with the crate.
* */

/*
* A crate can come in one of two forms: a binary crate or a library crate.
* Binary crates are programs you can compile to an executable that you can run,
* such as a command-line program or a server. Each must have a function called main
* that defines what happens when the executable runs.
* */

/*
* Library crates don’t have a main function, and they don’t compile to an executable.
* Instead, they define functionality intended to be shared with multiple projects.
* For example, the rand crate
* */

/* Most of the times crate referes to library crate and in general programming concept it is a library*/

/*
* A package is a bundle of one or more crates that provides a set of functionality.
* A package contains a Cargo.toml file that describes how to build those crates.
* Cargo is actually a package that contains the binary crate for the command-line tool to build your code.
* */

/* The Cargo package also contains a library crate that the binary crate depends on.
* Other projects can depend on the Cargo library crate to use the same logic the Cargo command-line tool
* uses. A package can contain as many binary crates as you like, but at most only one library crate.
* A package must contain at least one crate, whether that’s a library or binary crate.
* */

/*
* Cargo follows a convention that src/main.rs is the crate root of a binary crate with
* the same name as the package. Likewise, Cargo knows that if the package directory contains
* src/lib.rs, the package contains a library crate with the same name as the package and src/lib.rs
* is its crate root. Cargo passes the crate root files to rustc to build the library or binary.
* */
