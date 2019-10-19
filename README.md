# DankCaster

```
$ cd gst-plugin && cargo build && cd -
$ export GST_PLUGIN_PATH=`pwd`/target/debug
$ gst-launch-1.0 dkcdummysource name="mysource" ! dkcscene name="myscene" ! dkcdummysink name="mysink"  mysource.audio_src ! myscene.audio_sink_0  myscene.audio_src_0 ! mysink.audio_sink
```