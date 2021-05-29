#include "Fuses.h"

bool Fuses::Build(vector<DNF::Expression> Expressions, vector<bool>& FuseListOut, Configs::CircuitConfig* pConfig)
{
	if (!Expressions.size())
	{
		ERROR("%s", "No expressions were given");
		return false;			
	}

	//	Get row length for one DNF term.

	uint32_t iRowLength = Fuses::GetRowLength(pConfig);

	if (FuseListOut.size())
		FuseListOut.clear();

	//	Adjust fuselist size to the fuse list size of the integrated circuit.

	FuseListOut.resize(pConfig->m_iNumFuses);
	
	//	Set AR Fuses to zero (we don't need them as of yet)

	std:fill(FuseListOut.begin(), FuseListOut.begin() + iRowLength, false);
	
	//	Start writing expressions to FuseList.

	for(uint32_t Index = 0; Index < Expressions.size(); Index++)
	{
		uint32_t ExpIndexStart = Fuses::Output::GetFirstFuseIndex(Expressions[Index].m_OutputPin, pConfig);

		if(ExpIndexStart == -1)
		{
			ERROR("%s", "Couldn't get fuse index start");
			return false;
		}

		vector<bool> ExpressionBuffer;

		if (!Fuses::BuildFromExpression(Expressions[Index], Fuses::Output::MaximumTerms(Expressions[Index].m_OutputPin, pConfig) + 1, iRowLength, ExpressionBuffer, pConfig)) 
		{
			ERROR("%s", "Couldn't build all expression fuses");
			return false;
		}

		//	Copy ExpressionBuffer into the correct target destination in the fuse matrix.

		std::copy(ExpressionBuffer.begin(), ExpressionBuffer.end(), FuseListOut.begin() + ExpIndexStart);
	}

	//	Set SP fuses to zero because we also don't need them as of yet.

	uint32_t iLastFuseIDX = Fuses::Output::GetLastFuseIndex(pConfig->m_Outputs.front().first, pConfig);
	std::fill(FuseListOut.begin() + iLastFuseIDX, FuseListOut.begin() + iLastFuseIDX + iRowLength, false);

	//	Set S0 & S1 fuses.

	for(DNF::Expression Expression : Expressions)
	{
		pair<uint32_t, uint32_t> ModeFuses;

		if(!Fuses::Output::ModeFuseIndices(Expression.m_OutputPin, ModeFuses, pConfig))
		{
			ERROR("%s", "Invalid PIN");
			return false;
		}

		if(Expression.m_EnableFlipFlop)
		{
			FuseListOut[ModeFuses.first] = 1;
			FuseListOut[ModeFuses.second] = 0;
		}
		else
		{
			FuseListOut[ModeFuses.first] = 1;
			FuseListOut[ModeFuses.second] = 1;
		}
	}

	return true;
}

/*
*		Fuses::BuildFromExpression generates a fuselist for a specific expression and outputs the result in a supplied 
*		fuselist. It needs to know the term size and number of rows to correctly pad the fuselist with zeroes. 
*/

bool Fuses::BuildFromExpression(DNF::Expression Expression, uint32_t iNumRows, uint32_t iRowLength, vector<bool>& FuseList, Configs::CircuitConfig* pConfig)
{
	if(!Fuses::Output::IsValid(Expression.m_OutputPin, pConfig))
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

	for(uint32_t TermIndex = 0; TermIndex < Expression.m_Rows.size(); TermIndex++)
	{
		std::fill(FuseList.begin() + iRowLength + TermIndex * iRowLength, FuseList.begin() + iRowLength + TermIndex * iRowLength + iRowLength, true);

		for(uint32_t PinIndex = 0; PinIndex < Expression.m_Rows[TermIndex].m_Pins.size(); PinIndex++)
		{
			int Index = Fuses::PinToIndex
			(
				Expression.m_Rows[TermIndex].m_Pins[PinIndex].m_PinNumber,
				Expression.m_Rows[TermIndex].m_Pins[PinIndex].m_Inverted,
				Expression.m_EnableFlipFlop ? MacrocellMode::MODE_REGISTERED_HIGH : MacrocellMode::MODE_COMBINATORIAL_HIGH,
				pConfig
			);

			if(Index == -1)
			{
				ERROR("%s", "Couldn't resolve PIN index");
				return false;
			}

			FuseList[iRowLength + TermIndex * iRowLength + Index] = false;
		}
	}

	return true;
}

/*
*		Fuses::PinToIndex converts a PIN to a fuselist row index. It takes in a PIN number and a boolean
*		which indicates if the PIN is supposed to be inverted. It requires an extra parameter if the given  
*		PIN number correlates to an output pin. This parameter is called "Mode" and is an enum. The parameter
*		is needed because the output from the OLMC is inverted or not inverted depending on the mode it is
*		operating in (the mode is set through S0 and S1 fuses) so we need to know the mode to correctly pick
*		the inverted or non inverted output from the OLMC output. 
*		
*		Note: The "Mode" parameter is ignored if the PIN number correlates to an input pin. If the PIN is an 
*		output pin and the parameter is not set the function will return -1 which is not a valid index.
*		
*		Note: If the return value is -1 that means that the function couldn't find a valid index for the given
*		parameters.
*/

