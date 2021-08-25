EXE_PATH = /usr/local/bin
SERVICE_PATH = /etc/systemd/system

all:
	cargo build --release

install:
	sudo cp target/release/g910-macros $(EXE_PATH) && \
	sudo cp g910-macros.service $(SERVICE_PATH)

uninstall:
	sudo rm $(EXE_PATH)/g910-macros && \
	sudo rm $(SERVICE_PATH)/g910-macros.service

.PHONY: all install uninstall
