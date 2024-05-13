{ pkgs, lib, config, inputs, ... }:

{
# https://devenv.sh/basics/
  env = {
		GREET = "devenv";
		CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER = "${pkgs.llvmPackages.clangUseLLVM}/bin/clang";
		#CARGO_ENCODED_RUSTFLAGS = "-Clink-arg=-fuse-ld=${pkgs.mold}/bin/mold";
		LD_LIBRARY_PATH = "${pkgs.ncurses}/lib:$LD_LIBRARY_PATH";
	};
# https://devenv.sh/packages/
	packages = [ pkgs.git pkgs.ncurses];

# https://devenv.sh/scripts/
	scripts.hello.exec = "echo hello from $GREET";
  cachix.enable = false;
	enterShell = ''
		hello
		git --version
		'';

# https://devenv.sh/tests/
	enterTest = ''
		echo "Running tests"
		git --version | grep "2.42.0"
		'';

# https://devenv.sh/services/
# services.postgres.enable = true;

# https://devenv.sh/languages/
	languages.nix.enable = true;
	languages.rust.enable = true;

# https://devenv.sh/pre-commit-hooks/
	pre-commit.hooks = {
		commitizen.enable = true;
		clippy.enable = true;
	};
# https://devenv.sh/processes/
# processes.ping.exec = "ping example.com";

# See full reference at https://devenv.sh/reference/options/
}
