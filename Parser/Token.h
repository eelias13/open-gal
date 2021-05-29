/*
 * Token.h
 *
 *  Created on: May 29, 2020
 *      Author: elias
 */

#ifndef TOKEN_H_
#define TOKEN_H_

#include <string>

struct Token
{
    enum Type
    {
        keyword,
        identifier,
        number,
        boolean,
        symbol,
        ignore
    };
    Type type;
    std::string value;
};

#endif