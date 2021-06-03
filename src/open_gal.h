#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

static const uint32_t AND = '&';

static const uint32_t OR = '|';

static const uint32_t XOR = '?';

static const uint32_t NOT = '!';

static const bool NUM_FIRST = true;

static const bool COUNT_VERTICAL = false;

struct TransferU32Vec {
  uint32_t *arr;
  uintptr_t len;
};

struct TransferBoolVec {
  bool *arr;
  uintptr_t len;
};

struct TransferTableData {
  TransferU32Vec input_pins;
  uint32_t output_pin;
  TransferBoolVec table;
  bool enable_flip_flop;
};

struct TransferTableDataArr {
  TransferTableData *arr;
  uintptr_t len;
};

extern "C" {

TransferTableDataArr parse_file(char *input);

} // extern "C"
