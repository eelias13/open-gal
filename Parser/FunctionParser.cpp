/*
 * FunctionParser.cpp
 *
 *  Created on: May 31, 2020
 *      Author: elias
 */

#include "FunctionParser.h"

// ------------------------------------ constructor ------------------------------------

FunctionParser::FunctionParser()
{
    // sets precedence for parsing, the higher het index the higher the binding. Example: and before or
    operatorPrecedence[0] = OR;
    operatorPrecedence[1] = XOR;
    operatorPrecedence[2] = AND;
    operatorPrecedence[3] = NOT;
}

// ------------------------------------ public functions ------------------------------------

// this is where the magic happens. Read this first or you won't undestand what the functions are used for
vector<bool> FunctionParser::parse(vector<Token> expression, uint32_t lineIndex)
{
    this->lineIndex = lineIndex;

    // resets names from last parse and fills them with new names
    initNames(expression);

    // makes error if expression is not valid
    isValide(expression);

    // the lookuptable is used to set the leaf node values for the tree: Example: 'and' is not a leaf node but 'a' is
    initLookup();

    // builds tree
    Node *tree = nextNode(expression, NULL);

    vector<bool> result;

    for (uint32_t index = 0; index < pow(2, names.size()); index++)
    {
        result.push_back(evalNode(tree));
        // changing the values for the leaf nodes by changing the value the leaf node is pointing to
        updateLookup();
    }

    // frees all allocat memory
    deleteNode(tree);
    deleteLookup();

    return result;
}

vector<string> FunctionParser::getNames() { return names; }

// ------------------------------------ lookup ------------------------------------

void FunctionParser::initLookup()
{
    // heap allocats memory for tree leaf values and fills them with false for first parse
    lookupLength = names.size();

    // for ever reson new doson't work on linux and malloc doson't work on windows, but now idea on other platforms (or compilers)
#ifdef _WIN32
    lookupValues = new bool[lookupLength];
#elif __linux__
    lookupValues = (bool *)malloc((lookupLength - 1) * sizeof(bool));
#else
    Error::printError("this platform is not suported plea use linux or Windows");
#endif

    finished = false;

    for (uint32_t i = 0; i < lookupLength; i++)
    {
        pair<string, bool *> p;
        p.first = names.at(i);
        p.second = (lookupValues + (sizeof(bool) * i));
        lookup.push_back(p);

        *(p.second) = false;
    }
}

void FunctionParser::updateLookup()
{
    for (int i = lookupLength - 1; i >= 0; i--)
    {
        bool *currentValue = lookupValues + (sizeof(bool) * i);
        *currentValue = !(*currentValue);
        if (*currentValue == true)
            return;
    }
    finished = true;
}

void FunctionParser::deleteLookup()
{
    free(lookupValues);
    lookupLength = 0;
    lookup = {};
}

bool *FunctionParser::getBoolPtr(Token token)
{
    for (pair<string, bool *> p : lookup)
        if (p.first == token.value)
            return p.second;
    return NULL;
}

// ------------------------------------ functions for getNames ------------------------------------

// searches expression for identifiers and saves them in vector<string> names
void FunctionParser::initNames(vector<Token> expression)
{
    // resets names vector from last parse
    names = {};

    for (Token t : expression)
        if (t.type == Token::Type::identifier && !strInVec(names, t.value))
            names.push_back(t.value);
}

bool FunctionParser::strInVec(vector<string> vec, string str)
{
    for (string s : vec)
        if (s == str)
            return true;
    return false;
}

// ------------------------------------ operator precedence ------------------------------------

// resoves precedence of char
uint8_t FunctionParser::precedenceOf(char c)
{
    if (c == operatorPrecedence[0])
        return 0;
    if (c == operatorPrecedence[1])
        return 1;
    if (c == operatorPrecedence[2])
        return 2;
    if (c == operatorPrecedence[3])
        return 3;

    return 0xff;
}

// ------------------------------------ validate ------------------------------------

