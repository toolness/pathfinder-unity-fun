This is an experiment in building a [native Unity plugin][] for the Canvas API
portion of [Pathfinder][].

For more details on the context behind this, see 
[Pathfinder issue #147](https://github.com/pcwalton/pathfinder/issues/147).

Right now all we have is a plugin that draws a house on top of whatever
Unity has already drawn on every frame.

## Quick start

The project currently only works on Windows, and it only supports
Unity projects that use the OpenGL backend. It has been tested
with Unity 2019.1.

```
git submodule init
git submodule update
build_plugin
```

You will need to open the Unity project in the `unity-project` folder;
specifically, the scene `unity-project\Assets\Scenes\SampleScene.unity`.

To iterate on development and get decent debugger support, you can
do the following:

1. Build the Unity project in the `dist` directory. Then exit Unity so it
   doesn't have a lock on the plugin's DLL (either that or exclude the
   plugin from running in the editor, and restart Unity).
2. Open the Visual Studio solution in `dist/VSDebugHarness`. This solution
   doesn't have any code, it's just set up to launch the Unity project
   and attach a debugger to it.
3. Make sure you are targeting `x64` in Visual Studio and press <kbd>F5</kbd>.

Note that pressing <kbd>F5</kbd> automatically re-runs `build_plugin`, so you
don't need to worry about re-running it manually.

Logging produced by the plugin will be available in `dist/pathfinder-plugin.log`.

You can press <kbd>Esc</kbd> to exit the demo.

[native Unity plugin]: https://docs.unity3d.com/Manual/NativePlugins.html
[Pathfinder]: https://github.com/pcwalton/pathfinder