int Fuses::PinToIndex(uint32_t iPinNumber, bool bInverted, MacrocellMode Mode, Configs::CircuitConfig* pConfig)
{
	//	Handles special pins.

	for (auto SpecialPin : pConfig->m_SpecialPins)
		if (SpecialPin.first == iPinNumber)
			return bInverted ? SpecialPin.second + 1 : SpecialPin.second;
	
	int FuseIndex;

	//	Handles output pins.

	for(uint32_t Index = 0; Index < pConfig->m_Outputs.size(); Index++)
	{
		if(pConfig->m_Outputs[Index].first == iPinNumber)
		{
			FuseIndex = 2 + (pConfig->m_Outputs.size() - 1 - Index) * 4;
		
			if (Mode == MacrocellMode::MODE_COMBINATORIAL_HIGH)
				return bInverted ? FuseIndex + 1 : FuseIndex;
			else if (Mode != MacrocellMode::MODE_NONE)
				return bInverted ? FuseIndex : FuseIndex + 1;
			else
				return -1;
		}
	}

	//	Handles input pins.

	FuseIndex = (iPinNumber - 1) * 4;
	return bInverted ? FuseIndex + 1 : FuseIndex;
}

//	Fuses::GetRowLength returns the length of one DNF term row.

int Fuses::GetRowLength(Configs::CircuitConfig* pConfig)
{
	return (pConfig->m_Inputs.size() + pConfig->m_SpecialPins.size()) * 2;
}

/*
*		Fuses::Output::MaximumTerms returns the maximum amount of terms an output OLMC can handle.
*		if the function return value is -1 it means that the given pin number is not an valid output pin
*		thus the function can't return a valid term number.
*/

int Fuses::Output::MaximumTerms(uint32_t iPinNumber, Configs::CircuitConfig* pConfig)
{
	for(auto OutputPin : pConfig->m_Outputs)
		if (OutputPin.first == iPinNumber)
			return OutputPin.second;
	
	return -1;
}

//		Fuses::Output::IsValid checks if a given pin is an output pin:

bool Fuses::Output::IsValid(uint32_t iPinNumber, Configs::CircuitConfig* pConfig)
{
	for (auto OutputPin : pConfig->m_Outputs)
		if (OutputPin.first == iPinNumber)
			return true;

	return false;
}

/*
*		Fuses::Output::ModeFuseIndices returns the mode control fuses for a given output pin.
*		The return value is a boolean which indicates if the fuse pair was written to the given 
*		std::pair reference. The function will only return false if the given pin number is
*		an input pin who has no OLMC connected and therefore no control mode pin.
*/

bool Fuses::Output::ModeFuseIndices(uint32_t iPinNumber, pair<uint32_t, uint32_t>& FusesOut, Configs::CircuitConfig* pConfig)
{
	if (!Fuses::Output::IsValid(iPinNumber, pConfig))
		return false;

	//	Get last fuse.

	uint32_t FuseIndex = Fuses::Output::GetLastFuseIndex(pConfig->m_Outputs.front().first, pConfig) + Fuses::GetRowLength(pConfig);

	//	Get OLMC number.

	for(uint32_t Index = 0; Index < pConfig->m_Outputs.size(); Index++)
	{
		if (pConfig->m_Outputs[Index].first == iPinNumber)
		{
			uint32_t FusesStart = FuseIndex + (pConfig->m_Outputs.size() - 1 - Index) * 2;
			FusesOut = pair<uint32_t, uint32_t>(FusesStart, FusesStart + 1);
			break;
		}
	}

	return true;
}

//	Fuses::Output::GetFirstFuseIndex returns the first fuse of an OLMC output.

int Fuses::Output::GetFirstFuseIndex(uint32_t iPinNumber, Configs::CircuitConfig* pConfig)
{
	if(!Output::IsValid(iPinNumber, pConfig))
	{
		ERROR("%s", "Invalid output pin");
		return -1;
	}

	uint32_t iFuseIndex = Fuses::GetRowLength(pConfig);

	for (uint32_t OLMC = pConfig->m_Outputs.back().first; OLMC > iPinNumber; OLMC--)
		iFuseIndex += (Fuses::Output::MaximumTerms(OLMC, pConfig) + 1) * Fuses::GetRowLength(pConfig);

	return iFuseIndex;
}

//	Fuses::Output::GetLastFuseIndex returns the last fuse of an OLMC output.

int Fuses::Output::GetLastFuseIndex(uint32_t iPinNumber, Configs::CircuitConfig* pConfig)
{
	if(!Output::IsValid(iPinNumber, pConfig))
	{
		ERROR("%s", "Invalid output pin");
		return -1;
	}

	return Output::GetFirstFuseIndex(iPinNumber, pConfig) + (Output::MaximumTerms(iPinNumber, pConfig) + 1) * Fuses::GetRowLength(pConfig);
}

