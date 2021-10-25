/*
 * API.cpp
 *
 *  Created on: Aug 21, 2020
 *      Author: elias
 */

#include "API.h"

void api::code2TableData(string easyGALCode, string outputFileName, string deviceName)
{
    Parser parser = Parser(easyGALCode);
    vector<TableData> tableData = parser.parse();

    if (!deviceName.empty())
    {
        Configs::CircuitConfig DeviceType;
        vector<uint32_t> inputPins;
        vector<uint32_t> outputPins;
        initDeviceType(DeviceType, deviceName, inputPins, outputPins);
        validate(tableData, inputPins, outputPins);
    }

    json out = json::array();

    for (TableData td : tableData)
    {
        json temp = json::object();

        temp["inputPins"] = td.m_InputPins;
        temp["outputPin"] = td.m_OutputPin;
        temp["table"] = td.m_Table;
        temp["dff"] = td.m_EnableFlipFlop;

        out.push_back(temp);
    }

    json result = json::object();
    result["TableData"] = out;

    std::ofstream o(outputFileName);
    o << std::setw(4) << result << std::endl;

    cout << "compilation successfully, new json file was created " << outputFileName << endl;
}

TableData api::parseTableData(json tdJson)
{
    // auto glambda = [](auto a, auto &&b) { return a < b; };
    auto check = [](string expected, json tdJson)
    {
        if (tdJson.find(expected) == tdJson.end())
        {
            cerr << "missing property " << expected << " in json object " << tdJson << endl;
            exit(1);
        }
    };

    TableData tableData;

    tdJson.at("dff").get_to(tableData.m_EnableFlipFlop);
    tdJson.at("outputPin").get_to(tableData.m_OutputPin);
    for (uint32_t pin : tdJson["inputPins"])
        tableData.m_InputPins.push_back(pin);
    for (bool b : tdJson["table"])
        tableData.m_Table.push_back(b);

    return tableData;
}

vector<TableData> api::parseTableDataArray(vector<json> array)
{
    vector<TableData> result;
    for (json j : array)
        result.push_back(parseTableData(j));

    return result;
}

vector<TableData> api::readTableData(string tableDataJson)
{

    ifstream file(tableDataJson);
    if (file.good() == false)
    {
        cerr << "file " << tableDataJson << " doesn't exist" << endl;
        exit(1);
    }
    json jsonFile = json::parse(file);

    if (jsonFile.find("TableData") == jsonFile.end())
    {
        cerr << "missing property TableData in json file " << tableDataJson << endl;
        exit(1);
    }

    vector<TableData> tableData = parseTableDataArray(jsonFile["TableData"]);

    if (tableData.empty())
    {
        cerr << "on TableData found in json file " << tableDataJson << endl;
        exit(1);
    }

    return tableData;
}

void api::tableData2jedec(string tableDataJson, string outputFileName, string deviceName)
{
    ifstream file(tableDataJson);
    if (file.good() == false)
    {
        cerr << "file " << outputFileName << " doesn't exist" << endl;
        exit(1);
    }
    json jsonFile = json::parse(file);

    if (jsonFile.find("TableData") == jsonFile.end())
    {
        cerr << "missing property TableData in json file " << outputFileName << endl;
        exit(1);
    }

    vector<TableData> tableData = parseTableDataArray(jsonFile["TableData"]);

    if (tableData.empty())
    {
        cerr << "on TableData found in json file " << outputFileName << endl;
        exit(1);
    }

    Configs::CircuitConfig DeviceType;
    vector<uint32_t> inputPins;
    vector<uint32_t> outputPins;
    initDeviceType(DeviceType, deviceName, inputPins, outputPins);
    validate(tableData, inputPins, outputPins);

    Translator::Process(tableData, DeviceType, outputFileName);

    cout << "compilation successfully, new jedec file was created " << outputFileName << endl;
}