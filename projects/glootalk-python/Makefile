UNAME := $(shell uname)

build:
ifeq ($(UNAME),Darwin)
	cargo rustc --release -- -C link-arg=-undefined -C link-arg=dynamic_lookup
endif
hello:
	echo "hello world"
clean:
	rm -rf build dist glootalk.egg-info target
