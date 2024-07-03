package cachemulti

import (
	"fmt"
	"testing"

	"cosmossdk.io/core/log"
	"cosmossdk.io/store/iavl"
	"cosmossdk.io/store/types"
	dbm "github.com/cosmos/cosmos-db"
	"github.com/stretchr/testify/require"
)

func setupStore(b *testing.B, storeCount uint) (Store, map[string]types.StoreKey) {
	db := dbm.NewMemDB()

	storeKeys := make(map[string]types.StoreKey)
	stores := make(map[types.StoreKey]types.CacheWrapper)
	for i := uint(0); i < storeCount; i++ {
		key := types.NewKVStoreKey(fmt.Sprintf("store%d", i))
		storeKeys[key.Name()] = key
		sdb := dbm.NewPrefixDB(db, []byte(key.Name()))
		istore, err := iavl.LoadStore(sdb, log.NewNopLogger(), key, types.CommitID{}, 1000, false, nil)
		require.NoError(b, err)
		stores[key] = types.KVStore(istore)
	}

	return NewStore(db, stores, storeKeys, nil, types.TraceContext{}), storeKeys
}

func benchmarkStore(b *testing.B, storeCount, keyCount uint) {
	store, storeKeys := setupStore(b, storeCount)
	b.ResetTimer()

	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		b.StopTimer()
		for _, key := range storeKeys {
			cstore := store.GetKVStore(key)
			for j := uint(0); j < keyCount; j++ {
				dataKey := fmt.Sprintf("key%s-%d", key.Name(), j)
				dataValue := fmt.Sprintf("value%s-%d", key.Name(), j)
				cstore.Set([]byte(dataKey), []byte(dataValue))
			}
		}
		b.StartTimer()
		store.Write()
	}
}

func BenchmarkCacheMultiStore(b *testing.B) {
	storeCounts := []uint{2, 4, 8, 16, 32}
	keyCounts := []uint{100, 1000, 10000}

	for _, storeCount := range storeCounts {
		for _, keyCount := range keyCounts {
			b.Run(fmt.Sprintf("storeCount=%d/keyCount=%d", storeCount, keyCount), func(sub *testing.B) {
				benchmarkStore(sub, storeCount, keyCount)
			})
		}
	}

}
