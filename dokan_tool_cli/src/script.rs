use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum)]
pub enum ShellType {
    #[value(name = "nushell")]
    NuShell,
}

const SCRIPT_NU_SHELL: &str = r#"

# Custom commands for Nushell
def --env dokan [...args: string] {
	if (($args | length) < 1) {
		dokan-cli
		return
	}

	let arg0 = $args | get 0

	# cd
	if $arg0 == "cd" {
		let result = (dokan-cli ...$args)



	# other
	} else {
		dokan-cli ...$args
	}
}

"#;

pub fn print_script() {
    println!("{}", SCRIPT_NU_SHELL);
}