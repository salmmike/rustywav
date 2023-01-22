# Rustywav [WIP]

My first Rust project, a library that reads a wav file, maybe even plays it at some point.

# My thoughts while programming

## Initial thought about rust
This is my attempt at learning Rust, which seems to be what all the cool kids are talking about.
As a mostly C++ fan, my initial thought about Rust is that it makes simple things complicated to prevent memory leaks.
But this is before I have written anything more than a hello world in Rust and read a few articles about it.
My goal is to write a few projects in Rust, hopefully I'll learn to like it.

I think a wav reader will be a nice thing to try as a first project, because it will force me to deal with things I believe are difficult to do in Rust.
My main concern is the type safety in Rust. In C++ (and in C, but with slightly different function names), I should be able to just do this very simply, something like:
```cpp
typedef struct WavHeader {
...

std::ifstream wavheader;
# open in binary mode.
wavheader.open(path, ...);
wavheader.read(reinterpret_cast<char*>(header), sizeof(WavHeader));
#done
```
With Rust, I believe this would be more difficult.

## Thoughts during programming session

My first task was to divide my library into a lib and src direcories.
This was really easy, everything was handled by cargo almost automatically. I really think this is much faster than something like CMake (unless you have a script that does it for you).
I think Cargo definately is one of the good things about Rust. Nice!

As my second task, I wanted to read the Wav file contents into a Vec. This was pretty easy to do.

My third task of the day was to create a struct of the Wav file header.
At this point, things started to be a bit more complicated than in C. This is most likely because I have never used Rust before,
but my C++ experience isn't helping all that much. I wanted to create a struct beginning with the RIFF header, an array of 4 bytes.
So, naturally, my first idea is to use something like
```
struct wav_header {
riff: [u8, 4]
```
That didn't compile, and apparently I'll need to use a vector here.
It feels a bit annoying that I'd have to use an object that can change in size, even though I know it will never be of any different size.
Probably again just a problem that is caused by my lack of knownledge and understanding of Rust, but I am documenting the C++ developers experience using Rust for the first time,
without reading too much documentation beforehand.
I'm not sure if reading the data to a vector first is a good idea at all. Maybe there would have been a way to read everything right into the struct, just like in C.

My next task was to convert four bytes to an unsigned 32bit value.
This wasn't as difficult as I thought, but I'll need to test this with some real data and see that everything works the way I think it does.


After I was able to read four bytes as a 32 bit value, it was fairly simple to read the whole Wav file header.
The syntax I wrote on the other hand was completely awful.
There has to be a better way to do this, as many people seem to think Rust is awesome.
I think I'll try to make a version two of this at some point when I have learned more, and see how much more easier this task is.
(Well, knowing myself, it's pretty likely that that version 2 will never happen, but I'm sure I'll continue learning Rust anyways).

The Rust error handling might get a bit of getting used to, as the borrow and move system.
But, I think if I just use them enough, they'll be just fine.
This is the same with all the other syntax too. Currently,
I very much prefer the simple syntax of C and C++, but given that this is the first program I wrote in Rust, I guess that's expected. Maybe after 10 small programs, the syntax will be more familiar and nicer.
