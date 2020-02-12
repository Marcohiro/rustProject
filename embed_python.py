from ctypes import cdll
import os

lib_filename = "embed.dll";
lib_pathdirectory = os.path.dirname(os.path.abspath("target/release/"+lib_filename));
#print("filename: <<"+ lib_pathdirectory + "/" + lib_filename);

lib = cdll.LoadLibrary(lib_pathdirectory + "/" + lib_filename);
lib.process();

print("done!");