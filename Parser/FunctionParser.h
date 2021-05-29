/*
 * FunctionParser.cpp
 *
 *  Created on: May 31, 2020
 *      Author: elias
 */

#define FUNCTIONPARSER_H_
#ifdef FUNCTIONPARSER_H_

#include <string>
#include <vector>
#include <stack>
#include <utility>
#include <math.h>

#include "Token.h"
#include "Symbols.h"
#include "Error.h"

using namespace std;

class FunctionParser
{
public:
    // constructor
    FunctionParser();

public:
    // public functions
    vector<bool> parse(vector<Token> expression, uint32_t lineIndex);
    vector<string> getNames();

private:
    // lookup
    vector<pair<string, bool *>> lookup;
    bool *lookupValues;
    uint32_t lookupLength;

    void initLookup();
    void updateLookup();
    void deleteLookup();
    bool *getBoolPtr(Token token);

private:
    // for getNames
    void initNames(vector<Token> expression);
    bool strInVec(vector<string> vec, string str);
    vector<string> names;

private:
    // operator precedence
    uint8_t operatorPrecedence[4];
    uint8_t precedenceOf(char c);

private:
    // validate
    void isValide(vector<Token> expression);
    bool isBinary(char c);
    bool valideSymbol(char c);

private:
    // split functions
    struct Bundle;
    Bundle split(vector<Token> expression);
    Bundle splitParentheses(vector<Token> expression);
    bool isParentheses(vector<Token> expression);
    Bundle splitNot(vector<Token> expression);
    Bundle splitBineryOperator(vector<Token> expression);
    uint32_t splitIndex(vector<Token> expression);

private:
    // nodes
    struct Node;
    struct Node *newNode(Token token, FunctionParser::Node *parent);
    Node *nextNode(vector<Token> expression, Node *pNode);
    bool evalNode(Node *node);
    void deleteNode(Node *node);

private:
    // Error handling
    void syntaxError(Token got, Token::Type expected);
    void syntaxError(Token got, string expected);
    void parsingError(string msg);

private:
    bool finished;
    uint32_t lineIndex;
};

#endif /* FUNCTIONPARSER_H_ */