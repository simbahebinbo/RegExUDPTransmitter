#!/bin/bash

PORT=12345

socat -u UDP-RECVFROM:$PORT,fork EXEC:/bin/cat
