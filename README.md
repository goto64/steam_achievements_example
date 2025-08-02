## Bevy Engine - Steam Achievements Example 

This is an example of how Steam achievements can be handled in a Bevy Engine game.


This example uses the Steam demo game Space War.\
Install Space War by putting this URL in a browser while the Steam desktop app is running:
```
steam://run/480
```

When building an executable, put "steam_api64.dll" AND "steam_api64.lib" in the same folder as the .exe
They are found in build folder: target\release\build\steamworks-sys-??????????\out

**When running this example, make sure that the Steam desktop app is running.**

In Steam console, reset a Space War achievement with for instance:
```
achievement_clear 480 ACH_WIN_ONE_GAME
```
To reset the stat that is used for one achievement, in Steam console:
```
stats_reset 480 FeetTraveled
```


