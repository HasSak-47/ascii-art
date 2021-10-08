#include<exception>
#include<iostream>
#include<fstream>
#include"headers/image.h"

using namespace sk;

int assign_color(char* str, color col){
    char* c_format = color_text(col);
    for(size_t i = 0; i < 9; ++i) str[i] = c_format[i];
    return 9;
}

int get_color(byte* pixel){
    return (
        (*(pixel + 0) > 127) * 1 +
        (*(pixel + 1) > 127) * 2 +
        (*(pixel + 2) > 127) * 4 +
        (*(pixel + 3) > 127) * 8
    );
}

int main(int argc, char* argv[]){
    if(argc == 1){
        std::cerr << "no image provided\n";
        return 1;
    }

    try{
        image img(argv[1]);

        if(img.channels != 4) throw std::runtime_error("not a transpartent img!");

        //float ratio =(float)max_height / img.width;
        //img.resize(ratio);
        img.resize(2 * max_height, max_height);
        img.save("scaled.png");

        char* txt_data = new char[img.width * img.height * 11];
        int txt_index = 0;
        const int max_img_size = img.channels * img.height * img.width;
        color prev_color = {};
        for(size_t i = 0; i <  img.height; ++i){
            for(size_t j = 0; j <  img.width; ++j){
                int index = 4 * (i * img.width + j);
                int colors = get_color(img.data + index);
                if(colors <= 7 || (colors - 8) == 0) txt_data[txt_index++] = ' ';
                else{
                    colors -= 8;
                    if((color)colors != prev_color){
                        std::cout << colors << '\n';
                        assign_color(&txt_data[txt_index], (color)colors);
                        txt_index += 9;
                        prev_color = (color)colors;
                    }
                    txt_data[txt_index++] = '#';
                }
            }
            txt_data[txt_index++] = ' ';
            txt_data[txt_index++] = '\n';
        }

        std::ofstream file("text.txt");
        if(!file.is_open()) throw std::runtime_error("file not found!");
        file.write(txt_data, txt_index);
        file.close();
        delete[] txt_data;
        
    }catch(std::runtime_error& err){
        std::cout<< err.what() << '\n';
        return -1;
    }
    return 0;
}