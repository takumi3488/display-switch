default: target/release/display-switch /opt/homebrew/bin/displayplacer

target/release/display-switch: \
	Cargo.toml Cargo.lock \
	src/main.rs src/placer.rs src/store.rs
	cargo build --release

/opt/homebrew/bin/displayplacer: /opt/homebrew/bin/brew
	brew install displayplacer

/opt/homebrew/bin/brew:
	echo "Homebrew is not installed. Please install Homebrew first." >&2

install: /usr/local/bin/display-switch /usr/local/bin/dsw

/usr/local/bin/display-switch: target/release/display-switch
	cp target/release/display-switch /usr/local/bin/display-switch

/usr/local/bin/dsw: /usr/local/bin/display-switch
	ln -s /usr/local/bin/display-switch /usr/local/bin/dsw

clean:
	cargo clean
	rm -f /usr/local/bin/display-switch /usr/local/bin/dsw
