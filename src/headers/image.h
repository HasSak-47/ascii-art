#pragma once
#define DEBUG

#ifdef DEBUG
#include<iostream>
#endif
#define max_height 42
#define reset "\033[0m"

namespace sk{
    typedef unsigned char byte;
    
    enum class color {
        black, red, green, yellow, blue, purple, cyan, white
    };

    enum class img_t {
        png, bmp, tga, jpg, hdr
    };

    char* color_text(color foreground, color background = color::black);

    class image;
    void print_image(image& source);

    class image{
    public:
        int width, height, channels;
        byte* data;

        image(const char* path, int channels);
        image(const char* path);

        ~image();

        void resize(int width, int height);
        void resize(float mult);

        void save(const char* path);
        #ifdef DEBUG
        friend std::ostream& operator<<(std::ostream& os, const image& img);
        #endif
    };
}
