/*
 * EasyGAL.cpp
 *
 *  Created on: May 28, 2020
 *      Author: elias
 */

#include <iostream>
#include <vector>

#include "Shared/Validate.h"
#include "Shared/TableData.h"
#include "Shared/Dependencies/json.hpp"
#include "Shared/Utility.h"
#include "Shared/API.h"

#include "Parser/Parser.h"
#include "Parser/Error.h"

#include "Translator/Translator.hpp"
#include "Translator/Configs.h"

using namespace std;

uint8_t ConvertBoolArrayToByte(vector<bool> source)
{
	uint8_t result = 0;
	for (int i = 0; i < 8; i++)
		if (source[i])
			result |= (uint8_t)(1 << (7 - i));
	return result;
}

void printTableData(TableData tableData)
{
	printf("TableData { output_pin: %d, enable_flip_flop: %s, input_pins: [", tableData.m_OutputPin, tableData.m_EnableFlipFlop ? "true" : "false");
	for (uint32_t pin : tableData.m_InputPins)
		printf("%d, ", pin);
	printf("], table: [");
	for (bool b : tableData.m_Table)
		printf("%s, ", b ? "true" : "false");
	printf("]}\n");
}

void printNewTableData(TableData tableData)
{
	printf("TableData::new(vec![ ");
	for (uint32_t pin : tableData.m_InputPins)
		printf("%d, ", pin);
	printf("], %d, vec![", tableData.m_OutputPin);
	for (bool b : tableData.m_Table)
		printf("%s, ", b ? "true" : "false");
	printf("], %s)\n", tableData.m_EnableFlipFlop ? "true" : "false");
}

void printFusesBytes(vector<bool> Fuses)
{
	int index = 0;
	vector<bool> byte;

	for (int i = 0; i < Fuses.size() / 8; i++)
	{
		for (int j = 0; j < 8; j++)
			byte.push_back(Fuses.at(i * 8 + j));

		printf("0x%02hhX, ", ConvertBoolArrayToByte(byte));
		byte.clear();
	}
}

bool BuildFromExpression(DNF::Expression Expression, uint32_t iNumRows, uint32_t iRowLength, vector<bool> &FuseList, Configs::CircuitConfig *pConfig)
{
	if (!Fuses::Output::IsValid(Expression.m_OutputPin, pConfig))
	{
		ERROR("%s", "Expression has invalid output pin");
		return false;
	}
	else if (!Expression.m_Rows.size() || !iNumRows || !iRowLength)
	{
		ERROR("%s", "Invalid parameters");
		return false;
	}
	else if (Expression.m_Rows.size() > Fuses::Output::MaximumTerms(Expression.m_OutputPin, pConfig))
	{
		ERROR("%s", "Too many terms for given output pin");
		return false;
	}

	if (FuseList.size())
		FuseList.clear();

	FuseList.resize(iNumRows * iRowLength);

	//	Enable Output.

	std::fill(FuseList.begin(), FuseList.begin() + iRowLength, true);

	//	Start writing DNF terms.

	for (uint32_t TermIndex = 0; TermIndex < Expression.m_Rows.size(); TermIndex++)
	{
		std::fill(FuseList.begin() + iRowLength + TermIndex * iRowLength, FuseList.begin() + iRowLength + TermIndex * iRowLength + iRowLength, true);

		for (uint32_t PinIndex = 0; PinIndex < Expression.m_Rows[TermIndex].m_Pins.size(); PinIndex++)
		{
			int Index = Fuses::PinToIndex(
				Expression.m_Rows[TermIndex].m_Pins[PinIndex].m_PinNumber,
				Expression.m_Rows[TermIndex].m_Pins[PinIndex].m_Inverted,
				Expression.m_EnableFlipFlop ? MacrocellMode::MODE_REGISTERED_HIGH : MacrocellMode::MODE_COMBINATORIAL_HIGH,
				pConfig);

			if (Index == -1)
			{
				ERROR("%s", "Couldn't resolve PIN index");
				return false;
			}

			FuseList[iRowLength + TermIndex * iRowLength + Index] = false;
		}
	}

	return true;
}

int main(int argc, char *argv[])
{

	Configs::CircuitConfig Config;
	vector<uint32_t> inputPins;
	vector<uint32_t> outputPins;
	initDeviceType(Config, "g22v10", inputPins, outputPins);

	vector<json> json_vec{R"(
    {
      "dff": true,
      "inputPins": [10, 11],
      "outputPin": 23,
      "table": [false, false, true, false]
    }
)"_json,
						  R"({
      "dff": false,
      "inputPins": [10, 11],
      "outputPin": 17,
      "table": [false, false, false, true]
    })"_json,
						  R"({
      "dff": false,
      "inputPins": [10, 11],
      "outputPin": 19,
      "table": [false, true, true, false]
    })"_json,
						  R"({
      "dff": false,
      "inputPins": [10, 11],
      "outputPin": 18,
      "table": [false, true, true, true]
    })"_json,
						  R"({
      "dff": true,
      "inputPins": [3, 2],
      "outputPin": 23,
      "table": [true, true, false, true]
    })"_json,
						  R"({
      "dff": true,
      "inputPins": [3, 2],
      "outputPin": 23,
      "table": [false, true, true, false]
    })"_json};
	vector<TableData> TruthTables = api::parseTableDataArray(json_vec);

	printf("\n\nvec![");

	for (TableData TruthTable : TruthTables)
	{
		printNewTableData(TruthTable);
		printf(", ");
	}
	printf("];\n\n");

	vector<DNF::Expression> Expressions;
	if (!DNF::Build(TruthTables, Expressions, &Config))
	{
		ERROR("%s", "couldn't build all DNF expressions");
		return false;
	}

	vector<bool> Fuses;

	Configs::CircuitConfig *ConfigPtr = std::addressof(Config);

	if (!Fuses::Build(Expressions, Fuses, &Config))
	{
		ERROR("%s", "couldn't generate all fuses for given expressions");
		return false;
	}

	printf("\n\n");
	for (DNF::Expression e : Expressions)
		DNF::printNewExpression(e);
	printf("\n\nFuses %ld\n", Fuses.size());
	printf("vec![");
	printFusesBytes(Fuses);
	printf("]\n");
}