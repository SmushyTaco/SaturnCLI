use std::env;
mod one_argument_envirnment_variable_commands;
mod two_or_more_argument_envirnment_variable_commands;

fn main() {
	//Grabs arguments user inputs
	let args: Vec<String> = env::args().collect();
	//For some reason no arguments are equal to one so this checks if there are no arguments.
	if args.len() == 1 {
		one_argument_envirnment_variable_commands::help_menu();
		//For some reason 1 argument is equal to two so this checks if there is 1 argument.
	} else if args.len() == 2 {
		one_argument_envirnment_variable_commands::if_checks_for_things_with_one_argument();
	} else {
		two_or_more_argument_envirnment_variable_commands::if_checks_for_things_with_two_or_more_arguments();
	}
}
