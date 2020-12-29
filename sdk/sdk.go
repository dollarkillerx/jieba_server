package sdk

import (
	"context"

	"github.com/dollarkillerx/jieba_server/sdk/proto"
	"google.golang.org/grpc"
)

type JiebaServer struct {
	client proto.SingleJiebaClient
}

func New(addr string) (*JiebaServer, error) {
	dial, err := grpc.Dial(addr, grpc.WithInsecure())
	if err != nil {
		return nil, err
	}
	client := proto.NewSingleJiebaClient(dial)
	return &JiebaServer{
		client: client,
	}, nil
}

func (j *JiebaServer) CutAll(str string) ([]string, error) {
	cut, err := j.client.Cut(context.TODO(), &proto.CutReq{
		Message: str,
	})
	if err != nil {
		return nil, err
	}

	return cut.Word, nil
}
