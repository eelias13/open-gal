#pragma once

#include "DNF.hpp"

//	All OLMC operation modes for the GAL22V10D (http://web.mit.edu/6.115/www/document/gal22v10.pdf)
//	Only MODE_NONE doesn't exist in the documentation, this is because we only use this internally.

enum class MacrocellMode
{
	MODE_NONE,					
	MODE_REGISTERED_LOW,
	MODE_REGISTERED_HIGH,
	MODE_COMBINATORIAL_LOW,
	MODE_COMBINATORIAL_HIGH
};

//	The fuses namespace is used to generate fuse lists from DNF::Expression objects.

namespace Fuses
{
	bool Build(vector<DNF::Expression> Expressions, vector<bool>& FuseListOut, Configs::CircuitConfig* pConfig);
	bool BuildFromExpression(DNF::Expression Expression, uint32_t iNumRows, uint32_t iRowLength, vector<bool>& FuseListOut, Configs::CircuitConfig* pConfig);

	int PinToIndex(uint32_t iPinNumber, bool bInverted, MacrocellMode Mode, Configs::CircuitConfig* pConfig);
	int GetRowLength(Configs::CircuitConfig* pConfig);

	namespace Output
	{
		int MaximumTerms(uint32_t iPinNumber, Configs::CircuitConfig* pConfig);
		bool IsValid(uint32_t iPinNumber, Configs::CircuitConfig* pConfig);
		bool ModeFuseIndices(uint32_t iPinNumber, pair<uint32_t, uint32_t>& FusesOut, Configs::CircuitConfig* pConfig);
		
		int GetFirstFuseIndex(uint32_t iPinNumber, Configs::CircuitConfig* pConfig);
		int GetLastFuseIndex(uint32_t iPinNumber, Configs::CircuitConfig* pConfig);
	}
}
