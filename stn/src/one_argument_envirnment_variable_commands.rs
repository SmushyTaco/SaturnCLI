extern crate unicase;
use std::process::Command;
use unicase::UniCase;
use std::process;
use std::env;
pub fn help_menu() {
	//Prints help menu.
		//SaturnCLI Help
		println!("\n[1mSaturnCLI Help[0m\n");
		//stn search <query>
		println!("[1;35mstn [0m[1;33msearch <query>[0m\t\t\tSearches for specified query");
		//stn list [--flag(s)] <package(s)>
		println!("[1;35mstn [0m[1;33mlist [--flag(s)] <package(s)>[0m\tLists specified packages");
		//stn info <package(s)>
		println!("[1;35mstn [0m[1;33minfo <package(s)>[0m\t\t\tDisplay info on specified package(s)");
		//stn install <package(s)>
		println!("[1;35mstn [0m[1;33minstall <package(s)>[0m\t\tInstalls specified package(s)");
		//stn reinstall <package(s)>
		println!("[1;35mstn [0m[1;33mreinstall <package(s)>[0m\t\tReinstalls specified package(s)");
		//stn remove <package(s)>
		println!("[1;35mstn [0m[1;33mremove <package(s)>[0m\t\t\tRemoves specified package(s)");
		//stn add-key <filepath>
		println!("[1;35mstn [0m[1;33madd-key <filepath>[0m\t\t\tAdds a key to the list of trusted keys");
		//stn edit-sources
		println!("[1;35mstn [0m[1;33medit-sources[0m\t\t\tOpens the APT repo editor");
		//stn add-repositories [--flag(s)] <repo(s)>
		println!("[1;35mstn [0m[1;33madd-repos [--flag(s)] <repo(s)>[0m\tAdds new repo(s)");
		//stn autoremove
		println!("[1;35mstn [0m[1;33mautoremove[0m\t\t\t\tRemoves unneeded packages (orphans)");
		//stn update
		println!("[1;35mstn [0m[1;33mupdate[0m\t\t\t\tUpdates the repository lists");
		//stn upgrade <package(s)>
		println!("[1;35mstn [0m[1;33mupgrade <package(s)>[0m\t\tUpgrades specified packages");
		if cfg!(target_os = "windows") || cfg!(target_os = "linux") {
			//stn full-upgrade <package(s)>
			println!("[1;35mstn [0m[1;33mfull-upgrade <package(s)>[0m\t\tUpgrades the system (or package(s))");
		}
		//stn version
		println!("[1;35mstn [0m[1;33mversion[0m\t\t\t\tShow APT, DPKG, and SaturnCLI versions");
		//stn clean
		println!("[1;35mstn [0m[1;33mclean[0m\t\t\t\tClears the download cache");
		//stn help <command>
		println!("[1;35mstn [0m[1;33mhelp <command>[0m\t\t\tOpens help menu for specified commands");
		//stn about
		println!("[1;35mstn [0m[1;33mabout[0m\t\t\t\tView legal information and credits\n");
		process::exit(0);
}
pub fn if_checks_for_things_with_one_argument() {
	let args: Vec<String> = env::args().collect();
	if UniCase::new(&args[1]) == UniCase::new("list") {
		//Runs apt list
		if cfg!(target_os = "windows") {
			Command::new("wsl").arg("--exec").arg("apt").arg("list").status().unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1);
			});
		} else {
			Command::new("apt").arg("list").status().unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1);
			});
		}
		process::exit(0);
	} else if UniCase::new(&args[1]) == UniCase::new("autoremove") {
		//Runs sudo apt purge --autoremove
		if cfg!(target_os = "windows") {
			Command::new("wsl").arg("--exec").arg("sudo").arg("apt").arg("purge").arg("--autoremove").status().unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1);
			});
		} else {
			Command::new("sudo").arg("apt").arg("purge").arg("--autoremove").status().unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1);
			});
		}
		process::exit(0);
	} else if UniCase::new(&args[1]) == UniCase::new("update") {
		//Runs sudo apt + first argument user inputted.
		if cfg!(target_os = "windows") {
			Command::new("wsl").arg("--exec").arg("sudo").arg("apt").arg("update").status().unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1);
			});
		} else {
			Command::new("sudo").arg("apt").arg("update").status().unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1);
			});
		}
		process::exit(0);
	} else if UniCase::new(&args[1]) == UniCase::new("edit-sources") {
		//Runs sudo apt edit-sources
		if cfg!(target_os = "windows") {
			Command::new("wsl").arg("--exec").arg("sudo").arg("apt").arg("edit-sources").status().unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1);
			});
		} else {
			Command::new("sudo").arg("apt").arg("edit-sources").status().unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1);
			});
		}
		process::exit(0);
	} else if UniCase::new(&args[1]) == UniCase::new("upgrade") {
		//Runs sudo apt update then sudo apt upgrade
		if cfg!(target_os = "windows") {
			Command::new("wsl").arg("--exec").arg("sudo").arg("apt").arg("update").status().unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1);
			});
			Command::new("wsl").arg("--exec").arg("sudo").arg("apt").arg("upgrade").status().unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1);
			});
		} else {
			Command::new("sudo").arg("apt").arg("update").status().unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1);
			});
			Command::new("sudo").arg("apt").arg("upgrade").status().unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1);
			});
		}
		process::exit(0);
	} else if UniCase::new(&args[1]) == UniCase::new("full-upgrade") {
		if cfg!(target_os = "linux") {
			//Runs sudo apt update
			Command::new("sudo").arg("apt").arg("update").status().unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1);
			});
			//Runs sudo apt upgrade
			Command::new("sudo").arg("apt").arg("full-upgrade").status().unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1);
			});
			process::exit(0);
		} else if cfg!(target_os = "windows") {
			//Runs sudo apt update
			Command::new("wsl").arg("--exec").arg("sudo").arg("apt").arg("update").status().unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1);
			});
			//Runs sudo apt upgrade
			Command::new("wsl").arg("--exec").arg("sudo").arg("apt").arg("full-upgrade").status().unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1);
			});
			process::exit(0);
		} else {
			//Displays this message if the command the user entered doesn't exist.
			eprintln!("Unknown command {:?}, type \"stn help\" to see the list of available commands.", args[1].to_lowercase());
			process::exit(1);
		}
	} else if UniCase::new(&args[1]) == UniCase::new("clean") {
		if cfg!(target_os = "windows") {
			//Runs sudo apt autoclean
			Command::new("wsl").arg("--exec").arg("sudo").arg("apt").arg("autoclean").status().unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1);
			});
			//Runs sudo apt clean
			Command::new("wsl").arg("--exec").arg("sudo").arg("apt").arg("clean").status().unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1);
			});
		} else {
			//Runs sudo apt autoclean
			Command::new("sudo").arg("apt").arg("autoclean").status().unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1);
			});
			//Runs sudo apt clean
			Command::new("sudo").arg("apt").arg("clean").status().unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1);
			});
		}
		process::exit(0);
	} else if UniCase::new(&args[1]) == UniCase::new("help") {
		help_menu();
	} else if UniCase::new(&args[1]) == UniCase::new("about") {
		//Prints about message.
		println!("About SaturnCLI");
		println!("Copyright (C) 2019 SmushyTaco Development.");
		println!("https://www.smushytaco.com/\n");
		println!("Copyright 2019 SmushyTaco Development");
		println!("Licensed under the GNU Lesser General Public License v3.0, (the \"License\");");
		println!("you may not use this file except in compliance with the License.");
		println!("You may obtain a copy of the License at\n");
		println!("https://www.gnu.org/licenses/lgpl-3.0.en.html");
		println!("Unless required by applicable law or agreed to in writing, software");
		println!("distributed under the License is distributed on an \"AS IS\" BASIS,");
		println!("WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.");
		println!("See the License for the specific language governing permissions and");
		println!("limitations under the License.\n");
		println!("SaturnCLI is made possible by the following software and people:\n");
		println!("Nikan Radan (SmushyTaco) for writing it");
		println!("Debian for creating APT: https://packages.debian.org/stretch/apt");
		println!("Canonical for maintaining the latest versions of APT: https://launchpad.net/ubuntu/+source/apt/");
		process::exit(0);
	} else if UniCase::new(&args[1]) == UniCase::new("version") {
		//Displays the current version of SaturnCLI the user is running.
		println!("SaturnCLI 1.0.0\n");
		if cfg!(target_os = "windows") {
			//Displays the version of apt the user is running.
			Command::new("wsl").arg("--exec").arg("apt").arg("--version").status().unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1);
			});
			println!("");
			//Displays the version of apt the user is running.
			Command::new("wsl").arg("--exec").arg("dpkg").arg("--version").status().unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1);
			});
		} else {
			//Displays the version of apt the user is running.
			Command::new("apt").arg("--version").status().unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1);
			});
			println!("");
			//Displays the version of apt the user is running.
			Command::new("dpkg").arg("--version").status().unwrap_or_else(|err| {
				eprintln!("Application Error: {}", err);
				process::exit(1);
			});
		}
		process::exit(0);
	} else if UniCase::new(&args[1]) == UniCase::new("install") || UniCase::new(&args[1]) == UniCase::new("reinstall") || UniCase::new(&args[1]) == UniCase::new("remove") || UniCase::new(&args[1]) == UniCase::new("info") || UniCase::new(&args[1]) == UniCase::new("search") || UniCase::new(&args[1]) == UniCase::new("add-repositories") || UniCase::new(&args[1]) == UniCase::new("add-repos") {
		//Gives error message if only 1 argument was inputted for something that requires 2 arguments.
		eprintln!("{:?} needs at least two arguments but you only entered one, run \"stn help {}\" to learn how to properly use {:?}.", args[1].to_lowercase(), args[1].to_lowercase(), args[1].to_lowercase());
		process::exit(1);
	} else {
		//Displays this message if the command the user entered doesn't exist.
		eprintln!("Unknown command {:?}, type \"stn help\" to see the list of available commands.", args[1].to_lowercase());
		process::exit(1);
	}
}
