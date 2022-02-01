TARGET_DIR=target/mos-a800xl-none/release/examples
DOS_ATR_TEMPLATE=examples/data/dos2d64.atr
EXAMPLE_NAME=$(basename $(notdir $@))

all: $(addprefix $(TARGET_DIR)/,2048.done cls.done console.done draw.done files.done)

clean:
	cargo clean
	rm -f examples.atr*

examples.atr.done:
	cp $(DOS_ATR_TEMPLATE) examples.atr
	touch $@

$(TARGET_DIR)/%.done: examples.atr.done examples/%.rs
	cargo build --example  $(EXAMPLE_NAME) --release
	atr examples.atr put $(basename $@) $(EXAMPLE_NAME).com
	touch $@
