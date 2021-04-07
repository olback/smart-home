#!/bin/sh

scp $1 root@192.168.0.126:/root/
ssh root@192.168.0.126 /root/$(basename $1)
