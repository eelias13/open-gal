#pragma once

//  Author: Lukas Götz

#include <vector>
#include <stdint.h>

using namespace std;

/*
*   This data structure contains following data from processed expressions.
*   
*   - "m_InputPins" stores all the input pins which are used in the expression
*   - "m_OutputPin" stores the output pin
*   - "m_Table" contains the truth table for the expression and is used to generate a dnf expression later on
*   - "m_EnableDFlipFlop" holds a boolean which decides if the output pin should have its flip flop turned on.
*/

struct TableData
{
    vector<uint32_t> m_InputPins;
    uint32_t m_OutputPin;
    vector<bool> m_Table;
    bool m_EnableFlipFlop;
};