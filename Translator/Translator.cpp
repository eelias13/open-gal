#include "Translator.hpp"

/*
*		Translator::Process is the 'main' of the translator. The data processing happens in here in the following order:
*		TableData -> DNF::Expression -> Fuses -> JEDEC -> File on Disk. The boolean return value indicates if the translation
*		was a success and if the JEDEC file was written to the harddrive.
*/

bool Translator::Process(vector<TableData> TruthTables, Configs::CircuitConfig DeviceType, std::string Filename)
{
	Configs::CircuitConfig Config = DeviceType;

	vector<DNF::Expression> Expressions;

	if (!DNF::Build(TruthTables, Expressions, &Config))
	{
		ERROR("%s", "couldn't build all DNF expressions");
		return false;
	}

	std::vector<bool> Fuses;

	if (!Fuses::Build(Expressions, Fuses, &Config))
	{
		ERROR("%s", "couldn't generate all fuses for given expressions");
		return false;
	}

	JEDEC(Config.m_iNumPins, Config.m_iNumFuses, Fuses, Filename).Serialize();

	return true;
}
