# Origin
tendermint = https://raw.githubusercontent.com/tendermint/tendermint/master

# Outputs
tmkv = protobuf/github.com/tendermint/tendermint/libs/common/types.proto
tmmerkle = protobuf/github.com/tendermint/tendermint/crypto/merkle/merkle.proto
tmabci = protobuf/github.com/tendermint/tendermint/abci/types/types.proto

# You *only* need to run this to rebuild protobufs
update-proto:
	curl $(tendermint)/abci/types/types.proto > $(tmabci)
	curl $(tendermint)/libs/common/types.proto > $(tmkv)
	curl $(tendermint)/crypto/merkle/merkle.proto > $(tmmerkle)
	sed 's@package types;@package abci;@' $(tmabci) > protobuf/abci.proto
	curl $(tendermint)/version/version.go | grep -F -eTMCoreSem -eABCISemVer > version.txt
	
