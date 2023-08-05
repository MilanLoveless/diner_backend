#!/bin/bash
openssl req -x509 -newkey rsa:4096 -nodes -keyout secrets/ssl/key.pem -out secrets/ssl/cert.pem -days 365 -subj '/CN=localhost'
