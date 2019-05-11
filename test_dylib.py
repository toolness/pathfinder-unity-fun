from pathlib import Path
from ctypes import *

MY_DIR = Path(__file__).parent.resolve()

BUILD_TYPE = "debug"

DLL_PATH = MY_DIR / "target" / BUILD_TYPE / "pathfinder_c_api_fun.dll"

DLL_STR = str(DLL_PATH)

cdll.LoadLibrary(DLL_STR)

fun = CDLL(DLL_STR)

assert fun.boop(2) == 642

print("Hooray, it works!")
