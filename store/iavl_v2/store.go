package iavl_v2

import (
	"fmt"
	"io"
	"path/filepath"
	"time"

	abci "github.com/cometbft/cometbft/abci/types"
	"github.com/cosmos/iavl/v2"
	"github.com/dustin/go-humanize"

	gogotypes "github.com/cosmos/gogoproto/types"

	"github.com/cosmos/cosmos-sdk/store/cachekv"
	"github.com/cosmos/cosmos-sdk/store/listenkv"
	pruningtypes "github.com/cosmos/cosmos-sdk/store/pruning/types"
	"github.com/cosmos/cosmos-sdk/store/tracekv"
	"github.com/cosmos/cosmos-sdk/store/types"
	"github.com/cosmos/cosmos-sdk/telemetry"
)

var (
	_ types.KVStore                 = (*Store)(nil)
	_ types.CommitStore             = (*Store)(nil)
	_ types.CommitKVStore           = (*Store)(nil)
	_ types.Queryable               = (*Store)(nil)
	_ types.StoreWithInitialVersion = (*Store)(nil)
)

type Store struct {
	iavl.Tree
}

func LoadStoreWithInitialVersion(v2RootPath string, metadata *iavl.SqliteKVStore, key types.StoreKey, id types.CommitID, _ uint64) (types.CommitKVStore, error) {
	// TODO
	// handle initialVersion (last param). This parameter is non-zero when the storeKey is flagged as added in upgrades.
	// i.e. not the happy path.
	path := filepath.Join(v2RootPath, key.Name())
	pool := iavl.NewNodePool()
	sqlOpts := iavl.SqliteDbOptions{Path: path}

	mmapKey := []byte(fmt.Sprintf("mmap-%s", key.Name()))
	mmapSize, err := metadata.Get(mmapKey)
	if err != nil {
		return nil, fmt.Errorf("failed to get mmap size for sqlite db path=%s: %w", path, err)
	}
	if mmapSize == nil {
		sqlOpts.MmapSize, err = sqlOpts.EstimateMmapSize()
		if err != nil {
			return nil, fmt.Errorf("failed to estimate mmap size for sqlite db path=%s: %w", path, err)
		}
		bz, err := gogotypes.StdUInt64Marshal(sqlOpts.MmapSize)
		if err != nil {
			return nil, fmt.Errorf("failed to marshal mmap size for sqlite db path=%s: %w", path, err)
		}
		if err = metadata.Set(mmapKey, bz); err != nil {
			return nil, err
		}
	} else {
		var sz uint64
		err = gogotypes.StdUInt64Unmarshal(&sz, mmapSize)
		if err != nil {
			return nil, fmt.Errorf("failed to unmarshal mmap size for sqlite db path=%s: %w", path, err)
		}
		fmt.Printf("mmap size for sqlite path=%s: %s\n", path, humanize.IBytes(uint64(sz)))
		sqlOpts.MmapSize = sz
	}

	sql, err := iavl.NewSqliteDb(pool, sqlOpts)
	if err != nil {
		return nil, fmt.Errorf("failed to open sqlite db path=%s: %w", path, err)
	}

	tree := iavl.NewTree(sql, pool, iavl.TreeOptions{
		StateStorage:       true,
		CheckpointInterval: 10_000,
		EvictionDepth:      16,
		MetricsProxy:       &telemetry.GlobalMetricProxy{},
		HeightFilter:       1,
	})

	err = tree.LoadVersion(id.Version)
	if err != nil {
		return nil, err
	}
	//if err = sql.WarmLeaves(); err != nil {
	//	return nil, err
	//}
	return &Store{*tree}, nil
}

func (s *Store) SetInitialVersion(version int64) {
	//TODO implement me
	panic("implement me")
}

func (s *Store) Query(query abci.RequestQuery) abci.ResponseQuery {
	//TODO implement me
	panic("implement me")
}

func (s *Store) Commit() types.CommitID {
	defer telemetry.MeasureSince(time.Now(), "store", "iavl", "commit")

	hash, version, err := s.Tree.SaveVersion()
	if err != nil {
		panic(err)
	}

	return types.CommitID{
		Version: version,
		Hash:    hash,
	}
}

func (s *Store) LastCommitID() types.CommitID {
	hash := s.Tree.Hash()

	return types.CommitID{
		Version: s.Tree.Version(),
		Hash:    hash,
	}
}

func (s *Store) SetPruning(options pruningtypes.PruningOptions) {
	panic("cannot set pruning options on an initialized IAVL store")
}

func (s *Store) GetPruning() pruningtypes.PruningOptions {
	panic("cannot get pruning options on an initialized IAVL store")
}

func (s *Store) GetStoreType() types.StoreType {
	return types.StoreTypeIAVL
}

func (s *Store) CacheWrap() types.CacheWrap {
	return cachekv.NewStore(s)
}

func (s *Store) CacheWrapWithTrace(w io.Writer, tc types.TraceContext) types.CacheWrap {
	return cachekv.NewStore(tracekv.NewStore(s, w, tc))
}

func (s *Store) CacheWrapWithListeners(storeKey types.StoreKey, listeners []types.WriteListener) types.CacheWrap {
	return cachekv.NewStore(listenkv.NewStore(s, storeKey, listeners))
}

func (s *Store) Get(key []byte) []byte {
	defer telemetry.MeasureSince(time.Now(), "store", "iavl", "get")
	value, err := s.Tree.Get(key)
	if err != nil {
		panic(err)
	}
	return value
}

func (s *Store) Has(key []byte) bool {
	defer telemetry.MeasureSince(time.Now(), "store", "iavl", "has")
	has, err := s.Tree.Has(key)
	if err != nil {
		panic(err)
	}
	return has
}

func (s *Store) Set(key, value []byte) {
	defer telemetry.MeasureSince(time.Now(), "store", "iavl", "set")
	types.AssertValidKey(key)
	types.AssertValidValue(value)
	_, err := s.Tree.Set(key, value)
	if err != nil {
		panic(err)
	}
}

func (s *Store) Delete(key []byte) {
	defer telemetry.MeasureSince(time.Now(), "store", "iavl", "delete")
	s.Tree.Remove(key)
}

func (s *Store) Iterator(start, end []byte) types.Iterator {
	itr, err := s.Tree.Iterator(start, end, false)
	if err != nil {
		panic(err)
	}
	return itr
}

func (s *Store) ReverseIterator(start, end []byte) types.Iterator {
	itr, err := s.Tree.ReverseIterator(start, end)
	if err != nil {
		panic(err)
	}
	return itr
}

func (s *Store) DeleteVersions(versions ...int64) error {
	maxVersion := versions[0]
	for _, v := range versions {
		if v > maxVersion {
			maxVersion = v
		}
	}
	return s.Tree.DeleteVersionsTo(maxVersion)
}

func (s *Store) WorkingBytes() uint64 {
	return s.Tree.WorkingBytes()
}

func (s *Store) SetShouldCheckpoint() {
	s.Tree.SetShouldCheckpoint()
}
