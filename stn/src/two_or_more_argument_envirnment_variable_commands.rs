extern crate unicase;
use crate::one_argument_envirnment_variable_commands::help_menu;
use crate::one_argument_envirnment_variable_commands::if_checks_for_things_with_one_argument;
use std::process::Command;
use std::fs::OpenOptions;
use unicase::UniCase;
use std::path::Path;
use std::io::Write;
use std::process;
use std::env;
use std::io;
use std::fs;
fn file_path_return(mut file_name: String, files_flag_present: bool) -> String {
	//Make sure the file entree ends with .list
	if !file_name.contains(".list") {
		file_name = file_name.to_owned() + ".list";
	}
	if files_flag_present == true {
		if cfg!(target_os = "macos") && Path::new(&("/usr/local/etc/apt/sources.list.d/".to_owned() + &file_name)).exists() {
			return "/usr/local/etc/apt/sources.list.d/".to_owned() + &file_name
		} else if cfg!(target_os = "linux") && Path::new(&("/etc/apt/sources.list.d/".to_owned() + &file_name)).exists() {
			return "/etc/apt/sources.list.d/".to_owned() + &file_name
		} else if cfg!(target_os = "macos") && Path::new("/usr/local/etc/apt/sources.list.d/").exists() {
			println!("The file \"{}\" doesn't exist, would you like to create it? (Yes/No)", file_name);
			let mut choice = String::new();
			io::stdin().read_line(&mut choice).unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1);
			});
			let choice = choice.trim() as &str;
			if UniCase::new(choice) == UniCase::new("Yes") || UniCase::new(choice) == UniCase::new("Y") {
				OpenOptions::new()
					.write(true)
					.create_new(true)
					.open("/usr/local/etc/apt/sources.list.d/".to_owned() + &file_name)
					.unwrap_or_else(|err| {
						eprintln!("Application Error: {}", err);
						process::exit(1)
					});
				return "/usr/local/etc/apt/sources.list.d/".to_owned() + &file_name
			} else {
				println!("Skipping repository entree.");
				return String::from(". .")
			}
		} else if cfg!(target_os = "linux") && Path::new("/etc/apt/sources.list.d/").exists() {
			println!("The file \"{}\" doesn't exist, would you like to create it? (Yes/No)", file_name);
			let mut choice = String::new();
			io::stdin().read_line(&mut choice).unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1);
			});
			let choice = choice.trim() as &str;
			if UniCase::new(choice) == UniCase::new("Yes") || UniCase::new(choice) == UniCase::new("Y") {
				OpenOptions::new()
					.write(true)
					.create_new(true)
					.open("/etc/apt/sources.list.d/".to_owned() + &file_name)
					.unwrap_or_else(|err| {
						eprintln!("Application Error: {}", err);
						process::exit(1)
					});
				return "/etc/apt/sources.list.d/".to_owned() + &file_name
			} else {
				println!("Skipping repository entree.");
				return String::from(". .")
			}
		} else if cfg!(target_os = "macos") && Path::new("/usr/local/etc/apt/").exists() {
			println!("The folder \"sources.list.d\" doesn't exist, would you like to create it? (Yes/No)");
			let mut choice = String::new();
			io::stdin().read_line(&mut choice).unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1);
			});
			let choice = choice.trim() as &str;
			if UniCase::new(choice) == UniCase::new("Yes") || UniCase::new(choice) == UniCase::new("Y") {
				fs::create_dir("/usr/local/etc/apt/sources.list.d/").unwrap_or_else(|err| {
					eprintln!("Application Error: {}", err);
					process::exit(1);
				});
				println!("The file \"{}\" doesn't exist, would you like to create it? (Yes/No)", file_name);
				let mut choice = String::new();
				io::stdin().read_line(&mut choice).unwrap_or_else(|err| {
					eprintln!("Application Error: {}", err);
					process::exit(1);
				});
				let choice = choice.trim() as &str;
				if UniCase::new(choice) == UniCase::new("Yes") || UniCase::new(choice) == UniCase::new("Y") {
					OpenOptions::new()
					.write(true)
					.create_new(true)
					.open("/usr/local/etc/apt/sources.list.d/".to_owned() + &file_name)
					.unwrap_or_else(|err| {
						eprintln!("Application Error: {}", err);
						process::exit(1);
					});
					return "/usr/local/etc/apt/sources.list.d/".to_owned() + &file_name
				} else {
					println!("Skipping repository entree.");
					return String::from(". .")
				}
			} else {
				println!("Skipping repository entree.");
				return String::from(". .")
			}
		} else if cfg!(target_os = "linux") && Path::new("/etc/apt/").exists() {
			println!("The folder \"sources.list.d\" doesn't exist, would you like to create it? (Yes/No)");
			let mut choice = String::new();
			io::stdin().read_line(&mut choice).unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1);
			});
			let choice = choice.trim() as &str;
			if UniCase::new(choice) == UniCase::new("Yes") || UniCase::new(choice) == UniCase::new("Y") {
				fs::create_dir("/etc/apt/sources.list.d/").unwrap_or_else(|err| {
					eprintln!("Application Error: {}", err);
					process::exit(1)
				});
				println!("The file \"{}\" doesn't exist, would you like to create it? (Yes/No)", file_name);
				let mut choice = String::new();
				io::stdin().read_line(&mut choice).unwrap_or_else(|err| {
					eprintln!("Application Error: {}", err);
					process::exit(1);
				});
				let choice = choice.trim() as &str;
				if UniCase::new(choice) == UniCase::new("Yes") || UniCase::new(choice) == UniCase::new("Y") {
					OpenOptions::new()
					.write(true)
					.create_new(true)
					.open("/etc/apt/sources.list.d/".to_owned() + &file_name)
					.unwrap_or_else(|err| {
						eprintln!("Application Error: {}", err);
						process::exit(1)
					});
					return "/etc/apt/sources.list.d/".to_owned() + &file_name
				} else {
					println!("Skipping repository entree.");
					return String::from(". .")
				}
			} else {
				println!("Skipping repository entree.");
				return String::from(". .")
			}
		} else {
			println!("Skipping repository entree.");
			return String::from(". .")
		}
	} else {
		if cfg!(target_os = "macos") && Path::new("/usr/local/etc/apt/sources.list").exists() {
			return String::from("/usr/local/etc/apt/sources.list")
		} else if cfg!(target_os = "linux") && Path::new("/etc/apt/sources.list").exists() {
			return String::from("/etc/apt/sources.list")
		} else if cfg!(target_os = "macos") && Path::new("/usr/local/etc/apt/").exists() {
			println!("The file \"sources.list\" doesn't exist, would you like to create it? (Yes/No)");
			let mut choice = String::new();
			io::stdin().read_line(&mut choice).unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1);
			});
			let choice = choice.trim() as &str;
			if UniCase::new(choice) == UniCase::new("Yes") || UniCase::new(choice) == UniCase::new("Y") {
				OpenOptions::new()
					.write(true)
					.create_new(true)
					.open("/usr/local/etc/apt/sources.list")
					.unwrap_or_else(|err| {
						eprintln!("Application Error: {}", err);
						process::exit(1)
					});
				return String::from("/usr/local/etc/apt/sources.list")
			} else {
				println!("Skipping repository entree.");
				return String::from(". .")
			}
		} else if cfg!(target_os = "linux") && Path::new("/etc/apt/").exists() {
			println!("The file \"sources.list\" doesn't exist, would you like to create it? (Yes/No)");
			let mut choice = String::new();
			io::stdin().read_line(&mut choice).unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1);
			});
			let choice = choice.trim() as &str;
			if UniCase::new(choice) == UniCase::new("Yes") || UniCase::new(choice) == UniCase::new("Y") {
				OpenOptions::new()
					.write(true)
					.create_new(true)
					.open("/etc/apt/sources.list")
					.unwrap_or_else(|err| {
						eprintln!("Application Error: {}", err);
						process::exit(1)
					});
				return String::from("/etc/apt/sources.list")
			} else {
				println!("Skipping repository entree.");
				return String::from(". .")
			}
		} else {
			println!("Skipping repository entree.");
			return String::from(". .")
		}
	}
}

