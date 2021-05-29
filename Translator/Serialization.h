#pragma once

#include "Helper.hpp"

#include <vector>
#include <string>
#include <fstream>
#include <iomanip>
#include <sstream>
#include <bitset>

//	Version string for JEDEC file

constexpr char EASYGAL_VERSION[] = "EasyGal-Translator v1.0";

//	Needed JEDEC field identifiers ("http://www.pldtool.com/pdf/fmt_jedec.pdf").

constexpr char ID_NOTE = 'N';
constexpr char ID_CHECKSUM = 'C';
constexpr char ID_DEFAULT_FUSESTATE_FIELD = 'F';
constexpr char ID_FUSELIST_BINARY = 'L';
constexpr char ID_FUSELIST_HEX = 'K';
constexpr char ID_VALUE = 'Q';
constexpr char ID_PIN = 'P';
constexpr char ID_DEVICETYPE = 'D';
constexpr char ID_TERMINATOR = '*';

//	Start of text and end of text control characters which are used in JEDEC files.

constexpr char ASCII_CTRL_STX = '\x2';
constexpr char ASCII_CTRL_ETX = '\x3';

//	Output blocksize for fuselist.

constexpr uint32_t FUSE_BLOCKSIZE = 32;

//	The JEDEC class is used to serialize the results from translator to disk.

class JEDEC
{

public:

	JEDEC(uint32_t iNumPins, uint32_t iNumFuses, std::vector<bool>& FuseStates, std::string Filename) : m_iNumPins(iNumPins), m_iNumFuses(iNumFuses), m_FuseStates(FuseStates), m_Filename(Filename) 
	{
		if(!m_iNumPins || !m_iNumFuses || !FuseStates.size())
		{
			ERROR("%s", "JEDEC object was initialized with invalid parameters");
			getchar();
			exit(1);
		}
	}

	~JEDEC()
	{
	}

	void Serialize();

private:

	bool BlockContainsData(uint32_t StartIndex);

private:
	
	
	string m_FileBuffer;
	string m_Filename;
	
	uint32_t m_iNumPins = 0;
	uint32_t m_iNumFuses = 0;
	
	vector<bool> m_FuseStates;
};

