#!/bin/sh

scp $1 root@192.168.1.101:/root/
ssh root@192.168.1.101 /root/$(basename $1)