void FunctionParser::isValide(vector<Token> expression)
{
    if (expression.size() == 0)
        parsingError("no expression");

    // increments on '(' and decrements on ')' should never be -1. Exampel: (a) & b ) is invalid
    uint32_t counterParentheses = 0;
    // counts all binary operator (and, or, xor) and cheks if ther are enough identifiers. Exampel: !a & & b is invalid
    uint32_t counterBinaryOperator = 0;
    // afer an identifier ther must be an operator. Exampel: a a & b is invalid
    bool lastIdentifier = false;

    for (Token t : expression)
    {
        if (t.type == Token::Type::symbol)
        {
            char c = t.value.at(0);
            if (!valideSymbol(c))
            {
                string msg = "(, ), ";
                msg += AND;
                msg += ", ";
                msg += OR;
                msg += ", ";
                msg += NOT;
                msg += ", ";
                msg += XOR;
                syntaxError(t, msg);
            }

            if (isBinary(c))
                counterBinaryOperator++;

            if (c == '(')
                counterParentheses++;
            if (c == ')')
            {
                if (counterParentheses < 0)
                    syntaxError(t, "(");
                counterParentheses--;
            }
            lastIdentifier = false;
        }
        else
        {
            if (t.type != Token::Type::identifier)
                syntaxError(t, Token::Type::symbol);
            if (lastIdentifier == true)
                syntaxError(t, Token::Type::symbol);
            lastIdentifier = true;
        }
    }

    if (counterBinaryOperator != names.size() - 1)
        syntaxError(expression.at(expression.size() - 1), Token::Type::identifier);

    if (counterParentheses != 0)
        syntaxError(expression.at(expression.size() - 1), ")");
}

bool FunctionParser::isBinary(char c)
{
    switch (c)
    {
    case AND:
    case OR:
    case XOR:
        return true;

    default:
        return false;
    }
}

bool FunctionParser::valideSymbol(char c)
{
    switch (c)
    {
    case '(':
    case ')':
    case AND:
    case OR:
    case XOR:
    case NOT:
        return true;

    default:
        return false;
    }
}

// ------------------------------------ split functions ------------------------------------

struct FunctionParser::Bundle
{
    vector<Token> left;
    vector<Token> right;
    Token center;
};

// this is a very imortant function to buid the tree. It splits the function in to 3 parts Example: a & b | (c | d) -> a & b, |, (c | d)
// this works with any arbitrarily complicated function
FunctionParser::Bundle FunctionParser::split(vector<Token> expression)
{
    // is leavnode
    if (expression.size() == 1)
    {
        Bundle bundle;
        bundle.center = expression.at(0);
        bundle.left = {};
        bundle.right = {};
        return bundle;
    }

    // isParentheses: Example (a & b) returns true (a) & (b) return false
    if (isParentheses(expression))
        // splitParentheses: removes the first and last char
        return splitParentheses(expression);
    return splitBineryOperator(expression);
}

bool FunctionParser::isParentheses(vector<Token> expression)
{
    if (expression.at(0).value.at(0) != '(')
        return false;
    if (expression.at(expression.size() - 1).value.at(0) != ')')
        return false;

    // if counter is ever 0 then I know that i have this case (a) & (b)
    int counter = 1;
    for (uint32_t i = 1; i < expression.size() - 1; i++)
    {
        if (counter == 0)
            return false;
        char c = expression.at(i).value.at(0);
        if (c == '(')
            counter++;
        if (c == ')')
            counter--;
    }
    return true;
}

FunctionParser::Bundle FunctionParser::splitParentheses(vector<Token> expression)
{
    vector<Token> temp;
    for (uint32_t i = 1; i < expression.size() - 1; i++)
        temp.push_back(expression.at(i));
    return split(temp);
}

FunctionParser::Bundle FunctionParser::splitNot(vector<Token> expression)
{
    Bundle bundle;
    bundle.center = expression.at(0);
    for (uint32_t i = 1; i < expression.size(); i++)
        bundle.left.push_back(expression.at(i));
    bundle.right = {};
    return bundle;
}

