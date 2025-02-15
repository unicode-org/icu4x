#ifndef IanaParser_H
#define IanaParser_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"

#include "IanaParser.d.h"






IanaParser* icu4x_IanaParser_create_mv1(void);

typedef struct icu4x_IanaParser_create_with_provider_mv1_result {union {IanaParser* ok; DataError err;}; bool is_ok;} icu4x_IanaParser_create_with_provider_mv1_result;
icu4x_IanaParser_create_with_provider_mv1_result icu4x_IanaParser_create_with_provider_mv1(const DataProvider* provider);

void icu4x_IanaParser_iana_to_bcp47_mv1(const IanaParser* self, DiplomatStringView value, DiplomatWrite* write);

typedef struct icu4x_IanaParser_normalize_iana_mv1_result { bool is_ok;} icu4x_IanaParser_normalize_iana_mv1_result;
icu4x_IanaParser_normalize_iana_mv1_result icu4x_IanaParser_normalize_iana_mv1(const IanaParser* self, DiplomatStringView value, DiplomatWrite* write);

typedef struct icu4x_IanaParser_canonicalize_iana_mv1_result { bool is_ok;} icu4x_IanaParser_canonicalize_iana_mv1_result;
icu4x_IanaParser_canonicalize_iana_mv1_result icu4x_IanaParser_canonicalize_iana_mv1(const IanaParser* self, DiplomatStringView value, DiplomatWrite* write);

typedef struct icu4x_IanaParser_find_canonical_iana_from_bcp47_mv1_result { bool is_ok;} icu4x_IanaParser_find_canonical_iana_from_bcp47_mv1_result;
icu4x_IanaParser_find_canonical_iana_from_bcp47_mv1_result icu4x_IanaParser_find_canonical_iana_from_bcp47_mv1(const IanaParser* self, DiplomatStringView value, DiplomatWrite* write);


void icu4x_IanaParser_destroy_mv1(IanaParser* self);





#endif // IanaParser_H
