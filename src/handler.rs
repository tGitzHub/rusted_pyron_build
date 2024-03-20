fn match_maker{
    match matches.subcommand() {
    Some(("install", install_matches)) => {
        let package_name = install_matches.value_of("package").unwrap();
        packages::install_package(package_name);
    }
    _ => {
        println!("Invalid command");
    }
} 
}