/*
 * Error.cpp
 *
 *  Created on: May 29, 2020
 *      Author: elias
 */

#include "Error.h"

void Error::makeError(Type errorType, uint32_t lineIndex, Token got, string expected)
{
    string msg = errorType2Str(errorType);
    msg += " at line " + to_string(lineIndex + 1);
    msg += " expected value: " + expected;
    msg += " got instead: " + got.value;
    printError(msg);
}

void Error::makeError(Type errorType, uint32_t lineIndex, Token got, Token::Type expected)
{
    string msg = errorType2Str(errorType);

    msg += " at line " + to_string(lineIndex + 1);
    msg += " expected type: " + tokenType2Str(expected);
    msg += " got instead: " + tokenType2Str(got.type);
    printError(msg);
}

void Error::makeError(Type errorType, uint32_t lineIndex, string input)
{
    string msg = errorType2Str(errorType);
    msg += " at line " + to_string(lineIndex + 1) + " ";
    msg += input;
    printError(msg);
}

string Error::errorType2Str(Type type)
{
    switch (type)
    {
    case Error::Type::lexing:
        return "[Lexing Error]";
    case Error::Type::parsing:
        return "[Parsing Error]";
    case Error::Type::syntax:
        return "[Syntax Error]";
    default:
        return "[Undifinde Error]";
    }
}

string Error::tokenType2Str(Token::Type type)
{
    switch (type)
    {
    case Token::Type::boolean:
        return "{boolean}";
    case Token::Type::identifier:
        return "{identifier}";
    case Token::Type::ignore:
        return "{ignore}";
    case Token::Type::keyword:
        return "{keyword}";
    case Token::Type::number:
        return "{number}";
    case Token::Type::symbol:
        return "{symbol}";
    default:
        return "{undifinde}";
    }
}

void Error::printError(string msg)
{
    std::cerr << msg << std::endl;
    exit(1);
}