FunctionParser::Bundle FunctionParser::splitBineryOperator(vector<Token> expression)
{
    // splitIndex: gives the index of the operator by which the expression must be split
    // Exampel: "(a | b) & c" the function would return 6 witch is the index of the char '&'
    uint32_t index = splitIndex(expression);

    // if it is 0 it must be not or an error
    if (index == 0)
    {
        if (expression.at(0).value.at(0) == NOT)
            return splitNot(expression);
        string msg = "can not split expression: ";
        for (Token t : expression)
            msg += t.value;
        parsingError(msg);
    }

    Bundle bundle;
    for (uint32_t i = 0; i < index; i++)
        bundle.left.push_back(expression.at(i));
    bundle.center = expression.at(index);
    for (uint32_t i = index + 1; i < expression.size(); i++)
        bundle.right.push_back(expression.at(i));
    return bundle;
}

uint32_t FunctionParser::splitIndex(vector<Token> expression)
{
    uint32_t lowestOperatorIndex = 0;
    uint8_t lowestOperatorScore = 0xff;
    uint8_t parentheses = 0;

    for (uint32_t i = 0; i < expression.size(); i++)
    {
        Token t = expression.at(i);
        if (t.type != Token::Type::symbol)
            continue;

        if (parentheses != 0)
        {
            if (t.value.at(0) == ')')
                parentheses--;
            if (t.value.at(0) == '(')
                parentheses++;
            continue;
        }

        if (t.value.at(0) == '(')
            parentheses++;

        if (precedenceOf(t.value.at(0)) < lowestOperatorScore)
        {

            lowestOperatorScore = precedenceOf(t.value.at(0));
            lowestOperatorIndex = i;
            if (lowestOperatorScore == 0)
                break;
        }
    }

    return lowestOperatorIndex;
}

// ------------------------------------ nodes ------------------------------------

struct FunctionParser::Node
{
    Node *parent;
    Node *left;
    Node *right;

    Token token;
    bool *value;
};

struct FunctionParser::Node *FunctionParser::newNode(Token token, FunctionParser::Node *parent)
{
    // for ever reson new doson't work on linux and malloc doson't work on windows, but now idea on other platforms (or compilers)
#ifdef _WIN32
    Node *node = new Node();
#elif __linux__
    struct Node *node = (struct Node *)malloc(sizeof(struct Node));
#else
    Error::printError("this platform is not suported plea use linux or Windows");
#endif

    node->token = token;
    // gets the pointer of the lookuptabels leaf node if it isn't a leaf node getBoolPtr returns NULL
    node->value = getBoolPtr(token);
    node->parent = parent;
    node->left = NULL;
    node->right = NULL;
    return (node);
};

FunctionParser::Node *FunctionParser::nextNode(vector<Token> expression, Node *pNode)
{
    Bundle bundle = split(expression);
    Node *node = newNode(bundle.center, pNode);

    // builds tree recursively
    if (bundle.left.size() != 0)
        (*node).left = nextNode(bundle.left, node);
    if (bundle.right.size() != 0)
        (*node).right = nextNode(bundle.right, node);
    return node;
}

bool FunctionParser::evalNode(Node *node)
{
    // if levenode then return value from lookuptable
    if ((*node).value != NULL)
        return *((*node).value);

    // walks down the tree depth first (left) and evaluates it
    switch ((*node).token.value.at(0))
    {
    case AND:
        return evalNode((*node).left) & evalNode((*node).right);
    case OR:
        return evalNode((*node).left) | evalNode((*node).right);
    case XOR:
        return evalNode((*node).left) ^ evalNode((*node).right);
    case NOT:
        return !evalNode((*node).left);
    default:
        parsingError("can not resolve token " + (*node).token.value);
        return false;
    }
}

void FunctionParser::deleteNode(Node *node)
{
    if ((*node).left != NULL)
        deleteNode((*node).left);
    if ((*node).right != NULL)
        deleteNode((*node).right);
    free(node);
}

// ------------------------------------ Error handling ------------------------------------

void FunctionParser::syntaxError(Token got, string expected) { Error::makeError(Error::Type::syntax, lineIndex, got, expected); }
void FunctionParser::syntaxError(Token got, Token::Type expected) { Error::makeError(Error::Type::syntax, lineIndex, got, expected); }
void FunctionParser::parsingError(string msg) { Error::makeError(Error::Type::parsing, lineIndex, msg); }
