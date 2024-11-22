# DankCaster

## Prerequisites

The `autovideosink` element is needed for this demo, it might not always be there by default so please ensure you have it first:

```bash
gst-inspect-1.0 autovideosink
```

If it's not showing up despite using a custom installation of gstreamer (`/usr/local`) which is supposed to come with it, then you need to extend the default system path list:

```bash
export GST_PLUGIN_SYSTEM_PATH_1_0=$GST_PLUGIN_SYSTEM_PATH_1_0:/usr/local/lib/gstreamer-1.0
```

Most importantly, you need `cargo-c` to build the gstreamer plugins:

```bash
cargo install cargo-c
```

## Run test pipeline

``` bash
cargo cbuild -p gst-plugin-dkc
export RUSTC_TARGET_TRIPLE=$(rustc -vV | grep host: | awk -F: '{ print $2 }' | sed 's/ //g')
export GST_PLUGIN_PATH=`pwd`/target/${RUSTC_TARGET_TRIPLE}/debug
gst-launch-1.0 dkcdummysource name="mysource" ! dkcscene name="myscene" ! dkcdummysink name="mysink"  mysource.audio_src ! myscene.audio_sink_0  myscene.audio_src_0 ! mysink.audio_sink
```
