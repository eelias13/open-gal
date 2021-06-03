#include "ConvertRust.h"

char *convertString(string str)
{
    char *chars = (char *)malloc(sizeof(char) * str.size());
    for (size_t i = 0; i < str.size(); i++)
        *(chars + i) = str.at(i);
    return chars;
}

vector<uint32_t> convertU32Vec(TransferU32Vec tVec)
{
    vector<uint32_t> vec;
    for (size_t i = 0; i < tVec.len; i++)
        vec.push_back(*(tVec.arr + i));
    return vec;
}

vector<bool> convertBoolVec(TransferBoolVec tVec)
{
    vector<bool> vec;
    for (size_t i = 0; i < tVec.len; i++)
        vec.push_back(*(tVec.arr + i));
    return vec;
}

TableData convertTableData(TransferTableData tTableData)
{
    TableData td;

    td.m_InputPins = convertU32Vec(tTableData.input_pins);
    td.m_Table = convertBoolVec(tTableData.table);
    td.m_OutputPin = tTableData.output_pin;
    td.m_EnableFlipFlop = tTableData.enable_flip_flop;

    return td;
}

vector<TableData> convertTableDataArr(TransferTableDataArr tVec)
{
    vector<TableData> vec;
    for (size_t i = 0; i < tVec.len; i++)
    {
        TransferTableData tTableData = *(tVec.arr + i);
        vec.push_back(convertTableData(tTableData));
    }
    return vec;
}

vector<TableData> parseAndConvert(string file)
{
    return convertTableDataArr(parse_file(convertString(file)));
}