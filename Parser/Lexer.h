/*
 * Lexer.h
 *
 *  Created on: May 28, 2020
 *      Author: elias
 */

#ifndef LEXER_H_
#define LEXER_H_

#include <cstdint>
#include <string>
#include <vector>
#include <fstream>

#include "Symbols.h"
#include "Token.h"
#include "Error.h"

using namespace std;

class Lexer
{
public:
	// constructor
	Lexer(string path);
	Lexer();

public:
	// public function
	Token next();

public:
	// setters and getters
	bool isFinished();
	uint16_t getLineIndex();
	uint16_t getCharIndex();

private:
	uint16_t lineIndex;
	uint16_t charIndex;
	string line;
	bool eof;
	ifstream inReader;

private:
	char currentChar;
	Token token;

private:
	// lex functions
	void lexComment();
	void lexSymbol();
	void lexArrow();
	void lexBool();
	void lexNum();
	void lexWord();

private:
	// helpful functions
	bool isNum();
	bool isKeyword();
	bool isSpecial();
	void nextChar();

private:
	// error handling
	void lexingError(string msg);
};

#endif /* LEXER_H_ */
