#include "Helper.hpp"

vector<string> Helper::String::Tokenize(const string &InputString, string Delimiters)
{
    /*
	*	String::Tokenize() only returns one Token in case: 
	*
	*		- String only contains delimiters
	*		- String contains no delimiters
	*		- String is empty
	*/

    if (String::Find(InputString, Delimiters) == -1 || String::FindNot(InputString, Delimiters) == -1)
        return vector<string>{InputString};

    vector<string> Tokens;

    int iBegin = String::FindNot(InputString, Delimiters);

    for (unsigned int i = iBegin; i < InputString.size() + 1; i++)
    {
        int iEnd = String::Find(InputString, Delimiters, i);

        if (iEnd == -1)
        {
            Tokens.push_back(string(InputString.begin() + iBegin, InputString.end()));
            break;
        }

        Tokens.push_back(string(InputString.begin() + iBegin, InputString.begin() + iEnd));
        iBegin = String::FindNot(InputString, Delimiters, iEnd);

        if (iBegin == -1)
            break;

        i = iBegin;
    }

    return Tokens;
}

/*
*		String::TokenizeEx splits strings into tokens based on delimiters. This extended version of String::Tokenize 
*		also returns the delimiters which are found in the string as tokens.
*/

vector<string> Helper::String::TokenizeEx(const string &InputString, string Delimiters)
{
    if (String::Find(InputString, Delimiters) == -1 || String::FindNot(InputString, Delimiters) == -1)
        return vector<string>{InputString};

    vector<string> Tokens;

    int iBegin = String::FindNot(InputString, Delimiters);

    for (unsigned int i = iBegin; i < InputString.size() + 1; i++)
    {
        int iEnd = String::Find(InputString, Delimiters, i);

        if (iEnd == -1)
        {
            Tokens.push_back(string(InputString.begin() + iBegin, InputString.end()));
            break;
        }

        Tokens.push_back(string(InputString.begin() + iBegin, InputString.begin() + iEnd));
        iBegin = String::FindNot(InputString, Delimiters, iEnd);

        if (iBegin == -1)
        {
            // If no new non delimiter characters are found, the remaining characters are added as tokens.

            for (unsigned int j = i + 1; j < InputString.size(); j++)
                Tokens.push_back(string(1, InputString[j]));
            break;
        }

        for (int i = 0; i < iBegin - iEnd; i++)
            Tokens.push_back(string(1, InputString[iEnd + i]));

        i = iBegin;
    }

    return Tokens;
}

//		String::Find() returns the index of the first occurence of an character in a specified string.

int Helper::String::Find(const string &String, char Character, unsigned int StartIndex)
{
    for (unsigned int Index = StartIndex; Index < String.size(); Index++)
        if (String[Index] == Character)
            return Index;

    return -1;
}

/*
*		This overload of String::Find returns the index of the first occurence of specified characters in specified string.
*		Which character was found in string is not returned.
*/

int Helper::String::Find(const string &String, const string &Characters, unsigned int StartIndex)
{
    for (unsigned int Index = StartIndex; Index < String.size(); Index++)
        for (char Character : Characters)
            if (String[Index] == Character)
                return Index;

    return -1;
}

/*
*		String::FindNot returns the index of the first character that is not any of the specified characters.
*/

int Helper::String::FindNot(const string &String, const string &Characters, unsigned int StartIndex)
{
    for (unsigned int Index = StartIndex; Index < String.size(); Index++)
    {
        bool Valid = true;

        for (char Character : Characters)
            if (String[Index] == Character)
                Valid = false;

        if (Valid)
            return Index;
    }

    // Returns -1 if every character is a non allowed character

    return -1;
}

bool Helper::String::IsNumber(const string &String)
{
    if (String.empty())
        return false;

    for (char Character : String)
        if (Character < '0' || Character > '9')
            return false;

    return true;
}
