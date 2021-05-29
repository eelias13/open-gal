/*
 * Validate.h
 *
 *  Created on: Jul 7, 2020
 *      Author: elias
 */

#ifndef VALIDATE_H_
#define VALIDATE_H_

#include <vector>
#include <iostream>
#include <string>
#include <cmath>

#include "TableData.h"

using namespace std;

void printError(string msg);
bool inVec(vector<uint32_t> vec, uint32_t value);
void validate(vector<TableData> tables, vector<uint32_t> inputPins, vector<uint32_t> outputPins);

#endif /* VALIDATE_H_ */