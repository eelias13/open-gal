#include "Configs.h"

bool Configs::Load(const char *szConfigName, CircuitConfig *pConfigOut)
{
	if (!pConfigOut)
	{
		ERROR("%s", "pConfigOut was nullptr");
		exit(1);
	}

	char szCurPath[FILENAME_MAX];

	if (GetCurrentDir(szCurPath, FILENAME_MAX) == 0)
	{
		ERROR("%s", "Error getting current working directory");
		exit(1);
	}

	string CurPath(szCurPath);

#ifdef _WIN32

	CurPath += R"(\Configs\)";
	CurPath += szConfigName;
	CurPath += ".json";

#elif __linux__

	CurPath += "/Configs/";
	CurPath += szConfigName;
	CurPath += ".json";

#endif

	ifstream ConfigStream(CurPath);

	if (ConfigStream.good() == false)
	{
		ERROR("%s", "Provided config doesn't exist");
		return false;
	}

	json CurConfig = json::parse(ConfigStream);

	//	Get number of fuses.

	if (CurConfig.find("NumFuses") != CurConfig.end())
	{
		pConfigOut->m_iNumFuses = CurConfig["NumFuses"];
	}
	else
	{
		ERROR("%s", "NumFuses Key doesn't exist in Config");
		return false;
	}

	//	Get total number of pins.

	if (CurConfig.find("TotalNumPins") != CurConfig.end())
	{
		pConfigOut->m_iNumPins = CurConfig["TotalNumPins"];
	}
	else
	{
		ERROR("%s", "TotalNumPins Key doesn't exist in Config");
		return false;
	}

	//	Get input pins.

	if (CurConfig.find("InputPins") != CurConfig.end())
	{
		for (uint32_t Index = 0; Index < CurConfig["InputPins"].size(); Index++)
			pConfigOut->m_Inputs.push_back(CurConfig["InputPins"].at(Index));
	}
	else
	{
		ERROR("%s", "InputPins Key doesn't exist in Config");
		return false;
	}

	//	Get output pins and their maximum DNF term count

	if (CurConfig.find("OutputPins") != CurConfig.end())
	{
		for (uint32_t Index = 0; Index < CurConfig["OutputPins"].size(); Index++)
			pConfigOut->m_Outputs.push_back(CurConfig["OutputPins"].at(Index));
	}
	else
	{
		ERROR("%s", "OutputPins Key doesn't exist in Config");
		return false;
	}

	//	Get special pins

	if (CurConfig.find("SpecialPins") != CurConfig.end())
	{
		for (uint32_t Index = 0; Index < CurConfig["SpecialPins"].size(); Index++)
			pConfigOut->m_SpecialPins.push_back(CurConfig["SpecialPins"].at(Index));
	}
	else
	{
		ERROR("%s", "SpecialPins Key doesn't exist in Config");
		return false;
	}

	return true;
}
