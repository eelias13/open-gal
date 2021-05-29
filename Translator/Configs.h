#pragma once

#ifdef _WIN32

#include <direct.h>
#define GetCurrentDir _getcwd

#elif __linux__

#include <unistd.h>
#define GetCurrentDir getcwd

#endif

#include <stdint.h>
#include <vector>
#include <fstream>
#include <iostream>

#include "Helper.hpp"

//	JSON serializer: https://github.com/nlohmann/json

#include "../Shared/Dependencies/json.hpp"

using namespace std;
using namespace nlohmann;

namespace Configs
{
	/*
	*	The CircuitConfig datastructure stores every important attribute of an programmable logic device.
	*
	*	CircuitConfig::m_Outputs contains pairs of two unsigned integers.
	*	- The first value of a pair contains the designated output pin.
	*	- The second value of a pair contains the maximum output terms for the specified output pin.
	*
	*	The m_SpecialPins attribute is reserved for input pins which don't oblige the standard translation rules. 
	*	It contains a pair of two unsigned integers
	*	- The first value is the input pin number which is special.
	*	- The second value is the row index which should be used for this pin uninverted.
	*/

	struct CircuitConfig 
	{
		uint32_t m_iNumFuses = 0;
		uint32_t m_iNumPins = 0;
		vector<uint32_t> m_Inputs = {};
		vector<pair<uint32_t, uint32_t>> m_Outputs = {};
		vector<pair<uint32_t, uint32_t>> m_SpecialPins = {};
	};

	bool Load(const char* szConfigName, CircuitConfig* pConfigOut);
}
