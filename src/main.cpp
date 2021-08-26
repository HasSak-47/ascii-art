#include<exception>
#include<iostream>
#include<fstream>
#include<image.h>

using namespace sk;

int assign_color(char* str, color col){
    char* c_format = color_text(col);
    for(size_t i = 0; i < 9; ++i) str[i] = c_format[i];
    return 9;
}

int main(int argc, char* argv[]){
    if(argc == 0 ){
        std::cerr << "what the fuck!\n";
        return -1;
    }
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
                bool colors[4] = {
                    img.data[index + 0] > 127,
                    img.data[index + 1] > 127,
                    img.data[index + 2] > 127,
                    img.data[index + 3] > 127
                };
                if(!colors[3]){
                    if(index + 7 <= max_img_size && index != 0){
                        if(img.data[index + 4 + 3] < 127 && img.data[index - 4 + 3] < 127) txt_data[txt_index++] = ' ';
                        else {
                            if(prev_color != color::white){
                                assign_color(&txt_data[txt_index], color::white);
                                txt_data[txt_index += 9] = '#';
                                txt_index++;
                                prev_color = color::white;
                            }
                            else{
                                txt_data[txt_index++] = '#';
                            }
                        }
                    }
                    else txt_data[txt_index++] = ' ';
                }
                else{
                    if((color)(colors[0] + 2 * colors[1] + 4 * colors[2]) != prev_color){
                        assign_color(&txt_data[txt_index], (color)(colors[0] + 2 * colors[1] + 4 * colors[2]) );
                        txt_data[txt_index += 9] = '#';
                        txt_index++;
                        prev_color = (color)(colors[0] + 2 * colors[1] + 4 * colors[2]);
                    }
                    else{
                        txt_data[txt_index++] = '#';
                    }
                }
            }
            txt_data[txt_index++] = ' ';
            txt_data[txt_index++] = '\n';
        }

        std::ofstream file("text.txt");
        if(!file.is_open()) throw std::runtime_error("file not found!");
        file.write(txt_data, txt_index);
        file.close();

        std::cout << "line: " << __LINE__ << '\n';
        delete[] txt_data;
        std::cout << "line: " << __LINE__ << '\n';
        
    }catch(std::runtime_error& err){
        std::cout<< err.what() << '\n';
        return -1;
    }
    return 0;
}