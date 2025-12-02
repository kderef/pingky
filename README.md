# SDL3-cmake-template
simple template for SDL3 in C

### HOW TO USE
first, clone this repository:  
```
git clone https://github.com/kderef/SDL3-cmake-template
```
then edit the files inside the `src` folder,  
and then create and enter the build directory:  
```
mkdir build
cd build
```
then, run `cmake` to generate the build files.
```
cmake ..
```
*optionally, you can also add `-D CMAKE_BUILD_TYPE=Release` to enable optimizations and disable the console on windows.*  
finally, run `make` or open the solution if you are on windows!
