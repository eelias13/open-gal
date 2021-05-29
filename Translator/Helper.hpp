#pragma once

#include <vector>
#include <cstring>
#include <string>

using namespace std;

#define ERROR(FMT, ...) \
	printf("%s%s%s%s%s%i%s", "Error in ", __FILE__, "\nIn function ", __FUNCTION__, "\nIn line ", __LINE__, ": "); \
	printf(FMT, __VA_ARGS__); \
	printf("%s", "\n")

#define LOG(FMT, ...) \
	printf("%s", "[LOG] "); \
	printf(FMT, __VA_ARGS__); \
	printf("%s", "\n")

namespace Helper
{
	namespace String
	{
		vector<string> Tokenize(const string &InputString, string Delimiters);
		vector<string> TokenizeEx(const string &InputString, string Delimiters);

		int Find(const string &String, char Character, unsigned int StartIndex = 0);
		int Find(const string &String, const string &Characters, unsigned int StartIndex = 0);
		int FindNot(const string &String, const string &Characters, unsigned int StartIndex = 0);

		bool IsNumber(const string &String);
	}
} 