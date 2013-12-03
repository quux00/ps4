arch=arm

all:
	$(MAKE) all -C arch/$(arch)/

%:
	$(MAKE) $* -C arch/$(arch)/
