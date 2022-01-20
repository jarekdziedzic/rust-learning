//
// Created by jarek on 20/01/2022.
//

#include "greeting.h"
#include <cstdio>
#include <cstring>
#include <cstdlib>


extern "C" {
    const char* get_greeting(const char* name) {
        char buf[255];
        snprintf(static_cast<char*>(buf), sizeof(buf)/sizeof(buf[0]), "Hello %s!", name);
        return strndup(buf, strlen(buf) + 1);
    }

    void free_greeting(const char* greeting) {
        free(const_cast<char*>(greeting));
    }
}
