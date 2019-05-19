This is an experiment in building a native Unity plugin for the Canvas API
portion of [Pathfinder][].

For more details on the context behind this, see 
[Pathfinder issue #147](https://github.com/pcwalton/pathfinder/issues/147).

## Quick start

Right now all we have is an extremely simple plugin that doesn't do much.
It also currently only works on Windows.

```
git submodule init
git submodule update
build_plugin
```

You will need to open the Unity project in the `unity-project` folder.
To iterate on development and get decent debugger support, you can
do the following:

1. Build the Unity project in the `dist` directory.
2. Open the Visual Studio solution in `dist/VSDebugHarness`.
3. Make sure you are targeting `x64` in Visual Studio and press F5.

Logging produced by the plugin will be available in `dist/pathfinder-plugin.log`.

[Pathfinder]: https://github.com/pcwalton/pathfinder
