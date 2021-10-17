#include<exception>
#include<iostream>
#include<fstream>
#include<memory>
#include<cmath>

#define STB_IMAGE_IMPLEMENTATION
#include<stb_image.h>

#define STB_IMAGE_RESIZE_IMPLEMENTATION
#include<stb_image_resize.h>

#define STB_IMAGE_WRITE_IMPLEMENTATION
#include<stb_image_write.h>

#define default_height 25
#define default_width 50

const char default_path[] = "ascii_art.txt";
const char intensity[] = {' ', '#'};

int str_to_int(const char* str){
    int result = 0;
    while(*str && *str >= '0' && *str <= '9'){
        result *= 10;
        result += *str - '0';
        str++;
    }
    return result;
}

int main(int argc, char* argv[]){
    const char* path = default_path;
    int output_height = default_height, possible_height;
    int output_width  = default_width, possible_width;
    switch (argc){
    case 1:
        std::cout << "not image path given\n";
        return -1;
    case 3:
        path = argv[2];
        break;
    case 4:
        possible_width = str_to_int(argv[3]);
        if(possible_width == 0) break;
        output_width = possible_width;
        output_height = output_width / 2;
        break;
    case 5:
        possible_width = str_to_int(argv[3]);
        possible_height = str_to_int(argv[4]);
        if(possible_width == 0|| possible_height == 0) break;
        output_width = possible_width;
        output_height = possible_height;
        break;
    default:
        break;
    }

    int width, height, channels;
    std::unique_ptr<unsigned char[]> image( stbi_load(argv[1], &width, &height, &channels, 0) );

    if(!image.get()){
        std::cout << "image at "<< argv[1] <<" could not be loaded\n";
        return 1;
    }
    if(channels < 4){
        std::cout << "image is not a png!\n";
        return 1;
    }
    size_t new_img = (4 * (output_height) * output_width);
    std::unique_ptr<unsigned char[]> resized = std::make_unique<unsigned char[]>(new_img);
    stbir_resize_uint8(
        image.get(), width, height, 0,
        resized.get(), output_width, output_height, 0,
        4
    );
    image.reset();
    std::ofstream ascii_art(path);

    for(size_t i = 0; i < new_img / 4; ++i){
        char intense = (resized[(4 * i) + 3] > 128)? '#' : ' ';
        std::cout << intense;
        ascii_art << intense;
        if(i % output_width == (output_width - 1)) {
            std::cout << '\n';
            ascii_art << '\n';
        }
    }
    return 0;
}