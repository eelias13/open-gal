/*
 * Parser.h
 *
 *  Created on: May 29, 2020
 *      Author: elias
 */

#ifndef PARSER_H_
#define PARSER_H_

#include <string>
#include <vector>
#include <utility>
#include <math.h>

#include "../Shared/TableData.h"
#include "Lexer.h"
#include "Token.h"
#include "Symbols.h"
#include "Error.h"
#include "FunctionParser.h"
#include "TableParser.h"

using namespace std;

class Parser
{
public:
    // constructor
    Parser(string path);

public:
    // public function
    vector<TableData> parse();

private:
    vector<TableData> tables;
    vector<pair<string, uint32_t>> alias;

    Lexer lexer;
    Token currentToken;

    // used for tables
    bool isFill;
    bool fill;
    bool isCount;

    FunctionParser functionParser;
    TableParser tableParser;

private:
    // main parser function
    void parseNext();
    void parsePin();
    void parseTable();
    void parseIdentifier();

private:
    // helper for parsePin()
    string pin2Str(uint32_t pin);
    uint32_t str2Pin(string pinName);

private:
    // helper for tableParse()
    vector<uint32_t> getIdentifier();
    void extraFunction();
    vector<bool> getBooltable();
    void insertBooltable(vector<bool> &boolTable, string strTable);

private:
    // helper for parseIdentifier()
    void insertDFF(uint32_t pin);
    vector<Token> getExpression();
    bool validExpression();
    TableData assembleTableFromFunc(string outName, vector<Token> expression);

private:
    // helpful functions
    void nextToken();

    bool isToken(char expected);
    bool isToken(string expected);
    bool isToken(Token::Type expected);

    void expect(char expected);
    void expect(string expected);
    void expect(Token::Type expected);

    uint32_t getInt(char c);
    uint32_t str2Int(string str);

private:
    // error handling
    void parsingError(string msg);
    void syntaxError(string expected);
    void syntaxError(Token::Type expected);
};

#endif /* PARSER_H_ */