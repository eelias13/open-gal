/*
 * Validate.cpp
 *
 *  Created on: Aug 21, 2020
 *      Author: elias
 */

#include "Validate.h"

void printError(string msg)
{
    cerr << msg << endl;
    exit(1);
}

bool inVec(vector<uint32_t> vec, uint32_t value)
{
    for (uint32_t temp : vec)
        if (value == temp)
            return true;
    return false;
}

void validate(vector<TableData> tables, vector<uint32_t> inputPins, vector<uint32_t> outputPins)
{
    for (uint32_t i = 0; i < tables.size(); i++)
    {
        for (uint32_t pin : tables.at(i).m_InputPins)
            if (!inVec(inputPins, pin))
                printError("pin " + to_string(pin) + " is not an input pin");

        if (!inVec(outputPins, tables.at(i).m_OutputPin))
            printError("pin " + to_string(tables.at(i).m_OutputPin) + " is not an output pin");

        if (pow(2, tables.at(i).m_InputPins.size()) != tables.at(i).m_Table.size())
            printError("table at output pin " + to_string(tables.at(i).m_OutputPin) + " has the wrong shape");

        for (uint32_t j = i + 1; j < tables.size(); j++)
        {
            if (tables.at(i).m_OutputPin == tables.at(j).m_OutputPin)
                printError("table at output pin " + to_string(tables.at(i).m_OutputPin) + " was defined twice");
        }
    }
}