pub fn if_checks_for_things_with_two_or_more_arguments() {
	let mut args: Vec<String> = env::args().collect();
	if UniCase::new(&args[1]) == UniCase::new("search") || UniCase::new(&args[1]) == UniCase::new("info") {
		//Runs apt + first argument user inputted as lowercase + all other arguments.
		if cfg!(target_os = "windows") {
			Command::new("wsl").arg("--exec").arg("apt").arg(&args[1].to_lowercase()).args(&args[2..]).status().unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1)
			});
		} else {
			Command::new("apt").arg(&args[1].to_lowercase()).args(&args[2..]).status().unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1)
			});
		}
		process::exit(0);
	} else if UniCase::new(&args[1]) == UniCase::new("install") || UniCase::new(&args[1]) == UniCase::new("reinstall") {
		if cfg!(target_os = "windows") {
			//Runs sudo apt update
			Command::new("wsl").arg("--exec").arg("sudo").arg("apt").arg("update").status().unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1)
			});
			//Runs sudo apt + first argument user inputted as lowercase + all other arguments.
			Command::new("wsl").arg("--exec").arg("sudo").arg("apt").arg(&args[1].to_lowercase()).args(&args[2..]).status().unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1)
			});
		} else {
			//Runs sudo apt update
			Command::new("sudo").arg("apt").arg("update").status().unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1)
			});
			//Runs sudo apt + first argument user inputted as lowercase + all other arguments.
			Command::new("sudo").arg("apt").arg(&args[1].to_lowercase()).args(&args[2..]).status().unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1)
			});
		}
		process::exit(0);
	} else if UniCase::new(&args[1]) == UniCase::new("remove") {
		//Runs sudo apt purge + all other arguments.
		if cfg!(target_os = "windows") {
			Command::new("wsl").arg("--exec").arg("sudo").arg("apt").arg("purge").args(&args[2..]).status().unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1)
			});
		} else {
			Command::new("sudo").arg("apt").arg("purge").args(&args[2..]).status().unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1)
			});
		}
		process::exit(0);
	} else if UniCase::new(&args[1]) == UniCase::new("add-key") {
		//Runs sudo apt-key add + second argument the user inputted.
		if cfg!(target_os = "windows") {
			Command::new("wsl").arg("--exec").arg("sudo").arg("apt-key").arg("add").arg(&args[2]).status().unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1)
			});
		} else {
			Command::new("sudo").arg("apt-key").arg("add").arg(&args[2]).status().unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1)
			});
		}
		process::exit(0);
	} else if UniCase::new(&args[1]) == UniCase::new("upgrade") {
		if cfg!(target_os = "windows") {
			//Runs sudo apt update
			Command::new("wsl").arg("--exec").arg("sudo").arg("apt").arg("update").status().unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1)
			});
			//Runs sudo apt upgrade + all other arguments.
			Command::new("wsl").arg("--exec").arg("sudo").arg("apt").arg("upgrade").args(&args[2..]).status().unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1)
			});
		} else {
			//Runs sudo apt update
			Command::new("sudo").arg("apt").arg("update").status().unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1)
			});
			//Runs sudo apt upgrade + all other arguments.
			Command::new("sudo").arg("apt").arg("upgrade").args(&args[2..]).status().unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1)
			});
		}
		process::exit(0);
	} else if UniCase::new(&args[1]) == UniCase::new("full-upgrade") {
		if cfg!(target_os = "linux") {
			//Runs sudo apt update
			Command::new("sudo").arg("apt").arg("update").status().unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1)
			});
			//Runs sudo apt upgrade
			Command::new("sudo").arg("apt").arg("full-upgrade").args(&args[2..]).status().unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1)
			});
			process::exit(0);
		} else if cfg!(target_os = "windows") {
			//Runs sudo apt update
			Command::new("wsl").arg("--exec").arg("sudo").arg("apt").arg("update").status().unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1)
			});
			//Runs sudo apt upgrade
			Command::new("wsl").arg("--exec").arg("sudo").arg("apt").arg("full-upgrade").args(&args[2..]).status().unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1)
			});
			process::exit(0);
		} else {
			//Displays this message if the command the user entered doesn't exist.
			eprintln!("Unknown command {:?}, type \"stn help\" to see the list of available commands.", args[1].to_lowercase());
			process::exit(1);
		}
	} else if UniCase::new(&args[1]) == UniCase::new("add-repositories") || UniCase::new(&args[1]) == UniCase::new("add-repos") {
		//Runs add-repositories + all other arguments.
		if cfg!(target_os = "windows") {
			println!("This command doesn't support windows, sorry!");
		} else {
			//Reruns the program with sudo once
			if args[args.len() - 1] == "" {
				args.remove(args.len() - 1);
			} else {
				Command::new("sudo").args(&args).arg("").status().unwrap_or_else(|err| {
					eprintln!("Application Error: {}", err);
					process::exit(1);
				});
				process::exit(0);
			}
			args.remove(1);
			for i in 0 .. args.len() - 1 {
				if ((UniCase::new(&args[i + 1]) == UniCase::new("--file") && args.len() >= i + 7) || (UniCase::new(&args[i + 1]) == UniCase::new("-f") && args.len() >= i + 7)) && ((UniCase::new(&args[i + 3]) == UniCase::new("--advanced") && args.len() >= i + 7) || (UniCase::new(&args[i + 3]) == UniCase::new("-a") && args.len() >= i + 7)) {
					//If the repo url contains "http://" then replace it with https://
					if args[i + 4].contains("http://") && !args[i + 4].contains("https://") {
						args[i + 4] = args[i + 4].replace("http://", "https://");
						//If the repo url doesnt contain anything, then replace it with https://, an example "hello" the output would be "https://hello"
					} else if !args[i + 4].contains("http://") && !args[i + 4].contains("https://") && !args[i + 4].contains("fpt://") && !args[i + 4].contains("sfpt://") {
						args[i + 4] = "https://".to_owned() + &args[i + 4];
						//Replaces fpt:// with sfpt://
					} else if args[i + 4].contains("fpt://") && !args[i + 4].contains("sfpt://") {
						args[i + 4] = args[i + 4].replace("fpt://", "sfpt://");
					}
					//Makes sure there's a / at the end of the link to prevent any issues, have to have an empty if statement here because ! can't be used when checking characters.
					if args[i + 4].chars().last().unwrap() == '/' {} else {
						args[i + 4] = args[i + 4].to_owned() + "/"
					}

					let file_path = file_path_return(args[i + 2].to_owned(), true);
					if file_path == ". ." {
						args.remove(6);
						args.remove(5);
						args.remove(4);
						args.remove(3);
						args.remove(2);
						continue;
					}
					//Open source list, append so everything to a new line
					let mut source_list = OpenOptions::new()
						.write(true)
						.append(true)
						//Source list is located here.
						.open(file_path)
						.unwrap_or_else(|err| {
							eprintln!("Application Error: {}", err);
							process::exit(1)
						});
					//Basic checks if it was or wasn't able to write to the file.
					if let Err(e) = writeln!(source_list, "{}", "deb ".to_owned() + &args[i + 4] + " " + &args[i + 5] + " " + &args[i + 6]) {
						eprintln!("Couldn't write to file: {}", e);
					};
					args.remove(6);
					args.remove(5);
					args.remove(4);
					args.remove(3);
					args.remove(2);
				} else if (UniCase::new(&args[i + 1]) == UniCase::new("--file") && args.len() >= i + 4) || (UniCase::new(&args[i + 1]) == UniCase::new("-f") && args.len() >= i + 4) {
					//If the repo url contains "http://" then replace it with https://
					if args[i + 3].contains("http://") && !args[i + 3].contains("https://") {
						args[i + 3] = args[i + 3].replace("http://", "https://");
						//If the repo url doesnt contain anything, then replace it with https://, an example "hello" the output would be "https://hello"
					} else if !args[i + 3].contains("http://") && !args[i + 3].contains("https://") && !args[i + 3].contains("fpt://") && !args[i + 3].contains("sfpt://") {
						args[i + 3] = "https://".to_owned() + &args[i + 3];
						//Replaces fpt:// with sfpt://
					} else if args[i + 3].contains("fpt://") && !args[i + 3].contains("sfpt://") {
						args[i + 3] = args[i + 3].replace("fpt://", "sfpt://");
					}
					//Makes sure there's a / at the end of the link to prevent any issues, have to have an empty if statement here because ! can't be used when checking characters.
					if args[i + 3].chars().last().unwrap() == '/' {} else {
						args[i + 3] = args[i + 3].to_owned() + "/"
					}

					let file_path = file_path_return(args[i + 2].to_owned(), true);
					if file_path == ". ." {
						args.remove(3);
						args.remove(2);
						continue;
					}
					//Open source list, append so everything to a new line, this is why OpenOptions::new is used
					let mut source_list = OpenOptions::new()
						.write(true)
						.append(true)
						//Source list is located here.
						.open(file_path)
						.unwrap_or_else(|err| {
							eprintln!("Application Error: {}", err);
							process::exit(1)
						});

					//Basic checks if it was or wasn't able to write to the file.
					if let Err(e) = writeln!(source_list, "{}", "deb ".to_owned() + &args[i + 3] + " ./") {
						eprintln!("Couldn't write to file: {}", e);
					};
					args.remove(3);
					args.remove(2);
				} else if (UniCase::new(&args[i + 1]) == UniCase::new("--advanced") && args.len() >= i + 5) || (UniCase::new(&args[i + 1]) == UniCase::new("-a") && args.len() >= i + 5) {
					//If the repo url contains "http://" then replace it with https://
					if args[i + 2].contains("http://") && !args[i + 2].contains("https://") {
						args[i + 2] = args[i + 2].replace("http://", "https://");
						//If the repo url doesnt contain anything, then replace it with https://, an example "hello" the output would be "https://hello"
					} else if !args[i + 2].contains("http://") && !args[i + 2].contains("https://") && !args[i + 2].contains("fpt://") && !args[i + 2].contains("sfpt://") {
						args[i + 2] = "https://".to_owned() + &args[i + 2];
						//Replaces fpt:// with sfpt://
					} else if args[i + 2].contains("fpt://") && !args[i + 2].contains("sfpt://") {
						args[i + 2] = args[i + 2].replace("fpt://", "sfpt://");
					}

					//Makes sure there's a / at the end of the link to prevent any issues, have to have an empty if statement here because ! can't be used when checking characters.
					if args[i + 2].chars().last().unwrap() == '/' {} else {
						args[i + 2] = args[i + 2].to_owned() + "/"
					}

					let file_path = file_path_return(String::from("sources.list"), false);
					if file_path == ". ." {
						args.remove(4);
						args.remove(3);
						args.remove(2);
						continue;
					}
					//Open source list, append so everything is a new line
					let mut source_list = OpenOptions::new()
						.write(true)
						.append(true)
						//Source list is located here.
						.open(file_path)
						.unwrap_or_else(|err| {
							eprintln!("Application Error: {}", err);
							process::exit(1)
						});

					//Basic checks if it was or wasn't able to write to the file.
					if let Err(e) = writeln!(source_list, "{}", "deb ".to_owned() + &args[i + 2] + " " + &args[i + 3] + " " + &args[i + 4]) {
						eprintln!("Couldn't write to file: {}", e);
					};
					args.remove(4);
					args.remove(3);
					args.remove(2);
				} else {
					//If the repo url contains "http://" then replace it with https://
					if args[i + 1].contains("http://") && !args[i + 1].contains("https://") {
						args[i + 1] = args[i + 1].replace("http://", "https://");
						//If the repo url doesnt contain anything, then replace it with https://, an example "hello" the output would be "https://hello"
					} else if !args[i + 1].contains("http://") && !args[i + 1].contains("https://") && !args[i + 1].contains("fpt://") && !args[i + 1].contains("sfpt://") {
						args[i + 1] = "https://".to_owned() + &args[i + 1];
						//Replaces fpt:// with sfpt://
					} else if args[i + 1].contains("fpt://") && !args[i + 1].contains("sfpt://") {
						args[i + 1] = args[i + 1].replace("fpt://", "sfpt://");
					}

					//Makes sure there's a / at the end of the link to prevent any issues, have to have an empty if statement here because ! can't be used when checking characters.
					if args[i + 1].chars().last().unwrap() == '/' {} else {
						args[i + 1] = args[i + 1].to_owned() + "/"
					}

					let file_path = file_path_return(String::from("sources.list"), false);
					if file_path == ". ." {
						continue;
					}
					//Open source list, append so everything is a new line
					let mut source_list = OpenOptions::new()
						.write(true)
						.append(true)
						//Source list is located here.
						.open(file_path)
						.unwrap_or_else(|err| {
							eprintln!("Application Error: {}", err);
							process::exit(1);
						});

					//Basic checks if it was or wasn't able to write to the file.
					if let Err(e) = writeln!(source_list, "{}", "deb ".to_owned() + &args[i + 1] + " ./") {
						eprintln!("Couldn't write to file: {}", e);
					};
				}
				if i + 1 >= args.len() - 1 {
					break
				}
			}
			//Refreshes repositories
			Command::new("sudo").arg("apt").arg("update").status().unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1)
			});
		}
		process::exit(0);
	} else if UniCase::new(&args[1]) == UniCase::new("list") {
		//Runs apt list
		if cfg!(target_os = "windows") {
			Command::new("wsl").arg("--exec").arg("apt").arg("list").args(&args[2..]);
		} else {
			Command::new("apt").arg("list").args(&args[2..]);
		}
		process::exit(0);
	} else if UniCase::new(&args[1]) == UniCase::new("help") {
		if UniCase::new(&args[2]) == UniCase::new("search") {
			//NAME
			println!("\n[1mNAME[0m");
			//stn search
			println!("\t[1;35mstn [0m[1;33msearch[0m\n");
			//USAGE
			println!("[1mUSAGE[0m");
			//stn search <query>
			println!("\t[1;35mstn [0m[1;33msearch <query>[0m");
			//stn search <queries>
			println!("\t[1;35mstn [0m[1;33msearch <queries>[0m\n");
			//DESCRIPTION
			println!("[1mDESCRIPTION[0m");
			//The stn search command is used to search for the specified package(s)
			println!("\t[37mThe [0m[1;35mstn [0m[1;33msearch [0m[37mcommand is used to search for the specified package(s)");
			//throughout your APT repositories. When using stn search you can input
			println!("\tthroughout your APT repositories. When using [0m[1;35mstn [0m[1;33msearch [0m[37myou can input");
			//one or more search parameters.
			println!("\tone or more search parameters.[0m\n");
			//EXAMPLES
			println!("[1mEXAMPLES[0m");
			//stn search apt
			println!("\t[1;35mstn [0m[1;33msearch apt[0m");
			//stn search ap t
			println!("\t[1;35mstn [0m[1;33msearch ap t[0m\n");
			process::exit(0);
		} else if UniCase::new(&args[2]) == UniCase::new("list") {
			//NAME
			println!("\n[1mNAME[0m");
			//stn list
			println!("\t[1;35mstn [0m[1;33mlist[0m\n");
			//USAGE
			println!("[1mUSAGE[0m");
			//stn list
			println!("\t[1;35mstn [0m[1;33mlist[0m");
			//stn list [--flag(s)] <package(s)>
			println!("\t[1;35mstn [0m[1;33mlist [--flag(s)] <package(s)>[0m");
			//stn list <package(s)> [--flag(s)]
			println!("\t[1;35mstn [0m[1;33mlist <package(s)> [--flag(s)][0m");
			//stn list <package(s)>
			println!("\t[1;35mstn [0m[1;33mlist <package(s)>[0m");
			//stn list [--flag(s)]
			println!("\t[1;35mstn [0m[1;33mlist [--flag(s)][0m\n");
			//FLAGS
			println!("[1mFLAGS[0m");
			//stn list --installed
			println!("\t[1;35mstn [0m[1;33mlist --installed[0m");
			//stn list -i
			println!("\t[1;35mstn [0m[1;33mlist -i[0m");
			//stn list --upgradable
			println!("\t[1;35mstn [0m[1;33mlist --upgradable[0m");
			//stn list -u
			println!("\t[1;35mstn [0m[1;33mlist -u[0m\n");
			//DESCRIPTION
			println!("[1mDESCRIPTION[0m");
			//The stn list command is used to make a list of packages that are in all
			println!("\t[37mThe [0m[1;35mstn [0m[1;33mlist [0m[37mcommand is used to make a list of packages that are in all");
			//of your repositories. It can be used with or without arguments (as seen
			println!("\tof your repositories. It can be used with or without arguments (as seen");
			//under USAGE). Based on what arguments are (or aren't) inputted your
			println!("\tunder [0m[1mUSAGE [0m[37m). Based on what arguments are (or aren't) inputted your");
			//package list may differ.
			println!("\tpackage list may differ.[0m\n");
			//EXAMPLES
			println!("[1mEXAMPLES[0m");
			//stn list
			println!("\t[1;35mstn [0m[1;33mlist[0m");
			//stn list --installed apt
			println!("\t[1;35mstn [0m[1;33mlist --installed apt[0m");
			//stn list apt -u
			println!("\t[1;35mstn [0m[1;33mlist apt -u[0m");
			//stn list apt dpkg
			println!("\t[1;35mstn [0m[1;33mlist apt dpkg[0m");
			//stn list -i
			println!("\t[1;35mstn [0m[1;33mlist -i[0m\n");
			process::exit(0);
		} else if UniCase::new(&args[2]) == UniCase::new("info") {
			//NAME
			println!("\n[1mNAME[0m");
			//stn info
			println!("\t[1;35mstn [0m[1;33minfo[0m\n");
			//USAGE
			println!("[1mUSAGE[0m");
			//stn info <package(s)>
			println!("\t[1;35mstn [0m[1;33minfo <package(s)>[0m\n");
			//DESCRIPTION
			println!("[1mDESCRIPTION[0m");
			//The stn info command is used to search for the specified package(s)
			println!("\t[37mThe [0m[1;35mstn [0m[1;33minfo [0m[37mcommand is used to search for the specified package(s)");
			println!("\tthroughout your APT repositories and display the information for each");
			//package. When using stn info you can input one or more search
			println!("\tpackage. When using [0m[1;35mstn [0m[1;33minfo [0m[37myou can input one or more search");
			//parameters.
			println!("\tparameters.[0m\n");
			//EXAMPLES
			println!("[1mEXAMPLES[0m");
			//stn info apt
			println!("\t[1;35mstn [0m[1;33minfo apt[0m");
			//stn info apt dpkg
			println!("\t[1;35mstn [0m[1;33minfo apt dpkg[0m\n");
			process::exit(0);
		} else if UniCase::new(&args[2]) == UniCase::new("install") {
			//NAME
			println!("\n[1mNAME[0m");
			//stn install
			println!("\t[1;35mstn [0m[1;33minstall[0m\n");
			//USAGE
			println!("[1mUSAGE[0m");
			//stn install <package(s)>
			println!("\t[1;35mstn [0m[1;33minstall <package(s)>[0m\n");
			//DESCRIPTION
			println!("[1mDESCRIPTION[0m");
			//The stn install command is used to install the package(s) you specified
			println!("\t[37mThe [0m[1;35mstn [0m[1;33minstall [0m[37mcommand is used to install the package(s) you specified");
			//from your list of repositories. When using stn install you can input one
			println!("\tfrom your list of repositories. When using [0m[1;35mstn [0m[1;33minstall [0m[37myou can input one");
			//or more packages to install.
			println!("\tor more packages to install.[0m\n");
			//EXAMPLES
			println!("[1mEXAMPLES[0m");
			//stn install saturncli
			println!("\t[1;35mstn [0m[1;33minstall saturncli[0m");
			//stn install zsh saturncli
			println!("\t[1;35mstn [0m[1;33minstall zsh saturncli[0m\n");
			process::exit(0);
		} else if UniCase::new(&args[2]) == UniCase::new("reinstall") {
			//NAME
			println!("\n[1mNAME[0m");
			//stn reinstall
			println!("\t[1;35mstn [0m[1;33mreinstall[0m\n");
			//USAGE
			println!("[1mUSAGE[0m");
			//stn reinstall <package(s)>
			println!("\t[1;35mstn [0m[1;33mreinstall <package(s)>[0m\n");
			//DESCRIPTION
			println!("[1mDESCRIPTION[0m");
			//The stn reinstall command is used to install the package(s) you
			println!("\t[37mThe [0m[1;35mstn [0m[1;33mreinstall [0m[37mcommand is used to reinstall the package(s) you");
			//specified from your list of repositories. When using stn reinstall you
			println!("\tspecified from your list of repositories. When using [0m[1;35mstn [0m[1;33mreinstall [0m[37myou");
			//can input one or more packages to reinstall.
			println!("\tcan input one or more packages to reinstall.[0m\n");
			//EXAMPLES
			println!("[1mEXAMPLES[0m");
			//stn reinstall saturncli
			println!("\t[1;35mstn [0m[1;33mreinstall saturncli[0m");
			//stn reinstall zsh saturncli
			println!("\t[1;35mstn [0m[1;33mreinstall zsh saturncli[0m\n");
			process::exit(0);
		} else if UniCase::new(&args[2]) == UniCase::new("remove") {
			//NAME
			println!("\n[1mNAME[0m");
			//stn remove
			println!("\t[1;35mstn [0m[1;33mremove[0m\n");
			//USAGE
			println!("[1mUSAGE[0m");
			//stn remove <package(s)>
			println!("\t[1;35mstn [0m[1;33mremove <package(s)>[0m\n");
			//DESCRIPTION
			println!("[1mDESCRIPTION[0m");
			//The stn remove command is used to remove the package(s) you have
			println!("\t[37mThe [0m[1;35mstn [0m[1;33mremove [0m[37mcommand is used to remove the package(s) you have");
			//installed. When using stn remove you can input one or more packages to
			println!("\tinstalled. When using [0m[1;35mstn [0m[1;33mremove [0m[37myou can input one or more packages to");
			//remove.
			println!("\tremove.[0m\n");
			//EXAMPLES
			println!("[1mEXAMPLES[0m");
			//stn remove python3
			println!("\t[1;35mstn [0m[1;33mremove python3[0m");
			//stn remove zsh python3
			println!("\t[1;35mstn [0m[1;33mremove zsh python3[0m\n");
			process::exit(0);
		} else if UniCase::new(&args[2]) == UniCase::new("add-key") {
			//NAME
			println!("\n[1mNAME[0m");
			//stn add-key
			println!("\t[1;35mstn [0m[1;33madd-key[0m\n");
			//USAGE
			println!("[1mUSAGE[0m");
			//stn add-key <filepath>
			println!("\t[1;35mstn [0m[1;33madd-key <filepath>[0m\n");
			//DESCRIPTION
			println!("[1mDESCRIPTION[0m");
			//The stn add-key command is used to add a GPG key to the list of trusted
			println!("\t[37mThe [0m[1;35mstn [0m[1;33madd-key [0m[37mcommand is used to add a GPG key to the list of trusted");
			//keys.
			println!("\tkeys.[0m\n");
			//EXAMPLES
			println!("[1mEXAMPLES[0m");
			//stn add-key /folder/folder/destination.gpg
			println!("\t[1;35mstn [0m[1;33madd-key /folder/folder/destination.gpg[0m");
			//stn add-key /folder/folder/destination.asc
			println!("\t[1;35mstn [0m[1;33madd-key /folder/folder/destination.asc[0m\n");
			process::exit(0);
		} else if UniCase::new(&args[2]) == UniCase::new("edit-sources") {
			//NAME
			println!("\n[1mNAME[0m");
			//stn edit-sources
			println!("\t[1;35mstn [0m[1;33medit-sources[0m\n");
			//USAGE
			println!("[1mUSAGE[0m");
			//stn edit-sources
			println!("\t[1;35mstn [0m[1;33medit-sources[0m\n");
			//DESCRIPTION
			println!("[1mDESCRIPTION[0m");
			//The stn edit-sources command is used to open the file containing your
			println!("\t[37mThe [0m[1;35mstn [0m[1;33medit-sources [0m[37mcommand is used to open the file containing your");
			//APT sources with a command-line text editor.
			println!("\tAPT sources with a command-line text editor.[0m\n");
			//EXAMPLES
			println!("[1mEXAMPLES[0m");
			//stn edit-sources
			println!("\t[1;35mstn [0m[1;33medit-sources[0m\n");
			process::exit(0);
		} else if UniCase::new(&args[2]) == UniCase::new("autoremove") {
			//NAME
			println!("\n[1mNAME[0m");
			//stn autoremove
			println!("\t[1;35mstn [0m[1;33mautoremove[0m\n");
			//USAGE
			println!("[1mUSAGE[0m");
			//stn autoremove
			println!("\t[1;35mstn [0m[1;33mautoremove[0m\n");
			//DESCRIPTION
			println!("[1mDESCRIPTION[0m");
			//The stn autoremove command is used to automatically remove unneeded
			println!("\t[37mThe [0m[1;35mstn [0m[1;33mautoremove [0m[37mcommand is used to automatically remove unneeded");
			//packages (orphans).
			println!("\tpackages (orphans).[0m\n");
			//EXAMPLES
			println!("[1mEXAMPLES[0m");
			//stn autoremove
			println!("\t[1;35mstn [0m[1;33mautoremove[0m\n");
			process::exit(0);
		} else if UniCase::new(&args[2]) == UniCase::new("update") {
			//NAME
			println!("\n[1mNAME[0m");
			//stn update
			println!("\t[1;35mstn [0m[1;33mupdate[0m\n");
			//USAGE
			println!("[1mUSAGE[0m");
			//stn update
			println!("\t[1;35mstn [0m[1;33mupdate[0m\n");
			//DESCRIPTION
			println!("[1mDESCRIPTION[0m");
			//The stn update command is used to check your repositories and create a
			println!("\t[37mThe [0m[1;35mstn [0m[1;33mupdate [0m[37mcommand is used to check your repositories and create a");
			//new package list based on any changes.
			println!("\tnew package list based on any changes.[0m\n");
			//EXAMPLES
			println!("[1mEXAMPLES[0m");
			//stn update
			println!("\t[1;35mstn [0m[1;33mupdate[0m\n");
			process::exit(0);
		} else if UniCase::new(&args[2]) == UniCase::new("upgrade") {
			//NAME
			println!("\n[1mNAME[0m");
			//stn upgrade
			println!("\t[1;35mstn [0m[1;33mupgrade[0m\n");
			//USAGE
			println!("[1mUSAGE[0m");
			//stn upgrade
			println!("\t[1;35mstn [0m[1;33mupgrade[0m");
			//stn upgrade <package(s)>
			println!("\t[1;35mstn [0m[1;33mupgrade <package(s)>[0m\n");
			//DESCRIPTION
			println!("[1mDESCRIPTION[0m");
			//The stn upgrade command is used to upgrade the package(s) you specified
			println!("\t[37mThe [0m[1;35mstn [0m[1;33mupgrade [0m[37mcommand is used to upgrade the package(s) you specified");
			println!("\t(if no packages were specified then it will upgrade everything that can");
			//be upgraded) from your list of repositories. When using stn upgrade you
			println!("\tbe upgraded) from your list of repositories. When using [0m[1;35mstn [0m[1;33mupgrade [0m[37myou");
			//can input zero or more packages to upgrade.
			println!("\tcan input zero or more packages to upgrade.[0m\n");
			//EXAMPLES
			println!("[1mEXAMPLES[0m");
			//stn upgrade
			println!("\t[1;35mstn [0m[1;33mupgrade[0m");
			//stn upgrade saturncli
			println!("\t[1;35mstn [0m[1;33mupgrade saturncli[0m");
			//stn upgrade zsh saturncli
			println!("\t[1;35mstn [0m[1;33mupgrade zsh saturncli[0m\n");
			process::exit(0);
		} else if (UniCase::new(&args[2]) == UniCase::new("full-upgrade") && cfg!(target_os = "linux")) || (UniCase::new(&args[2]) == UniCase::new("full-upgrade") && cfg!(target_os = "windows")) {
			//NAME
			println!("\n[1mNAME[0m");
			//stn full-upgrade
			println!("\t[1;35mstn [0m[1;33mfull-upgrade[0m\n");
			//USAGE
			println!("[1mUSAGE[0m");
			//stn full-upgrade
			println!("\t[1;35mstn [0m[1;33mfull-upgrade[0m");
			//stn full-upgrade <package(s)>
			println!("\t[1;35mstn [0m[1;33mfull-upgrade <package(s)>[0m\n");
			//DESCRIPTION
			println!("[1mDESCRIPTION[0m");
			//The stn full-upgrade command is used to upgrade the package(s) you
			println!("\t[37mThe [0m[1;35mstn [0m[1;33mfull-upgrade [0m[37mcommand is used to upgrade the package(s) you");
			println!("\tspecified (if no packages were specified then it will upgrade your linux");
			println!("\tdistrobution) from your list of repositories. When using");
			//stn full-upgrade you can input zero or more packages to upgrade.
			println!("\t[0m[1;35mstn [0m[1;33mfull-upgrade [0m[37myou can input zero or more packages to upgrade.[0m");
			//EXAMPLES
			println!("[1mEXAMPLES[0m");
			//stn full-upgrade
			println!("\t[1;35mstn [0m[1;33mfull-pgrade[0m");
			//stn full-upgrade saturncli
			println!("\t[1;35mstn [0m[1;33mfull-upgrade saturncli[0m");
			//stn full-upgrade zsh saturncli
			println!("\t[1;35mstn [0m[1;33mfull-upgrade zsh saturncli[0m\n");
			process::exit(0);
		} else if UniCase::new(&args[2]) == UniCase::new("version") {
			//NAME
			println!("\n[1mNAME[0m");
			//stn version
			println!("\t[1;35mstn [0m[1;33mversion[0m\n");
			//USAGE
			println!("[1mUSAGE[0m");
			//stn version
			println!("\t[1;35mstn [0m[1;33mversion[0m\n");
			//DESCRIPTION
			println!("[1mDESCRIPTION[0m");
			//The stn version command is used to display the current version of APT,
			println!("\t[37mThe [0m[1;35mstn [0m[1;33mversion [0m[37mcommand is used to display the current version of APT,");
			//DPKG, and SaturnCLI you're running.
			println!("\tDPKG, and SaturnCLI you're running.[0m\n");
			//EXAMPLES
			println!("[1mEXAMPLES[0m");
			//stn version
			println!("\t[1;35mstn [0m[1;33mversion[0m\n");
			process::exit(0);
		} else if UniCase::new(&args[2]) == UniCase::new("clean") {
			//NAME
			println!("\n[1mNAME[0m");
			//stn clean
			println!("\t[1;35mstn [0m[1;33mclean[0m\n");
			//USAGE
			println!("[1mUSAGE[0m");
			//stn clean
			println!("\t[1;35mstn [0m[1;33mclean[0m\n");
			//DESCRIPTION
			println!("[1mDESCRIPTION[0m");
			//The stn clean command is used to clean the download cache.
			println!("\t[37mThe [0m[1;35mstn [0m[1;33mclean [0m[37mcommand is used to clean the download cache.[0m\n");
			//EXAMPLES
			println!("[1mEXAMPLES[0m");
			//stn version
			println!("\t[1;35mstn [0m[1;33mclean[0m\n");
			process::exit(0);
		} else if UniCase::new(&args[2]) == UniCase::new("help") {
			//NAME
			println!("\n[1mNAME[0m");
			//stn help
			println!("\t[1;35mstn [0m[1;33mhelp[0m\n");
			//USAGE
			println!("[1mUSAGE[0m");
			//stn help
			println!("\t[1;35mstn [0m[1;33mhelp[0m");
			//stn help <command>
			println!("\t[1;35mstn [0m[1;33mhelp <command>[0m\n");
			//DESCRIPTION
			println!("[1mDESCRIPTION[0m");
			//The stn help command is used to display the help menu for you. When
			println!("\t[37mThe [0m[1;35mstn [0m[1;33mhelp [0m[37mcommand is used to display the help menu for you. When");
			//using stn help <command> you can input zero arguments or one argument.
			println!("\tusing [0m[1;35mstn [0m[1;33mhelp <command> [0m[37myou can input zero arguments or one argument.[0m\n");
			//EXAMPLES
			println!("[1mEXAMPLES[0m");
			//stn help
			println!("\t[1;35mstn [0m[1;33mhelp[0m");
			//stn help list
			println!("\t[1;35mstn [0m[1;33mhelp list[0m\n");
			process::exit(0);
		} else if UniCase::new(&args[2]) == UniCase::new("about") {
			//NAME
			println!("\n[1mNAME[0m");
			//stn about
			println!("\t[1;35mstn [0m[1;33mabout[0m\n");
			//USAGE
			println!("[1mUSAGE[0m");
			//stn about
			println!("\t[1;35mstn [0m[1;33mabout[0m\n");
			//DESCRIPTION
			println!("[1mDESCRIPTION[0m");
			//The stn about command is used to view legal information and credits
			println!("\t[37mThe [0m[1;35mstn [0m[1;33mabout [0m[37mcommand is used to view legal information and credits");
			//regarding SaturnCLI.
			println!("\tregarding SaturnCLI.[0m\n");
			//EXAMPLES
			println!("[1mEXAMPLES[0m");
			//stn about
			println!("\t[1;35mstn [0m[1;33mabout[0m\n");
			process::exit(0);
		} else if UniCase::new(&args[2]) == UniCase::new("add-repositories") || UniCase::new(&args[2]) == UniCase::new("add-repos"){
			//NAME
			println!("\n[1mNAME[0m");
			//stn add-repositories
			println!("\t[1;35mstn [0m[1;33madd-repositories[0m\n");
			//USAGE
			println!("[1mUSAGE[0m");
			//stn add-repositories <repo(s)>
			println!("\t[1;35mstn [0m[1;33madd-repositories <repo(s)>[0m");
			//stn add-repositories [-a] <repo> <codename> <component>
			println!("\t[1;35mstn [0m[1;33madd-repositories [-a] <repo> <codename> <component>[0m");
			//stn add-repositories [-f] <filename> <repo>
			println!("\t[1;35mstn [0m[1;33madd-repositories [-f] <filename> <repo>[0m");
			//stn add-repositories [-f] <filename> [-a] <repo> <codename> <component>
			println!("\t[1;35mstn [0m[1;33madd-repositories [-f] <filename> [-a] <repo> <codename> <component>[0m");
			//stn add-repos <repo(s)>
			println!("\t[1;35mstn [0m[1;33madd-repos <repo(s)>[0m");
			//stn add-repos [-a] <repo> <codename> <component>
			println!("\t[1;35mstn [0m[1;33madd-repos [-a] <repo> <codename> <component>[0m");
			//stn add-repos [-f] <filename> <repo>
			println!("\t[1;35mstn [0m[1;33madd-repos [-f] <filename> <repo>[0m");
			//stn add-repos [-f] <filename> [-a] <repo> <codename> <component>
			println!("\t[1;35mstn [0m[1;33madd-repos [-f] <filename> [-a] <repo> <codename> <component>[0m\n");
			//USAGE
			println!("[1mUSAGE[0m");
			//stn add-repositories --advanced
			println!("\t[1;35mstn [0m[1;33madd-repositories --advanced[0m");
			//stn add-repositories -a
			println!("\t[1;35mstn [0m[1;33madd-repositories -a[0m");
			//stn add-repositories --file
			println!("\t[1;35mstn [0m[1;33madd-repositories --file[0m");
			//stn add-repositories -f
			println!("\t[1;35mstn [0m[1;33madd-repositories -f[0m\n");
			//DESCRIPTION
			println!("[1mDESCRIPTION[0m");
			//The stn add-repositories command is used to add new apt sources.
			println!("\t[37mThe [0m[1;35mstn [0m[1;33madd-repositories [0m[37mcommand is used to add new apt repositories.");
			//EXAMPLES
			println!("[1mEXAMPLES[0m");
			//stn add-repositories google.com
			println!("\t[1;35mstn [0m[1;33madd-repositories google.com[0m");
			//stn add-repositories -f example google.com
			println!("\t[1;35mstn [0m[1;33madd-repositories -f example google.com[0m");
			//stn add-repositories --file example google.com
			println!("\t[1;35mstn [0m[1;33madd-repositories --file example google.com[0m");
			//stn add-repositories -f example -a google.com catalina main
			println!("\t[1;35mstn [0m[1;33madd-repositories -f example -a google.com catalina main[0m");
			//stn add-repositories --files example -a google.com catalina main
			println!("\t[1;35mstn [0m[1;33madd-repositories --files example -a google.com catalina main[0m");
			//stn add-repositories google.com smushytaco.com
			println!("\t[1;35mstn [0m[1;33madd-repositories google.com smushytaco.com[0m");
			//stn add-repositories -a google.com catalina main
			println!("\t[1;35mstn [0m[1;33madd-repositories -a google.com catalina main[0m");
			//stn add-repositories --advanced google.com catalina main
			println!("\t[1;35mstn [0m[1;33madd-repositories --advanced google.com catalina main[0m");
			//stn add-repositories -a google.com catalina main smushytaco.com
			println!("\t[1;35mstn [0m[1;33madd-repositories -a google.com catalina main smushytaco.com[0m");
			//stn add-repositories --advanced google.com catalina main smushytaco.com
			println!("\t[1;35mstn [0m[1;33madd-repositories --advanced google.com catalina main smushytaco.com[0m\n");
			process::exit(0);
		} else {
			help_menu();
		}
	} else {
		//This is here so if a one argument command was typed with two or more arguments it'll be checked here and executed as should.
		if_checks_for_things_with_one_argument();
	}
}
