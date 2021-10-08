#include<stdexcept>
#include<iostream>
#include"../headers/image.h"

#define STB_IMAGE_IMPLEMENTATION
#include"../headers/stb_image.h"

#define STB_IMAGE_RESIZE_IMPLEMENTATION
#include"../headers/stb_image_resize.h"

#define STB_IMAGE_WRITE_IMPLEMENTATION
#include"../headers/stb_image_write.h"

namespace sk{
    char* color_text(color foreground, color background){
        static char format[] = "\033[3F;4Bm";
        format[3] = (char)foreground + '0';
        format[6] = (char)background + '0';
        return format;
    }


    image::image(const char* c_path){
        this->data = stbi_load(c_path, &this->width, &this->height, &this->channels, 0);
        if(!this->data) throw std::runtime_error("image not found!");
    }

    image::~image(){
        if(this->data) free(this->data);
    }

    void image::resize(int c_width, int c_height){
        std::cout<< *this << '\n';
        byte* new_data = (byte*)malloc(c_width * c_height * channels * sizeof(byte));
        int success = stbir_resize_uint8 (
            this->data, this->width, this->height, 0,
            new_data, c_width, c_height, 0,
            channels
        );
        if(!success){
            if(this->data) free(this->data);
            if(new_data) free(new_data);
            throw std::runtime_error("could not resize img!");
            return;
        }
        free(this->data);
        this->data = new_data;
        this->width = c_width;
        this->height = c_height;
        std::cout << *this << '\n';
    }

    void image::resize(float c_mult){
        int new_width = c_mult * width, new_height = c_mult * height;
        resize(new_height, new_height);
    }

    void image::save(const char* c_path){
        int success = stbi_write_png(c_path, width, height, channels, data, width * 4);
        if(!success) throw std::runtime_error("could not save img!");
    }

    #ifdef DEBUG
    std::ostream& operator<<(std::ostream& os, const image& img){
        os << "data:     " << (int*)img.data << '\n';
        os << "width:    " << img.width << '\n';
        os << "height:   " << img.height << '\n';
        os << "channels: " << img.channels << '\n';
        return os;
    }
    #endif
}