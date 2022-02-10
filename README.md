# fltk-smarttable-demo

This small rust app shall is a show case for a possible bug in the rust fltk-table crate.
In this example when you click on any call the callback always called twice as you can see in the log output.
On the first click it is called three times.

See a sample output here.

```
callback called!
got called by row no: -1
got called by col no: -1
callback called!
got called by row no: 1
got called by col no: 0
callback called!
got called by row no: 1
got called by col no: 0
callback called!
got called by row no: -1
got called by col no: -1
callback called!
got called by row no: 1
got called by col no: 1
callback called!
got called by row no: 1
got called by col no: 1
```

To try it out run

```
cargo run
```

The fltk-table crate is great and I can work around this issue easily but I'd like to contribute and help to improve this good work!
