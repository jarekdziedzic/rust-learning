//
// Created by jarek on 20/01/2022.
//

#ifndef EXTERNAL_CALLS_GREETING_H
#define EXTERNAL_CALLS_GREETING_H

extern "C" {
    const char *get_greeting(const char* name);
    void free_greeting(const char* greeting);
};

#endif //EXTERNAL_CALLS_GREETING_H
