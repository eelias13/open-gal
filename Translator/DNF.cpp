#include "DNF.hpp"

using namespace DNF;

/*
*		DNF::Build(TableData&, Expression&) builds a DNF expression from a TableData datastructure.
*		It's return value is optional, because the function can fail in case the supplied
*		TableData structure is faulty. The resulting Expression datastructure is stored in the supplied Expression reference.
*/

void DNF::printNewExpression(Expression expression)
{
	printf("Expression { out_pin: %d, enable_flip_flop: %s, rows: vec![", expression.m_OutputPin, expression.m_EnableFlipFlop ? "true" : "false");
	for (Row row : expression.m_Rows)
	{
		printNewRow(row);
		printf(", ");
	}
	printf("]}\n");
}

void DNF::printExpression(Expression expression)
{
	printf("Expression { rows: ");
	for (Row row : expression.m_Rows)
	{
		printRow(row);
		printf(", ");
	}
	printf(", out_pin: %d, enable_flip_flop: %s}\n", expression.m_OutputPin, expression.m_EnableFlipFlop ? "true" : "false");
}

void DNF::printNewRow(Row row)
{
	printf("Row { pins: vec![");
	for (Pin pin : row.m_Pins)
	{
		printNewPin(pin);
		printf(", ");
	}
	printf("]}\n");
}

void DNF::printRow(Row row)
{
	printf("Row { pins: [");
	for (Pin pin : row.m_Pins)
		printPin(pin);
	printf("]}\n");
}

void DNF::printNewPin(Pin pin)
{
	printf("Pin::new(%s, %d)", pin.m_Inverted ? "true" : "false", pin.m_PinNumber);
}

void DNF::printPin(Pin pin)
{
	printf("Pin { inverted: %s, pin_num: %d }\n", pin.m_Inverted ? "true" : "false", pin.m_PinNumber);
}

bool DNF::Build(TableData &TruthTable, Expression &ExpressionOut, Configs::CircuitConfig *pConfig)
{
	if (TruthTable.m_InputPins.size() > pConfig->m_Inputs.size())
	{
		ERROR("%s", "Too many input pins");
		return false;
	}
	else if (TruthTable.m_Table.size() != pow(2, TruthTable.m_InputPins.size()))
	{
		ERROR("%s", "Truth table size doesn't match input bits");
		return false;
	}

	vector<Row> Rows;

	for (uint32_t Index = 0; Index < TruthTable.m_Table.size(); Index++)
	{
		if (TruthTable.m_Table[Index])

		{
			Rows.push_back(BuildRow(bitset<MAX_INPUTS>(Index), TruthTable.m_InputPins));
		}
	}

	ExpressionOut = Expression(TruthTable.m_OutputPin, TruthTable.m_EnableFlipFlop, Rows);

	return true;
}

/*
*		DNF::Build(vector<TableData>&, vector<Expression>&) is basically a wrapper for DNF::Build(TableData&, Expression&)
*		and is able to build multiple DNF expressions at once by receiving a vector of TableData datastructures.
*		It stores the result in a vector which you need to supply to the function by reference. The function
*		returns a boolean which indicates if the building of all expressions was successful.
*/

bool DNF::Build(vector<TableData> &TruthTables, vector<Expression> &ExpressionsOut, Configs::CircuitConfig *pConfig)
{
	if (!TruthTables.size())
	{
		ERROR("%s", "No truth tables were supplied");
		return false;
	}

	vector<Expression> Expressions;

	for (TableData TruthTable : TruthTables)
	{
		Expression CurExpression;

		if (Build(TruthTable, CurExpression, pConfig) == false)
		{
			ERROR("%s", "Couldn't build all DNF expressions");
			return false;
		}

		Expressions.push_back(CurExpression);
	}

	ExpressionsOut = Expressions;

	return true;
}

/*
*		DNF::BuildRow builds rows of chained "and" statements which always turn out true based off the input bits.
*		You have to supply the input pin numbers, so the function can "name" the variables used in the row.
*		The result is returned in the DNF::Row datastructure.
*/


Row DNF::BuildRow(bitset<MAX_INPUTS> Bits, vector<uint32_t> Inputs)
{
	vector<Pin> Pins;
	std::reverse(Inputs.begin(), Inputs.end());
	for (uint32_t Index = 0; Index < Inputs.size(); Index++)
	{
		if (Bits[Inputs.size() - 1 - Index] == false)
			Pins.push_back(Pin(true, Inputs[Index]));
		else
			Pins.push_back(Pin(false, Inputs[Index]));
	}

	return Row(Pins);
}
