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

void compile(string easyGALCode, string outputFileName, string deviceName)
{
	Parser parser = Parser(easyGALCode);
	vector<TableData> tableData = parser.parse();

	Configs::CircuitConfig DeviceType;
	vector<uint32_t> inputPins;
	vector<uint32_t> outputPins;
	initDeviceType(DeviceType, deviceName, inputPins, outputPins);
	validate(tableData, inputPins, outputPins);

	Translator::Process(tableData, DeviceType, outputFileName);

	cout << "compilation successfully, new jedec file was created " << outputFileName << endl;
}

int main(int argc, char *argv[])
{

	if (argc == 1)
	{
		cerr << "invalid argument count" << endl;
		showHelpMenu();
		exit(1);
	}

	if (strcmp(argv[1], "help") == 0)
	{
		showHelpMenu();
		exit(0);
	}
	else if (strcmp(argv[1], "api") == 0)
	{
		if (argc == 2)
		{
			cerr << "invalid argument count" << endl;
			showHelpMenu();
			exit(1);
		}

		string fileEnding = getFileEnding(argv[2]);
		if (fileEnding == "json")
		{
			checkFileEnding(argv[3], "jedec");
			if (argc != 5)
			{
				cerr << "invalid argument count" << endl;
				showHelpMenu();
				exit(1);
			}
			api::tableData2jedec(argv[2], argv[3], argv[4]);
		}
		else if (fileEnding == "txt")
		{
			checkFileEnding(argv[3], "json");
			if (argc != 4 && argc != 5)
			{
				cerr << "invalid argument count" << endl;
				showHelpMenu();
				exit(1);
			}
			api::code2TableData(argv[2], argv[3], argc == 5 ? argv[4] : "");
		}
		else
		{
			cerr << "invalid file extention " + string(argv[2]) << endl;
			showHelpMenu();
			exit(1);
		}
	}
	else
	{
		if (argc == 4)
		{
			checkFileEnding(argv[1], "txt");
			checkFileEnding(argv[2], "jedec");
			compile(argv[1], argv[2], argv[3]);
		}
		else
		{
			cerr << "invalid argument count" << endl;
			showHelpMenu();
			exit(1);
		}
	}
}