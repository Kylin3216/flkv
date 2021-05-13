#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * keep db pointer
 */
typedef struct FlKv FlKv;

/**
 * keep writeBatch pointer
 */
typedef struct FlKvBatch FlKvBatch;

/**
 * Array struct
 */
typedef struct KvBuffer {
  const unsigned char *data;
  uintptr_t length;
} KvBuffer;

struct FlKv *db_open(const char *name, bool memory);

bool db_put(struct FlKv *flkv, struct KvBuffer *key, struct KvBuffer *value);

struct FlKvBatch *db_create_batch(void);

bool batch_add_kv(struct FlKvBatch *batch, struct KvBuffer *key, struct KvBuffer *value);

bool batch_clear(struct FlKvBatch *batch);

bool db_put_batch(struct FlKv *flkv, struct FlKvBatch *batch, bool sync);

struct KvBuffer *db_get(struct FlKv *flkv, struct KvBuffer *key);

bool db_delete(struct FlKv *flkv, struct KvBuffer *key);

bool db_flush(struct FlKv *flkv);

void db_close(struct FlKv *flkv);
