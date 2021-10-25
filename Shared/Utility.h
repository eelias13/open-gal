/*
 * Utility.h
 *
 *  Created on: Aug 21, 2020
 *      Author: elias
 */

#ifndef UTILITY_H_
#define UTILITY_H_

#include <iostream>

#include "Validate.h"
#include "TableData.h"
#include "Dependencies/json.hpp"

#include "../Translator/Configs.h"

using namespace std;

void showHelpMenu();
void initDeviceType(Configs::CircuitConfig &DeviceType, string deviceName, vector<uint32_t> &inputPins, vector<uint32_t> &outputPins);
string getFileEnding(string fileName);
void checkFileEnding(string fileName, string fileEnding);

#endif /* UTILITY_H_ */