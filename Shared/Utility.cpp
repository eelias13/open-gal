/*
 * Utility.cpp
 *
 *  Created on: Aug 21, 2020
 *      Author: elias
 */

#include "Utility.h"

void showHelpMenu()
{
    cout << "usage of EasyGAL: " << endl;
    cout << '\t' << "EasyGAL [EasyGAl code] [name of output jedec] [gal type]" << endl;
    cout << '\t' << "EasyGAL api [table data json] [name of output jedec] [gal type]" << endl;
    cout << '\t' << "EasyGAL api [EasyGAl code] [table data json] [gal type]" << endl;
    cout << '\t' << "EasyGAL api [EasyGAl code] [table data json]" << endl;
    cout << "Exampel:" << endl;
    cout << '\t' << "EasyGAL code.txt out.jedec g22v10" << endl;
    cout << '\t' << "EasyGAL api tableData.json out.jedec g22v10" << endl;
    cout << '\t' << "EasyGAL api code.txt tableData.json g22v10" << endl;
    cout << '\t' << "EasyGAL api code.txt tableData.json" << endl;
}

void initDeviceType(Configs::CircuitConfig &DeviceType, string deviceName, vector<uint32_t> &inputPins, vector<uint32_t> &outputPins)
{
    if (!Configs::Load(deviceName.c_str(), &DeviceType))
    {
        cerr << "Couldn't get config for device" << deviceName << endl;
        exit(1);
    }

    for (uint32_t i = 0; i < DeviceType.m_Inputs.size(); i++)
        inputPins.push_back(DeviceType.m_Inputs.at(i));

    for (uint32_t i = 0; i < DeviceType.m_SpecialPins.size(); i++)
        inputPins.push_back(DeviceType.m_SpecialPins.at(i).first);

    for (uint32_t i = 0; i < DeviceType.m_Outputs.size(); i++)
        outputPins.push_back(DeviceType.m_Outputs.at(i).first);
}

string getFileEnding(string fileName)
{
    string fileEnding;
    bool flag = true;
    for (uint8_t i = 0; i < fileName.size(); i++)
    {
        if (flag)
        {
            if (fileName.at(i) == '.')
                flag = false;
        }
        else
        {
            fileEnding.push_back(fileName.at(i));
        }
    }
    return fileEnding;
}

void checkFileEnding(string fileName, string fileEnding)
{
    if (getFileEnding(fileName) == fileEnding)
        return;

    cerr << "invalid file extention " + fileName << endl;
    showHelpMenu();
    exit(1);
}