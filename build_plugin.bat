@echo off

mkdir dist\unity-project_Data\Plugins 2>nul

cargo build && ^
copy target\debug\pathfinder_c_api_fun.dll unity-project\Assets\GfxPluginPathfinder.dll && ^
copy target\debug\pathfinder_c_api_fun.dll dist\unity-project_Data\Plugins\GfxPluginPathfinder.dll && ^
xcopy /q /s /y pathfinder\resources unity-project\Assets\StreamingAssets\pathfinder\ && ^
xcopy /q /s /y pathfinder\resources dist\unity-project_Data\StreamingAssets\pathfinder\
