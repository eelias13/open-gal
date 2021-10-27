/*
 * Error.h
 *
 *  Created on: May 29, 2020
 *      Author: elias
 */

#ifndef ERROR_H_
#define ERROR_H_

#include <string>
#include <iostream>

#include "Token.h"
#include "Symbols.h"

using namespace std;

namespace Error
{
    enum Type
    {
        parsing,
        syntax,
        lexing
    };

    void makeError(Type errorType, uint32_t lineIndex, Token got, string expected);
    void makeError(Type errorType, uint32_t lineIndex, Token got, Token::Type expected);
    void makeError(Type errorType, uint32_t lineIndex, string input);

    string tokenType2Str(Token::Type type);
    string errorType2Str(Type type);

    void printError(string msg);
} // namespace Error

#endif /* ERROR_H_ */