/*
 * Parser.cpp
 *
 *  Created on: May 29, 2020
 *      Author: elias
 */

#include "Parser.h"

// constructor

Parser::Parser(string path)
{
    // initialise variables
    lexer = Lexer(path);
    isFill = false;
    fill = false;
    isCount = false;

    // initialise extra parser
    functionParser = FunctionParser();
    tableParser = TableParser();
}

// public function
vector<TableData> Parser::parse()
{
    // ignore first token because it is the beginig of the file
    nextToken();
    while (!lexer.isFinished())
        parseNext();

    return tables;
}

// ------------------------------------ main parser function ------------------------------------
void Parser::parseNext()
{
    if (isToken(PIN))
        return parsePin();

    if (isToken(TABLE))
        return parseTable();

    if (isToken(Token::Type::identifier))
        return parseIdentifier();

    syntaxError(Token::Type::identifier);
}

void Parser::parsePin()
{
    string str;
    pair<string, uint32_t> temp;

    expect(PIN);
    str = currentToken.value;
    if (isToken(Token::Type::boolean))
        nextToken();
    else
        expect(Token::Type::number);
    temp.second = str2Int(str);
    expect("=");
    str = currentToken.value;
    expect(Token::Type::identifier);
    temp.first = str;
    expect(";");

    alias.push_back(temp);
}

void Parser::parseTable()
{
    isFill = false;
    fill = false;
    isCount = false;

    tableParser.setLineIndex(lexer.getLineIndex());

    expect(TABLE);
    expect("(");
    vector<uint32_t> inPins = getIdentifier();
    expect("->");
    vector<uint32_t> outPins = getIdentifier();
    expect(")");
    extraFunction();
    expect("{");
    vector<bool> boolTable = getBooltable();
    expect("}");

    vector<TableData> temp;
    if (isCount)
        temp = tableParser.getTableDataCount(boolTable, inPins, outPins);
    else if (isFill)
        temp = tableParser.getTableDataFill(boolTable, inPins, outPins, fill);
    else
        temp = tableParser.getTableData(boolTable, inPins, outPins);

    for (TableData t : temp)
        tables.push_back(t);
}

#include <iostream>

void Parser::parseIdentifier()
{
    string pinName = currentToken.value;
    expect(Token::Type::identifier);
    if (isToken("."))
    {
        expect(".");
        expect("dff");
        expect(";");
        insertDFF(str2Pin(pinName));

        return;
    }

    expect("=");
    vector<Token> expression = getExpression();
    expect(";");

    tables.push_back(assembleTableFromFunc(pinName, expression));
}

// ------------------------------------ helper for parsePin() ------------------------------------
string Parser::pin2Str(uint32_t pin)
{
    for (pair<string, uint32_t> p : alias)
        if (p.second == pin)
            return p.first;

    parsingError("pin " + to_string(pin) + " is not diffident");

    return "";
}

uint32_t Parser::str2Pin(string pinName)
{
    for (pair<string, uint32_t> p : alias)
        if (p.first == pinName)
            return p.second;

    parsingError("pin " + pinName + " is not diffident");

    return 0;
}

// ------------------------------------ helper for tableParse() ------------------------------------
vector<uint32_t> Parser::getIdentifier()
{
    Token t;
    vector<uint32_t> pins;

    t = currentToken;
    expect(Token::Type::identifier);
    pins.push_back(str2Pin(t.value));

    while (isToken(","))
    {
        expect(",");
        t = currentToken;
        expect(Token::Type::identifier);
        pins.push_back(str2Pin(t.value));
    }

    return pins;
}

void Parser::extraFunction()
{
    if (!isToken("."))
        return;

    expect(".");
    if (isToken(COUNT))
    {
        expect(COUNT);
        isCount = true;
        return;
    }

    expect(FILL);
    isFill = true;
    expect("(");
    string temp = currentToken.value;
    expect(Token::Type::boolean);
    expect(")");

    if (temp.size() != 1)
    {
        string msg = "";
        msg += ZERO;
        msg += " or ";
        msg += ONE;
        return syntaxError(msg);
    }

    if (temp.at(0) == ONE)
        fill = true;
}

