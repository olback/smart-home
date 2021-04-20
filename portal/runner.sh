#!/bin/sh

set -e

IP='192.168.1.155'
BASE_PATH='/root/'
SSH_USER='root'

scp $1 $SSH_USER@$IP:$BASE_PATH/
ssh $SSH_USER@$IP $BASE_PATH/$(basename $1)

