/*
 * TableParser.h
 *
 *  Created on: Jun 3, 2020
 *      Author: elias
 */

#ifndef TABLE_PARSER_H_
#define TABLE_PARSER_H_

#include <cstdint>
#include <vector>
#include <utility>
#include <cmath>
#include <math.h>

#include "../Shared/TableData.h"
#include "Error.h"

using namespace std;

class TableParser
{

public:
    // proces table
    vector<TableData> getTableData(vector<bool> tableStream, vector<uint32_t> inPins, vector<uint32_t> outPins);
    vector<TableData> getTableDataFill(vector<bool> tableStream, vector<uint32_t> inPins, vector<uint32_t> outPins, bool fill);
    vector<TableData> getTableDataCount(vector<bool> tableStream, vector<uint32_t> inPins, vector<uint32_t> outPins);

public:
    // setter function
    void setLineIndex(uint32_t);

private:
    uint32_t lineIndex;

private:
    // helper
    vector<TableData> buildTableData(vector<vector<bool>>, vector<uint32_t>, vector<uint32_t>);

    vector<vector<bool>> match(vector<vector<bool>>, uint32_t, bool);
    pair<uint32_t, vector<bool>> matchLine(vector<bool>, uint32_t);

    vector<vector<bool>> splitRows(vector<bool>, uint32_t);

    uint32_t bool2Int(vector<bool>);
};

#endif /* TABLE_PARSER_H_ */