vector<bool> Parser::getBooltable()
{
    vector<bool> boolTable;
    insertBooltable(boolTable, currentToken.value);
    expect(Token::Type::boolean);
    while (isToken(Token::Type::boolean))
    {
        insertBooltable(boolTable, currentToken.value);
        expect(Token::Type::boolean);
    }
    return boolTable;
}

// convert the string in to the vector<bool> and adds it to boolTable
void Parser::insertBooltable(vector<bool> &boolTable, string strTable)
{
    for (uint32_t i = 0; i < strTable.size(); i++)
        if (strTable.at(i) == ONE)
            boolTable.push_back(true);
        else
            boolTable.push_back(false);
}

// ------------------------------------ helper for parseIdentifier() ------------------------------------
void Parser::insertDFF(uint32_t pin)
{
    for (uint32_t i = 0; i < tables.size(); i++)
        if (tables.at(i).m_OutputPin == pin)
        {
            tables.at(i).m_EnableFlipFlop = true;
            return;
        }

    parsingError(pin2Str(pin) + " is not an output pin or not yet defined");
}

vector<Token> Parser::getExpression()
{
    vector<Token> tokens;
    if (!validExpression())
        syntaxError(Token::Type::identifier);

    while (validExpression())
    {
        tokens.push_back(currentToken);
        nextToken();
    }

    return tokens;
}

bool Parser::validExpression()
{
    if (isToken(Token::Type::symbol) && currentToken.value.size() == 1)
        switch (currentToken.value.at(0))
        {
        case AND:
        case OR:
        case XOR:
        case NOT:
        case '(':
        case ')':
            return true;
        default:
            return false;
        }

    if (isToken(Token::Type::identifier))
        return true;
    return false;
}

TableData Parser::assembleTableFromFunc(string outName, vector<Token> expression)
{
    TableData table;
    table.m_EnableFlipFlop = false;
    table.m_OutputPin = str2Pin(outName);

    table.m_Table = functionParser.parse(expression, lexer.getLineIndex());

    vector<uint32_t> inPins;
    for (string inName : functionParser.getNames())
        inPins.push_back(str2Pin(inName));
    table.m_InputPins = inPins;

    return table;
}

// ------------------------------------ helpful functions ------------------------------------

// requests the next token from the lexer and stors it in currentToken
void Parser::nextToken() { currentToken = lexer.next(); }

// compares curent token value to the expectad one
bool Parser::isToken(char expected) { return currentToken.value == "" + expected; }
bool Parser::isToken(string expected) { return currentToken.value == expected; }
bool Parser::isToken(Token::Type expected) { return currentToken.type == expected; }

// if it dosn't get the expected value it makes an error
// if it gets the expected value it requests the next token from the lexer
void Parser::expect(char expected) { expect("" + expected); }
void Parser::expect(string expected)
{
    if (currentToken.value != expected)
        syntaxError(expected);
    nextToken();
}
void Parser::expect(Token::Type expected)
{
    if (currentToken.type != expected)
        syntaxError(expected);
    nextToken();
}

uint32_t Parser::str2Int(string str)
{
    uint32_t result = 0;
    for (uint32_t i = 0; i < str.size(); i++)
        result += getInt(str.at(i)) * pow(10, str.size() - i - 1);
    return result;
}

uint32_t Parser::getInt(char c)
{
    switch (c)
    {
    case '0':
        return 0;
    case '1':
        return 1;
    case '2':
        return 2;
    case '3':
        return 3;
    case '4':
        return 4;
    case '5':
        return 5;
    case '6':
        return 6;
    case '7':
        return 7;
    case '8':
        return 8;
    case '9':
        return 9;
    default:

        parsingError(c + " is not a number");
        return 0;
    }
}

// ------------------------------------ error handling ------------------------------------
void Parser::parsingError(string msg) { Error::makeError(Error::Type::parsing, lexer.getLineIndex(), msg); }
void Parser::syntaxError(string expected) { Error::makeError(Error::Type::syntax, lexer.getLineIndex(), currentToken, expected); }
void Parser::syntaxError(Token::Type expected) { Error::makeError(Error::Type::syntax, lexer.getLineIndex(), currentToken, expected); }