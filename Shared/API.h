/*
 * API.h
 *
 *  Created on: Aug 21, 2020
 *      Author: elias
 */

#ifndef API_H_
#define API_H_

#include <string>
#include <vector>
#include <iostream>
#include <iomanip>

#include "Dependencies/json.hpp"
#include "Utility.h"

#include "../Parser/Parser.h"
#include "../Translator/Translator.hpp"

using namespace std;
using json = nlohmann::json;

namespace api
{
    void code2TableData(string easyGALCode, string outputFileName, string deviceName);
    TableData parseTableData(json tdJson);
    vector<TableData> parseTableDataArray(json array);
    void tableData2jedec(string tableDataJson, string outputFileName, string deviceName);
} // namespace api

#endif /* API_H_ */