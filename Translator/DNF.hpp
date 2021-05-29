#pragma once

/*
*	This namespace is used for generating DNF expression from truth tables.
*	The output is a datastructure in which the DNF "and" and "or" rows are stored.
*/

#include <bitset>
#include <cmath>

#include "Helper.hpp"
#include "../Shared/TableData.h"
#include "Configs.h"

//	Maximum allowed inputs for expressions.

constexpr uint32_t MAX_INPUTS = 64;

namespace DNF 
{
	//	DNF::Pin stores a pin number and if it is inverted.

	struct Pin 
	{
		Pin(bool Inverted, uint32_t PinNumber) 
		{
			this->m_Inverted = Inverted;
			this->m_PinNumber = PinNumber;
		}

		bool m_Inverted;
		uint32_t m_PinNumber;
	};

	//	DNF::Row stores pins which represent one logical "and" row in a DNF.

	struct Row  
	{ 
		Row(vector<Pin> Pins)
		{
			this->m_Pins = Pins;
		}
		
		vector<Pin> m_Pins; 
	};

	//	DNF::Expression stores all "and" rows which are or'd together in the DNF expression.

	struct Expression 
	{
		Expression() 
		{
			this->m_OutputPin = 0;
			this->m_EnableFlipFlop = false;
		}

		Expression(uint32_t OutputPin, bool EnableFlipFlops, vector<Row> Rows)
		{
			this->m_OutputPin = OutputPin;
			this->m_EnableFlipFlop = EnableFlipFlops;
			this->m_Rows = Rows;
		}

		uint32_t m_OutputPin;
		bool m_EnableFlipFlop;
		vector<Row> m_Rows;
	};

	bool Build(TableData& TruthTable, Expression& ExpressionOut, Configs::CircuitConfig* pConfig);
	bool Build(vector<TableData>& TruthTables, vector<Expression>& ExpressionsOut, Configs::CircuitConfig* pConfig);
	Row BuildRow(bitset<MAX_INPUTS> Bits, vector<uint32_t> Inputs);
}