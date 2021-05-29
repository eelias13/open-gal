/*
 * TableParser.cpp
 *
 *  Created on: Jun 3, 2020
 *      Author: elias
 */

#include "TableParser.h"

// ------------------------------------ proces table ------------------------------------
vector<TableData> TableParser::getTableData(vector<bool> tableStream, vector<uint32_t> inPins, vector<uint32_t> outPins)
{
    // cheks the size of tableStream.
    if (tableStream.size() != pow(2, inPins.size()) * (outPins.size() + inPins.size()))
    {
        Error::makeError(Error::Type::parsing, lineIndex, "incorrect table shape");
        return {};
    }

    // the logic is the same as in getTableDataFill the false (on the end) is arbitrarily
    return getTableDataFill(tableStream, inPins, outPins, false);
}

vector<TableData> TableParser::getTableDataFill(vector<bool> tableStream, vector<uint32_t> inPins, vector<uint32_t> outPins, bool fill)
{
    /*
    * splits the tableStream in to rows
    * Eaxample tableStream = 111000010100 
    * tabel is 
    * 11 1
    * 00 0
    * 01 0
    * 10 0
    */
    vector<vector<bool>> temp2D = splitRows(tableStream, outPins.size() + inPins.size());
    /*
    * sorts the rows like in the example above the first row is the last one 
    * and completes the table if rows are not defined
    * and removes the first part of the table
    */
    vector<vector<bool>> table2D = match(temp2D, outPins.size(), fill);

    return buildTableData(table2D, inPins, outPins);
}

vector<TableData> TableParser::getTableDataCount(vector<bool> tableStream, vector<uint32_t> inPins, vector<uint32_t> outPins)
{

    if (tableStream.size() != pow(2, inPins.size()) * outPins.size())
    {
        Error::makeError(Error::Type::parsing, lineIndex, "incorrect table shape");
        return {};
    }

    vector<vector<bool>> table2D;

    // look in Symbols.h for further information
#ifdef COUNT_VERTICAL
    for (uint32_t i = 0; i < outPins.size(); i++)
    {
        vector<bool> temp;
        for (int j = 0; j < pow(2, inPins.size()); j++)
            temp.push_back(tableStream.at(i * pow(2, inPins.size()) + j));
        table2D.push_back(temp);
    }
#else
    for (uint32_t i = 0; i < outPins.size(); i++)
        table2D.push_back({});
    for (uint32_t i = 0; i < tableStream.size(); i++)
        table2D.at(i % outPins.size()).push_back(tableStream.at(i));
#endif

    return buildTableData(table2D, inPins, outPins);
}

// ------------------------------------ setter function ------------------------------------

void TableParser::setLineIndex(uint32_t lineIndex) { this->lineIndex = lineIndex; }

// ------------------------------------ helper ------------------------------------
vector<TableData> TableParser::buildTableData(vector<vector<bool>> table2D, vector<uint32_t> inPins, vector<uint32_t> outPins)
{

    if (table2D.size() != outPins.size())
        return {};

    vector<TableData> tableData;
    for (uint32_t i = 0; i < table2D.size(); i++)
    {
        TableData temp;
        temp.m_Table = table2D.at(i);
        temp.m_InputPins = inPins;
        temp.m_OutputPin = outPins.at(i);
        tableData.push_back(temp);
    }
    return tableData;
}

vector<vector<bool>> TableParser::splitRows(vector<bool> boolVec, uint32_t len)
{
    vector<vector<bool>> rows;
    uint32_t index = 0;
    while (boolVec.size() != index * len)
    {
        for (uint32_t i = 0; i < len; i++)
            if (i == 0)
                rows.push_back(vector<bool>{boolVec.at(i + len * index)});
            else
                rows.at(index).push_back(boolVec.at(i + len * index));
        index++;
    }
    return rows;
}

pair<uint32_t, vector<bool>> TableParser::matchLine(vector<bool> boolVec, uint32_t outLen)
{
    vector<bool> temp;

    for (uint32_t i = 0; i < boolVec.size() - outLen; i++)
        temp.push_back(boolVec.at(i));

    uint32_t index = bool2Int(temp);
    temp = {};

    for (uint32_t i = 0; i < outLen; i++)
        temp.push_back(boolVec.at(boolVec.size() - outLen + i));

    return pair<uint32_t, vector<bool>>(index, temp);
}

uint32_t TableParser::bool2Int(vector<bool> boolVec)
{
    uint32_t result = 0;
    for (uint32_t i = 0; i < boolVec.size(); i++)
        if (boolVec.at(i))
            result += pow(2, boolVec.size() - i - 1);
    return result;
}

vector<vector<bool>> TableParser::match(vector<vector<bool>> vec2D, uint32_t outLen, bool init)
{
    vector<vector<bool>> result;

    for (uint32_t i = 0; i < outLen; i++)
    {
        result.push_back(vector<bool>{init});
        for (uint32_t j = 1; j < pow(2, vec2D.at(0).size() - outLen); j++)
            result.at(i).push_back(init);
    }

    for (vector<bool> b : vec2D)
    {
        pair<uint32_t, vector<bool>> p = matchLine(b, outLen);
        for (uint32_t i = 0; i < p.second.size(); i++)
            result.at(i).at(p.first) = p.second.at(i);
    }

    return result;
}